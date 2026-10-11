use std::io::{ BufRead, BufReader, Read };
use std::path::{ Path, PathBuf };
use std::process::{ Command, Stdio };

/// Regenerate stale `.sqlx` entries with a real, isolated, live `cargo check`
/// of the consumer crate, writing into `cache_dir`. `Ok` once the check
/// succeeded -- the caller re-diffs and reports with [`regenerated`] -- or the
/// warning saying why it did not run or failed.
///
/// The caller only invokes this with a database URL configured. The nested
/// invocation always gets its own `--target-dir`: reusing the shared/default
/// target directory deadlocks (see
/// `docs/invariant/002_consumer_integration_contract.md`, Isolated
/// Target-Dir Regeneration).
pub fn attempt_regeneration( manifest_dir : &Path, cache_dir : &Path ) -> Result< (), String >
{
  let Some( cargo ) = std::env::var_os( "CARGO" ) else
  {
    return Err( "cargo:warning=sqlx_query: auto_regen enabled but CARGO is unset; skipping regeneration".to_string() );
  };

  let isolated_target = std::env::var_os( "OUT_DIR" )
  .map_or_else( || manifest_dir.join( "target" ), PathBuf::from )
  .join( "sqlx_query-offline-regen" );

  // The nested check runs this same consumer build.rs again. Its explicit
  // SQLX_OFFLINE=false makes that run stand aside (see Options::offline), so
  // sqlx compiles live and writes into SQLX_OFFLINE_DIR. Removing
  // SQLX_QUERY_AUTO_REGEN is a second, independent recursion guard: without
  // it, an earlier version recursed 11+ levels deep (6.8 GiB of nested
  // target directories) before being killed. The database URL is inherited,
  // or read from `.env` by sqlx itself; the variables set here beat `.env`.
  let child = Command::new( &cargo )
  .arg( "check" )
  .arg( "--manifest-path" ).arg( manifest_dir.join( "Cargo.toml" ) )
  .arg( "--target-dir" ).arg( &isolated_target )
  // Fix(auto_regen_mre_repairs_queries_outside_the_default_targets): check every target with
  // every feature.
  // Root cause: a plain `cargo check` compiles only the library and binaries with default
  // features, so a stale query in tests/, examples/, benches/ or behind a feature was reported
  // but never regenerated, and `cargo test` kept failing with SQLX_QUERY_AUTO_REGEN=1 set.
  // Pitfall: Cargo runs this build script per package, not per target, so the nested check
  // cannot know which targets the outer build compiles -- it has to cover them all.
  .arg( "--all-targets" )
  .arg( "--all-features" )
  .env( "SQLX_OFFLINE", "false" )
  .env( "SQLX_OFFLINE_DIR", cache_dir )
  .env_remove( "SQLX_QUERY_AUTO_REGEN" )
  // Fix(auto_regen_mre_keeps_the_nested_check_output_captured): pipe the check's stderr and pass
  // it on through `eprint!`.
  // Root cause: the inherited descriptor bypassed the test harness's capture, so a passing
  // `cargo test` printed the nested build's compile errors into the log, CI's included.
  // Pitfall: Cargo shows a build script's stderr only when it fails, or as it runs under `-vv` --
  // pass it on line by line, and read to the end, or the check blocks on a full pipe.
  .stderr( Stdio::piped() )
  .spawn();

  let mut child = match child
  {
    Ok( child ) => child,
    Err( error ) => return Err( format!( "cargo:warning=sqlx_query: auto_regen failed to spawn `cargo check`: {error}" ) ),
  };
  if let Some( stderr ) = child.stderr.take()
  {
    pass_on_lines( stderr );
  }

  match child.wait()
  {
    Ok( status ) if status.success() => Ok( () ),
    Ok( status ) =>
    {
      Err( format!( "cargo:warning=sqlx_query: auto_regen's isolated `cargo check` exited with {status}; .sqlx entries may still be stale" ) )
    }
    Err( error ) =>
    {
      Err( format!( "cargo:warning=sqlx_query: auto_regen could not wait for `cargo check`: {error}" ) )
    }
  }
}

/// Passes `stream` on through `eprint!` -- which the test harness captures and
/// a build script's own stderr receives -- one line at a time, to its end.
fn pass_on_lines( stream : impl Read )
{
  let mut reader = BufReader::new( stream );
  let mut line = Vec::new();
  while reader.read_until( b'\n', &mut line ).is_ok_and( | read | read > 0 )
  {
    eprint!( "{}", String::from_utf8_lossy( &line ) );
    line.clear();
  }
}

/// The report after a successful nested check, given how many sites were
/// stale before it and how many the re-diff still finds: what the check never
/// compiles -- `#[ cfg( any() ) ]` code, another platform's -- stays stale.
#[ must_use ]
pub fn regenerated( before : usize, remaining : usize ) -> String
{
  if remaining == 0
  {
    "cargo:warning=sqlx_query: regenerated stale .sqlx entries via an isolated live `cargo check`".to_string()
  }
  else
  {
    format!(
      "cargo:warning=sqlx_query: regenerated {} of {before} stale .sqlx entries via an isolated live `cargo check`, which compiles none of the code holding the rest",
      before.saturating_sub( remaining ),
    )
  }
}

/// The notice for `SQLX_QUERY_AUTO_REGEN=1` without a database URL --
/// `database_url_var` unset or empty, as sqlx sees it in the environment and
/// in `.env` files: there is nothing to regenerate against.
#[ must_use ]
pub fn skipped_without_database_url( database_url_var : &str ) -> String
{
  format!( "cargo:warning=sqlx_query: auto_regen skipped: {database_url_var} is unset or empty" )
}

/// The notice for `SQLX_QUERY_AUTO_REGEN=1` while `SQLX_OFFLINE_DIR` names
/// `dir`, which is not a directory: sqlx writes no entry there, so a nested
/// check against it could only fail.
#[ must_use ]
pub fn skipped_without_offline_dir( dir : &Path ) -> String
{
  format!( "cargo:warning=sqlx_query: auto_regen skipped: SQLX_OFFLINE_DIR names {}, which is not a directory", dir.display() )
}
