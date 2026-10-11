use std::path::{ Path, PathBuf };

/// The variable sqlx reads the database URL from, unless the crate's
/// `sqlx.toml` names another.
const DEFAULT_DATABASE_URL_VAR : &str = "DATABASE_URL";

/// Runtime configuration for [`crate::ensure_offline_mode_with`].
///
/// [`Options::from_env`] is what [`crate::ensure_offline_mode`] uses;
/// [`Options::resolve`] builds the same thing from any variable lookup, and
/// the fields can be adjusted afterwards to override individual settings.
#[ derive( Debug, Clone ) ]
pub struct Options
{
  /// Directory of the consumer crate's `Cargo.toml` (`CARGO_MANIFEST_DIR`).
  pub manifest_dir : PathBuf,
  /// `SQLX_OFFLINE_DIR`: the cache directory sqlx searches first and writes
  /// live results into. A relative path is resolved against the workspace
  /// root, as sqlx itself resolves it.
  pub cache_dir_override : Option< PathBuf >,
  /// An explicit `SQLX_OFFLINE` value, parsed the way sqlx parses it (`true`
  /// in any case, or `1`, means offline; any other value -- empty included --
  /// means online); `None` when unset. Never overridden: `Some( false )`,
  /// which `cargo sqlx prepare` sets, makes `sqlx_query` stand aside entirely.
  pub offline : Option< bool >,
  /// The variable sqlx reads the database URL from: `DATABASE_URL`, unless the
  /// crate's `sqlx.toml` sets `[common] database-url-var`.
  pub database_url_var : String,
  /// Whether that variable is set to a non-empty value -- sqlx's own
  /// condition for compiling queries against a live database.
  pub database_url_set : bool,
  /// Attempt a real, isolated regeneration of stale entries via a nested
  /// `cargo check`. Only takes effect with a database URL and the
  /// `auto_regen` feature; ignored otherwise.
  pub auto_regen : bool,
  /// Escape hatch for stale entries: skip forcing `SQLX_OFFLINE=true` so a
  /// build with a database URL compiles live instead, letting sqlx's own
  /// macro rewrite the `.sqlx` entries. Controlled by `SQLX_QUERY_SOFT_WARN`.
  pub soft_warn : bool,
  /// The `.env` files and the `sqlx.toml` these settings were read from, each
  /// declared to Cargo so that editing it re-runs the check.
  pub config_files : Vec< PathBuf >,
}

impl Default for Options
{
  /// Every setting empty or off, the database URL in `DATABASE_URL`.
  fn default() -> Self
  {
    Self
    {
      manifest_dir : PathBuf::new(),
      cache_dir_override : None,
      offline : None,
      database_url_var : DEFAULT_DATABASE_URL_VAR.to_string(),
      database_url_set : false,
      auto_regen : false,
      soft_warn : false,
      config_files : Vec::new(),
    }
  }
}

impl Options
{
  /// Build options from this build script's environment: `CARGO_MANIFEST_DIR`
  /// (set by Cargo for every `build.rs`) plus the variables and files
  /// [`Options::resolve`] reads.
  #[ must_use ]
  pub fn from_env() -> Self
  {
    let manifest_dir = std::env::var_os( "CARGO_MANIFEST_DIR" ).map_or_else( || PathBuf::from( "." ), PathBuf::from );
    Self::resolve( manifest_dir, | key | std::env::var( key ).ok() )
  }

