use std::path::{ Path, PathBuf };
use std::process::{ Command, Stdio };
use proc_macro2::{ TokenStream, TokenTree };

/// A single sqlx query-macro call site found in source.
#[ derive( Debug, Clone ) ]
pub struct QuerySite
{
  /// File the call site was found in, relative to the crate root.
  pub file : PathBuf,
  /// 1-based line number of the macro name.
  pub line : usize,
  /// The SQL text sqlx hashes for this site: the literal (or `+`-joined
  /// literals) of a `query*!` macro, the file contents of a `query_file*!` one.
  pub sql : String,
  /// The SQL file a `query_file*!` site reads.
  pub sql_file : Option< PathBuf >,
}

/// What one walk of a package directory found.
#[ derive( Debug, Default ) ]
pub struct PackageScan
{
  /// Every query-macro call site whose SQL could be determined.
  pub sites : Vec< QuerySite >,
  /// Paths whose changes must re-run the build script (see [`scan_package`]).
  pub watched : Vec< PathBuf >,
}

/// Where a query macro's SQL comes from.
#[ derive( Clone, Copy ) ]
enum SqlSource
{
  /// String literals (joined by `+`) at this argument position.
  Literal( usize ),
  /// A path literal at this argument position, naming a file relative to the crate root.
  File( usize ),
}

/// Every sqlx 0.9 query macro, mapped to the argument that carries its SQL.
fn sql_source( macro_name : &str ) -> Option< SqlSource >
{
  // Fix(unchecked_variants_mre_are_scanned): the table lists all twelve sqlx 0.9 query macros.
  // Root cause: only query!/query_as!/query_scalar! were recognized, assuming the `_unchecked`
  // and `query_file*` variants never consult .sqlx -- every one of them does.
  // Pitfall: `_unchecked` only skips bind-argument type checks; the cache lookup is identical.
  match macro_name
  {
    "query" | "query_unchecked" | "query_scalar" | "query_scalar_unchecked" => Some( SqlSource::Literal( 0 ) ),
    "query_as" | "query_as_unchecked" => Some( SqlSource::Literal( 1 ) ),
    "query_file" | "query_file_unchecked" | "query_file_scalar" | "query_file_scalar_unchecked" => Some( SqlSource::File( 0 ) ),
    "query_file_as" | "query_file_as_unchecked" => Some( SqlSource::File( 1 ) ),
    _ => None,
  }
}

/// Walk the package rooted at `manifest_dir`: collect every query-macro call
/// site in its `.rs` files, and every path whose change must re-run the
/// build script.
///
/// Excludes what is not this package's own source, close to the way Cargo's
/// package listing does: every directory in `skip`, hidden entries (`.git/`,
/// `.sqlx/`), entries git ignores, Cargo target directories (`target/` at the
/// root, or any directory holding the `CACHEDIR.TAG` Cargo writes into every
/// target directory), and nested packages (a subdirectory with its own
/// `Cargo.toml`).
///
/// `watched` holds every top-level entry that was walked, except that a
/// directory with an excluded entry somewhere below it (hidden entries aside)
/// is replaced by its own entries, recursively: `cargo:rerun-if-changed`
/// watches a directory recursively, and excluded entries -- a target
/// directory, a log, a cache sqlx writes into -- may change during every build.
#[ must_use ]
pub fn scan_package( manifest_dir : &Path, skip : &[ PathBuf ] ) -> PackageScan
{
  let exclusions = Exclusions { skip, git_ignored : git_ignored( manifest_dir ) };
  let mut scan = PackageScan::default();
  walk_dir( manifest_dir, manifest_dir, &exclusions, &mut scan.sites, &mut scan.watched );
  scan.watched.extend( scan.sites.iter().filter_map( | site | site.sql_file.clone() ) );
  scan.watched.sort();
  scan.watched.dedup();
  scan
}

/// Entries [`walk_dir`] leaves out, beyond the ones it recognizes by itself.
struct Exclusions< 'a >
{
  skip : &'a [ PathBuf ],
  git_ignored : Vec< PathBuf >,
}

