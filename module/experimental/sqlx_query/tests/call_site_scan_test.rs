//! Which macro calls count as sqlx query sites, and which SQL text -- hence
//! which `.sqlx` entry -- each one is keyed by.
//!
//! Most tests use the divergence pair in `support::assert_site_keyed_by`: the
//! site is reported stale while its entry is missing, and fresh once the entry
//! for exactly the expected SQL exists. That proves the site is found *and*
//! keyed correctly, which "no warning" alone never could. Expected entry names
//! come from `support::entry_name` or from independently computed digests.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `query_sql_is_the_first_argument` | `query!` | `query!( "SELECT 1" )` | keyed by `SELECT 1` | ✅ |
//! | `query_as_sql_follows_the_record_type` | Record type with a top-level comma | `query_as!( Pair< i64, String >, "SELECT .." )` | keyed by the SQL | ✅ |
//! | `query_scalar_sql_is_the_first_argument` | `query_scalar!` | `query_scalar!( "SELECT COUNT(*) FROM t" )` | keyed by the SQL | ✅ |
//! | `bind_arguments_never_change_the_cache_key` | Same SQL, different binds | two sites binding `1` and `id` | one entry serves both | ✅ |
//! | `string_escapes_are_decoded_before_hashing` | `\n` escape | `"SELECT id\nFROM t"` | keyed by the text with a real newline | ✅ |
//! | `raw_string_literals_are_scanned` | Raw string with quotes | `r#"SELECT "id" FROM t"#` | keyed by the raw text | ✅ |
//! | `concatenated_literals_mre_are_scanned` | `+`-joined literals (MRE) | `"SELECT id " + "FROM t"` | keyed by the pieces joined with no separator | ✅ |
//! | `unchecked_variants_mre_are_scanned` | `_unchecked` variants (MRE) | `query_unchecked!`, `query_as_unchecked!`, `query_scalar_unchecked!` | three sites | ✅ |
//! | `query_file_variants_mre_are_scanned` | All six `query_file*` macros (MRE) | each naming `queries/q.sql` | six sites, keyed by the file's contents | ✅ |
//! | `query_file_is_keyed_by_exact_file_contents` | Trailing newline | entry for the trimmed text | still stale | ✅ |
//! | `query_file_paths_sqlx_rejects_are_skipped` | Bare name, absolute path, missing file | `q.sql`, an absolute path to an existing file, `queries/missing.sql` | no site | ✅ |
//! | `nested_macro_sites_mre_are_scanned` | Inside `vec!`, `assert!`, `macro_rules!` (MRE) | three nested sites | three sites | ✅ |
//! | `non_literal_sql_is_skipped` | `const`, `concat!` | `query!( SQL )`, `query!( concat!( .. ) )` | no site | ✅ |
//! | `unrelated_macros_are_ignored` | Lookalikes | `println!`, `format!`, `sqlx::query(`, `query_builder!` | no site | ✅ |
//! | `commented_out_queries_are_ignored` | Line, block and doc comments | three commented `query!` calls | no site | ✅ |
//! | `unparseable_source_is_skipped` | Unbalanced delimiters in one file | `src/broken.rs` + a valid `src/lib.rs` | only the valid file's site | ✅ |
//! | Which files are scanned at all | — | — | `package_walk_test.rs` | N/A here |

#![ cfg( feature = "enabled" ) ]

mod support;

use support::{ assert_site_keyed_by, entry_name, site_warning, site_warnings, PackageDir };

