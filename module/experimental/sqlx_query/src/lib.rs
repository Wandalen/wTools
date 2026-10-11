#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/sqlx_query/latest/sqlx_query/" ) ]
// The readme's example runs as a doctest and calls `ensure_offline_mode`, which only `enabled` compiles.
#![ cfg_attr( all( doc, feature = "enabled" ), doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]

//!
//! Force sqlx's `SQLX_OFFLINE` mode from your own `build.rs`, and warn --
//! before CI or a teammate without a database ever finds out -- when a
//! query's cached `.sqlx/` metadata no longer matches its SQL text.
//!

#[ cfg( feature = "enabled" ) ] mod options;
#[ cfg( feature = "enabled" ) ] mod hash;
#[ cfg( feature = "enabled" ) ] mod scan;
#[ cfg( feature = "enabled" ) ] mod cache_dir;
#[ cfg( feature = "enabled" ) ] mod report;
#[ cfg( feature = "auto_regen" ) ] mod regen;

#[ cfg( feature = "enabled" ) ]
pub use options::Options;

/// Variables whose value changes this crate's decisions, besides the one
/// holding the database URL ([`Options::database_url_var`]); each is declared
/// to Cargo with `cargo:rerun-if-env-changed`.
#[ cfg( feature = "enabled" ) ]
const TRACKED_ENV_VARS : [ &str; 4 ] =
[
  "SQLX_OFFLINE",
  "SQLX_OFFLINE_DIR",
  "SQLX_QUERY_SOFT_WARN",
  "SQLX_QUERY_AUTO_REGEN",
];

/// Force `SQLX_OFFLINE=true` for this build, after scanning this crate's own
/// sources for sqlx query-macro call sites and warning about any whose
/// `.sqlx` cache entry is missing.
///
/// Call this from `fn main` in your own `build.rs`:
///
/// ```no_run
/// sqlx_query::ensure_offline_mode();
/// ```
///
/// Reads [`Options::from_env`] for its configuration. An explicit
/// `SQLX_OFFLINE` is never overridden -- `SQLX_OFFLINE=false`, which
/// `cargo sqlx prepare` sets, makes this a no-op -- and `SQLX_QUERY_SOFT_WARN=1`
/// opts out of the default fail-fast behavior (see
/// `docs/feature/001_offline_mode_enforcement.md`).
#[ cfg( feature = "enabled" ) ]
pub fn ensure_offline_mode()
{
  ensure_offline_mode_with( &Options::from_env() );
}

/// Same as [`ensure_offline_mode`], with explicit [`Options`] instead of
/// reading them where sqlx reads them -- the escape hatch for consumers who
/// need a non-standard configuration.
#[ cfg( feature = "enabled" ) ]
pub fn ensure_offline_mode_with( opts : &Options )
{
  for directive in build_script_directives( opts )
  {
    println!( "{directive}" );
  }
}