/// Scan the entries of `dir`, returning whether its subtree holds an excluded
/// entry -- one that must not be watched as part of a directory.
fn walk_dir
(
  dir : &Path,
  manifest_dir : &Path,
  exclusions : &Exclusions< '_ >,
  sites : &mut Vec< QuerySite >,
  watched : &mut Vec< PathBuf >,
) -> bool
{
  let Ok( entries ) = std::fs::read_dir( dir ) else { return false };
  let mut entries : Vec< _ > = entries.filter_map( Result::ok ).collect();
  entries.sort_by_key( std::fs::DirEntry::file_name );

  let mut holds_excluded = false;
  for entry in entries
  {
    let path = entry.path();
    let Ok( kind ) = entry.file_type() else { continue };

    // Checked before the hidden-entry rule: a cache directory sqlx writes into is never
    // watched through its parent, hidden or not.
    if exclusions.skip.contains( &path )
    {
      holds_excluded = true;
      continue;
    }
    if entry.file_name().to_string_lossy().starts_with( '.' )
    {
      continue;
    }
    // Cargo's own package listing leaves git-ignored files out, so before sqlx_query emitted
    // rerun-if directives, a log or a `node_modules/` changing never re-ran the build script.
    if exclusions.git_ignored.contains( &path )
    {
      holds_excluded = true;
      continue;
    }

    if kind.is_dir()
    {
      // Fix(target_dir_with_cachedir_tag_mre_is_skipped): a directory holding CACHEDIR.TAG is
      // a Cargo target directory, whatever its name.
      // Root cause: only directories literally named `target` were skipped.
      // Pitfall: CARGO_TARGET_DIR can point anywhere inside the package; watching such a
      // directory would re-run the build script on every build.
      //
      // Fix(nested_package_mre_is_not_scanned): a subdirectory with its own Cargo.toml is
      // another package, compiled and cached on its own.
      // Root cause: the walk descended into nested packages and reported their queries
      // as this crate's stale entries.
      // Pitfall: Cargo's own package listing stops at nested packages; this walk must too.
      if is_target_dir( &path, dir == manifest_dir ) || path.join( "Cargo.toml" ).is_file()
      {
        holds_excluded = true;
        continue;
      }

      let mut below = Vec::new();
      if walk_dir( &path, manifest_dir, exclusions, sites, &mut below )
      {
        holds_excluded = true;
        watched.append( &mut below );
      }
      else
      {
        watched.push( path );
      }
    }
    else if kind.is_file()
    {
      if path.extension().is_some_and( | ext | ext == "rs" )
      {
        scan_file( &path, manifest_dir, sites );
      }
      watched.push( path );
    }
  }

  holds_excluded
}

fn is_target_dir( dir : &Path, at_root : bool ) -> bool
{
  ( at_root && dir.file_name().is_some_and( | name | name == "target" ) ) || dir.join( "CACHEDIR.TAG" ).is_file()
}

/// The entries below `dir` that git ignores, as git reports them -- a wholly
/// ignored directory as a single entry. Empty when git is unavailable or `dir`
/// is not inside a repository: then nothing is excluded on this ground.
fn git_ignored( dir : &Path ) -> Vec< PathBuf >
{
  let output = Command::new( "git" )
  .arg( "-C" ).arg( dir )
  .args( [ "ls-files", "-z", "--others", "--ignored", "--exclude-standard", "--directory" ] )
  .stderr( Stdio::null() )
  .output();

  let Ok( output ) = output else { return Vec::new() };
  if !output.status.success()
  {
    return Vec::new();
  }

  output.stdout
  .split( | byte | *byte == 0 )
  .filter_map( | entry | core::str::from_utf8( entry ).ok() )
  .filter( | entry | !entry.is_empty() )
  .map( | entry | dir.join( entry.trim_end_matches( '/' ) ) )
  .collect()
}

fn scan_file( path : &Path, manifest_dir : &Path, sites : &mut Vec< QuerySite > )
{
  let Ok( source ) = std::fs::read_to_string( path ) else { return };
  let Ok( tokens ) = source.parse::< TokenStream >() else { return };

  let file = path.strip_prefix( manifest_dir ).unwrap_or( path ).to_path_buf();
  collect_sites( tokens, &file, manifest_dir, sites );
}