/// `query!`'s SQL is its first argument.
#[ test ]
fn query_sql_is_the_first_argument()
{
  let package = PackageDir::new( r#"fn f()
{
  let _ = sqlx::query!( "SELECT 1" );
}
"# );
  assert_site_keyed_by( &package, 3, &entry_name( "SELECT 1" ) );
}

/// `query_as!`'s SQL follows the record type -- which may itself contain a
/// top-level comma, since `<` and `>` do not group tokens.
#[ test ]
fn query_as_sql_follows_the_record_type()
{
  let package = PackageDir::new( r#"fn f()
{
  let _ = sqlx::query_as!( Pair< i64, String >, "SELECT 1 AS a, 'x' AS b", 7 );
}
"# );
  assert_site_keyed_by( &package, 3, &entry_name( "SELECT 1 AS a, 'x' AS b" ) );
}

/// `query_scalar!`'s SQL is its first argument.
#[ test ]
fn query_scalar_sql_is_the_first_argument()
{
  let package = PackageDir::new( r#"fn f()
{
  let _ = sqlx::query_scalar!( "SELECT COUNT(*) FROM t" );
}
"# );
  assert_site_keyed_by( &package, 3, &entry_name( "SELECT COUNT(*) FROM t" ) );
}

/// sqlx keys an entry by SQL text alone, never by bind arguments: two sites
/// running the same SQL with different binds share one entry
/// (`docs/invariant/001_query_dispatch_invariant.md`).
#[ test ]
fn bind_arguments_never_change_the_cache_key()
{
  let package = PackageDir::new( r#"fn f( id : i64 )
{
  let _ = sqlx::query!( "SELECT id FROM t WHERE id = ?", 1 );
  let _ = sqlx::query!( "SELECT id FROM t WHERE id = ?", id );
}
"# );
  package.mkdir( ".sqlx" );

  let before = package.directives( &[] );
  assert_eq!( site_warnings( &before ), [ site_warning( "src/lib.rs", 3 ), site_warning( "src/lib.rs", 4 ) ], "{before:#?}" );

  package.cache_entry( ".sqlx", "SELECT id FROM t WHERE id = ?" );
  let after = package.directives( &[] );
  assert_eq!( site_warnings( &after ), Vec::< String >::new(), "one entry should serve both sites: {after:#?}" );
}

/// sqlx hashes the literal's value, not its source spelling: `\n` in the
/// source is a newline in the SQL (digest computed with `printf | sha256sum`).
#[ test ]
fn string_escapes_are_decoded_before_hashing()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT id\\nFROM t\" ); }\n" );
  assert_site_keyed_by( &package, 1, "query-bfad2a5c0b136bfed8574b71dd68c6dfe0d96cf5dc742c8e9d5c49f1d3039bd9.json" );
}

/// A raw string's value is its text verbatim, embedded quotes included.
#[ test ]
fn raw_string_literals_are_scanned()
{
  let package = PackageDir::new( r##"fn f() { let _ = sqlx::query!( r#"SELECT "id" FROM t"# ); }
"## );
  assert_site_keyed_by( &package, 1, "query-b5fd4f324ad7ccc7a3ac805c049934128f1ed4ea86409613587119b542b534de.json" );
}

/// sqlx accepts SQL split into `+`-joined literals and hashes the pieces
/// joined with no separator.
///
/// ## Root Cause
///
/// The first scanner (`src/scan.rs`, a `syn` visitor) accepted only a single
/// `LitStr` as the SQL argument, so `query!( "SELECT id " + "FROM t" )` -- valid
/// sqlx, whose `source` grammar is `Punctuated< LitStr, Token![ + ] >` -- was
/// silently not a site at all.
///
/// ## Why Not Caught
///
/// Scanner tests used one literal per call; sqlx's macro grammar was never
/// read for other accepted shapes.
///
/// ## Fix Applied
///
/// `string_literals` in `src/scan.rs` accepts `Lit ( + Lit )*` and
/// `resolve_sql` joins the decoded values with `concat()`.
///
/// ## Prevention
///
/// Derive accepted argument shapes from sqlx's own macro input grammar, not
/// from typical usage.
///
/// ## Pitfall
///
/// Joining with a space (or any separator) produces a different digest; the
/// pieces must be concatenated exactly.
// test_kind: mre
#[ test ]
fn concatenated_literals_mre_are_scanned()
{
  let package = PackageDir::new( r#"fn f()
{
  let _ = sqlx::query!( "SELECT id " + "FROM t" );
}
"# );
  assert_site_keyed_by( &package, 3, &entry_name( "SELECT id FROM t" ) );
}

/// The `_unchecked` macros read and write `.sqlx` exactly like their checked
/// counterparts.
///
/// ## Root Cause
///
/// The macro table in `src/scan.rs` listed only `query`, `query_as` and
/// `query_scalar`, on the assumption that `_unchecked` variants skip
/// compile-time checking -- they skip only bind-argument type checks; the
/// offline cache lookup is identical, so their stale entries went unreported.
///
/// ## Why Not Caught
///
/// No test used an `_unchecked` macro, and the scope decision was never
/// checked against `sqlx-macros-core`, where all variants share one expansion.
///
/// ## Fix Applied
///
/// `sql_source` in `src/scan.rs` maps all twelve sqlx 0.9 query macros to the
/// argument carrying their SQL.
///
/// ## Prevention
///
/// When sqlx adds a query macro, add it to `sql_source` and to this test.
///
/// ## Pitfall
///
/// `query_as_unchecked!`'s SQL is its second argument, like `query_as!`'s.
// test_kind: mre
#[ test ]
fn unchecked_variants_mre_are_scanned()
{
  let package = PackageDir::new( r#"fn f( id : i64 )
{
  let _ = sqlx::query_unchecked!( "SELECT 1" );
  let _ = sqlx::query_as_unchecked!( Row, "SELECT 2", id );
  let _ = sqlx::query_scalar_unchecked!( "SELECT 3" );
}
"# );
  package.mkdir( ".sqlx" );

  let before = package.directives( &[] );
  let expected = [ site_warning( "src/lib.rs", 3 ), site_warning( "src/lib.rs", 4 ), site_warning( "src/lib.rs", 5 ) ];
  assert_eq!( site_warnings( &before ), expected, "{before:#?}" );

  for sql in [ "SELECT 1", "SELECT 2", "SELECT 3" ]
  {
    package.cache_entry( ".sqlx", sql );
  }
  let after = package.directives( &[] );
  assert_eq!( site_warnings( &after ), Vec::< String >::new(), "{after:#?}" );
}

/// All six `query_file*` macros key their entry by the contents of the SQL
/// file they name, resolved against the crate root.
///
/// ## Root Cause
///
/// The `query_file*` family was declared out of scope, so `src/scan.rs` never
/// recognized it: a crate keeping its SQL in files got no warnings at all, and
/// `SQLX_QUERY_SOFT_WARN=1` could never apply to those sites.
///
/// ## Why Not Caught
///
/// The scope decision assumed external SQL files were rare; no test covered
/// them, and the manual scenario that did (S4) ran only after release.
///
/// ## Fix Applied
///
/// `sql_source` maps the six macros to `SqlSource::File`; `resolve_sql` reads
/// the named file (`query_file_path`), and its path joins the rerun-watched set.
///
/// ## Prevention
///
/// Treat every macro that consults `.sqlx` as in scope; the cache does not
/// care how the SQL reached the macro.
///
/// ## Pitfall
///
/// sqlx hashes the file's exact contents -- a trailing newline is part of the
/// key (`query_file_is_keyed_by_exact_file_contents`).
// test_kind: mre
#[ test ]
fn query_file_variants_mre_are_scanned()
{
  let package = PackageDir::new( r#"fn f( id : i64 )
{
  let _ = sqlx::query_file!( "queries/q.sql" );
  let _ = sqlx::query_file_unchecked!( "queries/q.sql" );
  let _ = sqlx::query_file_as!( Row, "queries/q.sql" );
  let _ = sqlx::query_file_as_unchecked!( Row, "queries/q.sql", id );
  let _ = sqlx::query_file_scalar!( "queries/q.sql" );
  let _ = sqlx::query_file_scalar_unchecked!( "queries/q.sql" );
}
"# );
  package.write( "queries/q.sql", "SELECT id FROM t\n" );
  package.mkdir( ".sqlx" );

  let before = package.directives( &[] );
  let expected : Vec< String > = ( 3..=8 ).map( | line | site_warning( "src/lib.rs", line ) ).collect();
  assert_eq!( site_warnings( &before ), expected, "{before:#?}" );

  package.cache_entry( ".sqlx", "SELECT id FROM t\n" );
  let after = package.directives( &[] );
  assert_eq!( site_warnings( &after ), Vec::< String >::new(), "{after:#?}" );
}

/// The key is the file's exact contents: an entry for the same SQL without the
/// file's trailing newline is a different entry.
#[ test ]
fn query_file_is_keyed_by_exact_file_contents()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query_file!( \"queries/q.sql\" ); }\n" );
  package.write( "queries/q.sql", "SELECT id FROM t\n" );
  package.cache_entry( ".sqlx", "SELECT id FROM t" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), [ site_warning( "src/lib.rs", 1 ) ], "{directives:#?}" );
}

/// sqlx refuses a bare file name and an absolute path, and fails on a missing
/// file -- none of them is a site whose entry could be checked.
#[ test ]
fn query_file_paths_sqlx_rejects_are_skipped()
{
  let package = PackageDir::new( "" );
  package.write( "q.sql", "SELECT 1" );
  let absolute = package.write( "queries/q.sql", "SELECT 2" );
  package.write( "src/lib.rs", &format!(
    "fn f()\n{{\n  let _ = sqlx::query_file!( \"q.sql\" );\n  let _ = sqlx::query_file!( {absolute:?} );\n  let _ = sqlx::query_file!( \"queries/missing.sql\" );\n}}\n"
  ) );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// A query nested inside another macro's arguments is still compiled -- and
/// still needs its entry.
///
/// ## Root Cause
///
/// `src/scan.rs` walked the AST with `syn::visit::Visit`, which never looks
/// inside another macro's token stream: a query in `vec![ .. ]`,
/// `assert!( .. )`, `tokio::join!( .. )` or a `macro_rules!` body was invisible,
/// so its stale entry went unreported and `SQLX_QUERY_SOFT_WARN=1` did not
/// apply to it.
///
/// ## Why Not Caught
///
/// Every scanner test placed the query directly in a function body.
///
/// ## Fix Applied
///
/// `collect_sites` in `src/scan.rs` lexes each file into raw token trees,
/// matches `name ! ( .. )` anywhere, and recurses into every group.
///
/// ## Prevention
///
/// Test a source shape where the call is wrapped in each common macro.
///
/// ## Pitfall
///
/// A `macro_rules!` body whose SQL is a `$sql` fragment is not statically
/// known and stays skipped -- only literal SQL is resolvable.
// test_kind: mre
#[ test ]
fn nested_macro_sites_mre_are_scanned()
{
  let package = PackageDir::new( r#"macro_rules! first_id
{
  () => { sqlx::query_scalar!( "SELECT 1" ) };
}

fn f()
{
  let _ = vec![ sqlx::query!( "SELECT 2" ) ];
  assert!( sqlx::query!( "SELECT 3" ).is_valid() );
}
"# );
  package.mkdir( ".sqlx" );

  let before = package.directives( &[] );
  let expected = [ site_warning( "src/lib.rs", 3 ), site_warning( "src/lib.rs", 8 ), site_warning( "src/lib.rs", 9 ) ];
  assert_eq!( site_warnings( &before ), expected, "{before:#?}" );

  for sql in [ "SELECT 1", "SELECT 2", "SELECT 3" ]
  {
    package.cache_entry( ".sqlx", sql );
  }
  let after = package.directives( &[] );
  assert_eq!( site_warnings( &after ), Vec::< String >::new(), "{after:#?}" );
}

/// SQL that is not literal text -- a `const`, a `concat!` -- is not statically
/// known (sqlx rejects it anyway), so it is not a site.
#[ test ]
fn non_literal_sql_is_skipped()
{
  let package = PackageDir::new( r#"const SQL : &str = "SELECT 1";

fn f()
{
  let _ = sqlx::query!( SQL );
  let _ = sqlx::query!( concat!( "SELECT ", "2" ) );
}
"# );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// Only the twelve sqlx query macros are sites: other macros taking SQL-like
/// strings, and the runtime `sqlx::query` function, are not.
#[ test ]
fn unrelated_macros_are_ignored()
{
  let package = PackageDir::new( r#"fn f()
{
  println!( "SELECT 1" );
  let _ = format!( "SELECT 2" );
  let _ = sqlx::query( "SELECT 3" );
  let _ = query_builder!( "SELECT 4" );
}
"# );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// Commented-out code is never compiled, so it is not a site -- including
/// inside a doc comment, which lexes as one string-literal attribute.
#[ test ]
fn commented_out_queries_are_ignored()
{
  let package = PackageDir::new( r#"/// Runs `sqlx::query!( "SELECT 1" )`.
fn f()
{
  // let _ = sqlx::query!( "SELECT 2" );
  /* let _ = sqlx::query!( "SELECT 3" ); */
}
"# );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// A file that does not lex is skipped whole -- rustc reports it -- while the
/// rest of the package is still scanned.
#[ test ]
fn unparseable_source_is_skipped()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.write( "src/broken.rs", "fn g( {\n  let _ = sqlx::query!( \"SELECT 2\" );\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), [ site_warning( "src/lib.rs", 1 ) ], "{directives:#?}" );
}