/// The `cargo:` directives [`ensure_offline_mode_with`] prints for `opts`, in
/// order, without printing them. A dry run, except that `auto_regen`'s nested
/// `cargo check` does run: its outcome decides the remaining directives.
#[ cfg( feature = "enabled" ) ]
#[ must_use ]
pub fn build_script_directives( opts : &Options ) -> Vec< String >
{
  let dirs = cache_dir::resolve( &opts.manifest_dir, opts.cache_dir_override.as_deref() );
  let scan = scan::scan_package( &opts.manifest_dir, &dirs.lookup );

  // Fix(rerun_triggers_mre_cover_every_input): declare every input to Cargo.
  // Root cause: with no rerun-if directives, Cargo re-ran this build script only when a package
  // file changed -- never for an environment change, nor for a .sqlx it does not list (hidden
  // outside git, or at the workspace root) -- and replayed the previous run's output instead,
  // stale SQLX_OFFLINE decision included.
  // Pitfall: emitting any rerun-if directive replaces Cargo's package-wide default, so the
  // source tree has to be declared as well.
  let mut directives : Vec< String > = core::iter::once( opts.database_url_var.as_str() )
  .chain( TRACKED_ENV_VARS )
  .map( | var | format!( "cargo:rerun-if-env-changed={var}" ) )
  .collect();
  // The `.env` files and `sqlx.toml` the settings came from are inputs too; the walk skips the
  // hidden `.env`, and a `sqlx.toml` it does list is declared once.
  let mut inputs : Vec< &std::path::Path > = scan.watched.iter().chain( &opts.config_files ).map( std::path::PathBuf::as_path ).collect();
  inputs.sort();
  inputs.dedup();
  directives.extend( inputs.into_iter().map( rerun_if_changed ) );

  // Fix(explicit_offline_false_mre_is_never_overridden): stand aside when SQLX_OFFLINE is
  // explicitly false.
  // Root cause: `cargo:rustc-env=SQLX_OFFLINE=true` overrode the SQLX_OFFLINE=false that
  // `cargo sqlx prepare` sets, so prepare -- having already deleted the .sqlx entries -- failed
  // and left .sqlx empty, and `prepare --check` compiled from the old entries, checking nothing.
  // Pitfall: rustc-env beats the inherited environment. Cache directories stay unwatched here
  // too: a live build rewrites every cache file it compiles, so watching them would make each
  // build trigger the next.
  if opts.offline == Some( false )
  {
    return directives;
  }

  let stale = report::stale_sites( &scan.sites, &dirs.lookup );

  // Fix(missing_offline_dir_mre_says_sqlx_cannot_write): tell an explicit SQLX_OFFLINE_DIR that is
  // not a directory apart from a cache sqlx writes into.
  // Root cause: the summaries promised entries written into it, and auto_regen spent a nested
  // build on it, while sqlx failed every query it compiled live with "sqlx offline path does not
  // exist".
  // Pitfall: sqlx never creates the directory, and its offline lookup skips it silently -- only
  // live builds fail on it.
  let missing_dir = dirs.primary.as_deref().filter( | dir | !dir.is_dir() );
  // The warnings below say to create it only over stale sites. Declared while missing, it re-runs
  // the check on every build until it exists -- and each build compiling a stale query fails anyway.
  let named_missing_dir = missing_dir.filter( | _ | !stale.is_empty() && opts.offline != Some( true ) );
  directives.extend( dirs.lookup.iter().filter( | dir | dir.is_dir() || named_missing_dir == Some( dir.as_path() ) ).map( | dir | rerun_if_changed( dir ) ) );

  let Some( primary ) = dirs.primary.as_deref() else
  {
    directives.push( report::no_cache_dir_warning() );
    return directives;
  };

  #[ cfg( feature = "auto_regen" ) ]
  let stale = if stale.is_empty() || !opts.auto_regen || opts.offline == Some( true )
  {
    stale
  }
  else if let Some( dir ) = missing_dir
  {
    directives.push( regen::skipped_without_offline_dir( dir ) );
    stale
  }
  else if opts.database_url_set
  {
    let attempt = regen::attempt_regeneration( &opts.manifest_dir, primary );
    let remaining = report::stale_sites( &scan.sites, &dirs.lookup );
    directives.push( match attempt
    {
      Ok( () ) => regen::regenerated( stale.len(), remaining.len() ),
      Err( failure ) => failure,
    } );
    remaining
  }
  else
  {
    // Fix(auto_regen_without_database_url_mre_reports_the_skip): say why regeneration did not run.
    // Root cause: without a database URL the opt-in was skipped silently.
    // Pitfall: an opt-in that silently does nothing reads as a broken feature, not a missing input.
    directives.push( regen::skipped_without_database_url( &opts.database_url_var ) );
    stale
  };

  directives.extend( stale.iter().map( | site | report::site_warning( site ) ) );

  if opts.offline == Some( true )
  {
    if !stale.is_empty()
    {
      directives.push( report::offline_already_set_summary( stale.len() ) );
    }
    return directives;
  }

  // Fix(soft_warn_mre_needs_no_database_url_in_process_env): honor SQLX_QUERY_SOFT_WARN=1 alone.
  // Root cause: it also required DATABASE_URL in this process's environment, so the standard
  // setup -- DATABASE_URL in `.env`, which sqlx reads itself -- still forced SQLX_OFFLINE=true.
  // Pitfall: sqlx picks live or offline from its own view of the environment; with no database
  // it stays offline anyway, so not forcing is always safe here.
  if !stale.is_empty() && opts.soft_warn
  {
    // sqlx 0.9 writes a live result only into an explicit SQLX_OFFLINE_DIR -- an existing
    // `.sqlx` at the default location is not enough (empirically confirmed). One already set, in
    // the environment or a `.env` file, reaches sqlx without help.
    if opts.cache_dir_override.is_none()
    {
      directives.push( format!( "cargo:rustc-env=SQLX_OFFLINE_DIR={}", primary.display() ) );
    }
    directives.push( match missing_dir
    {
      None => report::soft_warn_summary( stale.len(), primary, &opts.database_url_var ),
      Some( dir ) => report::soft_warn_without_offline_dir_summary( dir, &opts.database_url_var ),
    } );
    return directives;
  }

  if !stale.is_empty()
  {
    directives.extend( report::forced_offline_summary( stale.len(), &opts.database_url_var, missing_dir ) );
  }
  directives.push( "cargo:rustc-env=SQLX_OFFLINE=true".to_string() );
  directives
}

#[ cfg( feature = "enabled" ) ]
fn rerun_if_changed( path : &std::path::Path ) -> String
{
  format!( "cargo:rerun-if-changed={}", path.display() )
}
