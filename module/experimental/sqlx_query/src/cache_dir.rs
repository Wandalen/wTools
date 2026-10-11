use std::path::{ Path, PathBuf };
use std::process::{ Command, Stdio };

/// The `.sqlx` directories sqlx consults for one crate.
#[ derive( Debug, Clone, Default ) ]
pub struct CacheDirs
{
  /// sqlx's own per-query search order -- `SQLX_OFFLINE_DIR`, then
  /// `<crate>/.sqlx`, then `<workspace root>/.sqlx`, deduplicated. For each
  /// query, the first directory holding its file wins.
  pub lookup : Vec< PathBuf >,
  /// Where a live build should write fresh entries: `SQLX_OFFLINE_DIR` when
  /// set -- even when it is not a directory, which sqlx then fails on instead
  /// of creating -- otherwise the first of the two `.sqlx` directories that
  /// exists. `None` means the crate has no offline cache yet.
  pub primary : Option< PathBuf >,
}

/// Resolve the cache directories of the crate at `manifest_dir`, given the
/// `SQLX_OFFLINE_DIR` value in effect (if any).
#[ must_use ]
pub fn resolve( manifest_dir : &Path, offline_dir : Option< &Path > ) -> CacheDirs
{
  let workspace_root = workspace_root( manifest_dir );

  // Fix(relative_offline_dir_mre_resolves_from_workspace_root): resolve a relative
  // SQLX_OFFLINE_DIR against the workspace root.
  // Root cause: it was used relative to this build script's working directory, the crate root.
  // Pitfall: sqlx runs inside rustc, which Cargo starts from the workspace root, not the crate.
  let explicit = offline_dir.map( | dir | workspace_root.as_deref().unwrap_or( manifest_dir ).join( dir ) );
  let local = manifest_dir.join( ".sqlx" );
  let shared = workspace_root.map( | root | root.join( ".sqlx" ) );

  let mut lookup : Vec< PathBuf > = Vec::new();
  for dir in explicit.iter().chain( Some( &local ) ).chain( shared.iter() )
  {
    if !lookup.contains( dir )
    {
      lookup.push( dir.clone() );
    }
  }

  let primary = explicit.or_else( || lookup.iter().find( | dir | dir.is_dir() ).cloned() );
  CacheDirs { lookup, primary }
}

/// The root of the workspace the crate at `manifest_dir` belongs to, as Cargo
/// itself determines it -- `None` when Cargo cannot tell.
fn workspace_root( manifest_dir : &Path ) -> Option< PathBuf >
{
  // Fix(nested_workspace_mre_resolves_the_nearest_root): ask Cargo for the workspace root.
  // Root cause: a text search for the outermost `[workspace]` ancestor picked an enclosing
  // workspace over the one the crate actually belongs to, so live builds wrote another
  // project's .sqlx.
  // Pitfall: sqlx itself asks Cargo (`cargo metadata`); excluded packages, nested workspaces
  // and `package.workspace` all defeat a manifest-text heuristic.
  let cargo = std::env::var_os( "CARGO" ).unwrap_or_else( || "cargo".into() );
  let output = Command::new( cargo )
  .args( [ "locate-project", "--workspace", "--message-format", "plain" ] )
  .current_dir( manifest_dir )
  .env_remove( "__CARGO_FIX_PLZ" )
  .stderr( Stdio::null() )
  .output()
  .ok()?;

  if !output.status.success()
  {
    return None;
  }

  let root_manifest = PathBuf::from( String::from_utf8( output.stdout ).ok()?.trim_end() );
  root_manifest.parent().map( Path::to_path_buf )
}