  /// Build options for the crate at `manifest_dir` as sqlx's macros would see
  /// them, looking variables up through `env`, then in every `.env` file from
  /// `manifest_dir` up to the filesystem root:
  /// - `SQLX_OFFLINE_DIR` -- [`Options::cache_dir_override`]
  /// - `SQLX_OFFLINE` -- [`Options::offline`]
  /// - `DATABASE_URL`, or the variable the crate's `sqlx.toml` names --
  ///   [`Options::database_url_var`], [`Options::database_url_set`]
  ///
  /// As in sqlx, a variable `env` holds -- even an empty one -- beats every
  /// `.env` file, and among those the one nearest the filesystem root wins.
  /// The crate's own switches are read from `env` alone, never from `.env`:
  /// - `SQLX_QUERY_AUTO_REGEN=1` -- [`Options::auto_regen`]
  /// - `SQLX_QUERY_SOFT_WARN=1` -- [`Options::soft_warn`]
  #[ must_use ]
  pub fn resolve( manifest_dir : PathBuf, env : impl Fn( &str ) -> Option< String > ) -> Self
  {
    let mut config_files = Vec::new();
    let database_url_var = database_url_var( &manifest_dir, &mut config_files );
    let dotenv = Dotenv::load( &manifest_dir, &database_url_var, &mut config_files );

    // Fix(empty_database_url_mre_counts_as_unset): require a non-empty DATABASE_URL.
    // Root cause: the variable's presence alone counted as a configured database.
    // Pitfall: sqlx treats `DATABASE_URL=''` as unset and stays offline.
    let database_url_set = env( &database_url_var ).or( dotenv.database_url ).is_some_and( | url | !url.is_empty() );
    Self
    {
      cache_dir_override : env( "SQLX_OFFLINE_DIR" ).or( dotenv.offline_dir ).map( PathBuf::from ),
      offline : env( "SQLX_OFFLINE" ).or( dotenv.offline ).map( | value | is_truthy( &value ) ),
      database_url_var,
      database_url_set,
      // Never from `.env`: the nested regeneration check removes SQLX_QUERY_AUTO_REGEN from its
      // environment as a recursion guard, which a `.env` value would defeat.
      auto_regen : env( "SQLX_QUERY_AUTO_REGEN" ).is_some_and( | value | value == "1" ),
      soft_warn : env( "SQLX_QUERY_SOFT_WARN" ).is_some_and( | value | value == "1" ),
      config_files,
      manifest_dir,
    }
  }
}

/// The settings sqlx takes from `.env` files, unparsed.
#[ derive( Default ) ]
struct Dotenv
{
  database_url : Option< String >,
  offline : Option< String >,
  offline_dir : Option< String >,
}

impl Dotenv
{
  /// Read every `.env` file from `manifest_dir` up to the filesystem root, as
  /// sqlx 0.9 does (`load_env` in `sqlx-macros-core`), recording each one in
  /// `files`.
  fn load( manifest_dir : &Path, database_url_var : &str, files : &mut Vec< PathBuf > ) -> Self
  {
    // Fix(dotenv_mre_supplies_what_sqlx_reads_from_it): read `.env` files the way sqlx does.
    // Root cause: only the process environment was read, so settings kept in `.env` -- sqlx's
    // standard setup -- left auto_regen skipped, SQLX_OFFLINE=false overridden, and soft-warn
    // entries written into a directory other than the one `.env` named.
    // Pitfall: sqlx reads every ancestor's `.env` and keeps the last value, so the outermost file
    // wins, while a process variable, even an empty one, beats them all.
    let mut values = Self::default();
    for dir in manifest_dir.ancestors()
    {
      let path = dir.join( ".env" );
      if !path.is_file()
      {
        continue;
      }
      // A file dotenvy cannot parse fails every sqlx query macro; the lines before the error are kept.
      if let Ok( entries ) = dotenvy::from_path_iter( &path )
      {
        for ( name, value ) in entries.map_while( Result::ok )
        {
          match name.as_str()
          {
            "SQLX_OFFLINE_DIR" => values.offline_dir = Some( value ),
            "SQLX_OFFLINE" => values.offline = Some( value ),
            _ if name == database_url_var => values.database_url = Some( value ),
            _ => {}
          }
        }
      }
      files.push( path );
    }
    values
  }
}

/// The variable `[common] database-url-var` names in the crate's own
/// `sqlx.toml`, or `DATABASE_URL`. An existing `sqlx.toml` is recorded in
/// `files`; one sqlx cannot parse fails every query macro anyway.
fn database_url_var( manifest_dir : &Path, files : &mut Vec< PathBuf > ) -> String
{
  // Fix(database_url_var_mre_follows_sqlx_toml): read the variable's name from sqlx.toml.
  // Root cause: DATABASE_URL was hard-coded, so with `database-url-var` set, the auto_regen gate,
  // the rerun trigger and the advice all followed a variable sqlx ignores.
  // Pitfall: sqlx reads only the crate's own sqlx.toml, never an ancestor's or the workspace root's.
  let path = manifest_dir.join( "sqlx.toml" );
  if !path.is_file()
  {
    return DEFAULT_DATABASE_URL_VAR.to_string();
  }
  let configured = std::fs::read_to_string( &path )
  .ok()
  .and_then( | text | text.parse::< toml_edit::DocumentMut >().ok() )
  .and_then( | document | document.get( "common" )?.get( "database-url-var" )?.as_str().map( str::to_string ) );
  files.push( path );
  configured.unwrap_or_else( || DEFAULT_DATABASE_URL_VAR.to_string() )
}

/// sqlx's own `SQLX_OFFLINE` parsing (`is_truthy_bool` in `sqlx-macros-core`).
fn is_truthy( value : &str ) -> bool
{
  value.eq_ignore_ascii_case( "true" ) || value == "1"
}