/// Find every `name!( ... )` invocation of a sqlx query macro in `tokens`, at
/// any nesting depth.
fn collect_sites( tokens : TokenStream, file : &Path, manifest_dir : &Path, sites : &mut Vec< QuerySite > )
{
  // Fix(nested_macro_sites_mre_are_scanned): walk raw token trees, descending into every group.
  // Root cause: syn's AST visitor never looks inside another macro's tokens, so a query in
  // `vec![ .. ]`, `assert!( .. )`, `tokio::join!( .. )` or a `macro_rules!` body was invisible.
  // Pitfall: those sites still need .sqlx entries -- missing one silently disables
  // SQLX_QUERY_SOFT_WARN for it, since sqlx_query then believes nothing is stale.
  let trees : Vec< TokenTree > = tokens.into_iter().collect();
  for ( index, tree ) in trees.iter().enumerate()
  {
    let TokenTree::Group( group ) = tree else { continue };

    if let [ .., TokenTree::Ident( name ), TokenTree::Punct( bang ) ] = &trees[ ..index ]
    {
      if bang.as_char() == '!'
      {
        let resolved = sql_source( &name.to_string() )
        .and_then( | source | resolve_sql( group.stream(), source, manifest_dir ) );
        if let Some( ( sql, sql_file ) ) = resolved
        {
          sites.push( QuerySite { file : file.to_path_buf(), line : name.span().start().line, sql, sql_file } );
        }
      }
    }

    collect_sites( group.stream(), file, manifest_dir, sites );
  }
}

/// The SQL text a query macro with arguments `arguments` compiles, or `None`
/// when it is not statically known (a `const`, a `format!` call -- which sqlx
/// rejects anyway).
fn resolve_sql( arguments : TokenStream, source : SqlSource, manifest_dir : &Path ) -> Option< ( String, Option< PathBuf > ) >
{
  let arguments = split_arguments( arguments );
  let ( SqlSource::Literal( position ) | SqlSource::File( position ) ) = source;

  // A record type (`query_as!( Wrapper< A, B >, .. )`) may itself contain
  // commas, so after position 0 the SQL is the first all-literal argument.
  let literals = if position == 0
  {
    string_literals( arguments.first()? )?
  }
  else
  {
    arguments.iter().skip( 1 ).find_map( | argument | string_literals( argument ) )?
  };

  match source
  {
    SqlSource::Literal( _ ) => Some( ( literals.concat(), None ) ),
    SqlSource::File( _ ) =>
    {
      let [ relative ] = literals.as_slice() else { return None };
      let path = query_file_path( manifest_dir, relative )?;
      let sql = std::fs::read_to_string( &path ).ok()?;
      Some( ( sql, Some( path ) ) )
    }
  }
}

/// Split a macro's argument tokens at its top-level commas -- commas inside a
/// nested group are part of that group's single token tree.
fn split_arguments( tokens : TokenStream ) -> Vec< Vec< TokenTree > >
{
  let mut arguments = vec![ Vec::new() ];
  for tree in tokens
  {
    match &tree
    {
      TokenTree::Punct( punct ) if punct.as_char() == ',' => arguments.push( Vec::new() ),
      _ =>
      {
        if let Some( current ) = arguments.last_mut()
        {
          current.push( tree );
        }
      }
    }
  }
  arguments
}

/// The values of `argument` when it is string literals joined by `+` -- the
/// exact `source` grammar sqlx accepts (`Punctuated< LitStr, Token![ + ] >`).
fn string_literals( argument : &[ TokenTree ] ) -> Option< Vec< String > >
{
  // Fix(concatenated_literals_mre_are_scanned): accept `"a" + "b"` the way sqlx does.
  // Root cause: only a single string literal was recognized as the SQL argument.
  // Pitfall: sqlx hashes the pieces joined with no separator -- never insert one.
  let mut values = Vec::new();
  let mut trees = argument.iter();
  loop
  {
    let TokenTree::Literal( literal ) = trees.next()? else { return None };
    let syn::Lit::Str( text ) = syn::Lit::new( literal.clone() ) else { return None };
    values.push( text.value() );
    match trees.next()
    {
      None => return Some( values ),
      Some( TokenTree::Punct( plus ) ) if plus.as_char() == '+' => {}
      Some( _ ) => return None,
    }
  }
}

/// Resolve a `query_file*!` path the way sqlx does: relative to the crate
/// root, with absolute paths and bare file names rejected (sqlx refuses both).
fn query_file_path( manifest_dir : &Path, relative : &str ) -> Option< PathBuf >
{
  // Fix(query_file_variants_mre_are_scanned): read the SQL file a `query_file*!` site names.
  // Root cause: the `query_file*` family was out of scope, so its sites were never checked.
  // Pitfall: sqlx hashes the file's exact contents, trailing newline included.
  let path = Path::new( relative );
  let has_parent = path.parent().is_some_and( | parent | !parent.as_os_str().is_empty() );
  ( path.is_relative() && has_parent ).then( || manifest_dir.join( path ) )
}
