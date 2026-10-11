//! Helpers shared by this crate's integration tests; each test binary declares
//! `mod support;` and uses part of it.
//!
//! - [`PackageDir`]: a package on disk that `sqlx_query::build_script_directives`
//!   reads without building anything -- the fast tests.
//! - [`FixtureCrate`]: a real consumer crate (a `sqlx` dependency, a `build.rs`
//!   calling `sqlx_query::ensure_offline_mode`, a real `SQLite` database) driven by
//!   real `cargo build` invocations -- the `#[ignore]`d end-to-end tests.

#![ allow( dead_code ) ]

use core::time::Duration;
use std::io::Read;
use std::path::{ Path, PathBuf };
use std::process::{ Command, Output, Stdio };
use std::thread::JoinHandle;
use std::time::Instant;

use sha2::{ Digest, Sha256 };

/// Every variable `sqlx_query` reads. Fixture builds clear them all before
/// setting their own, so a developer's shell never leaks into a test.
const SQLX_QUERY_ENV_VARS : [ &str; 5 ] =
[
  "DATABASE_URL",
  "SQLX_OFFLINE",
  "SQLX_OFFLINE_DIR",
  "SQLX_QUERY_SOFT_WARN",
  "SQLX_QUERY_AUTO_REGEN",
];

/// Upper bound on one fixture `cargo build`. A cold build of sqlx's dependency
/// tree -- twice over with `auto_regen`'s nested `cargo check` -- takes minutes
/// while several end-to-end tests compile in parallel; a deadlock never returns
/// at all, so the bound only has to be finite.
const BUILD_BOUND : Duration = Duration::from_mins( 10 );

/// A package on disk -- a `Cargo.toml` that is its own workspace root, and a
/// `src/lib.rs` -- plus whatever a test adds to it.
#[ derive( Debug ) ]
pub struct PackageDir
{
  dir : tempfile::TempDir,
  root : PathBuf,
}

impl PackageDir
{
  /// A package named `fixture` whose `src/lib.rs` is `lib_rs`. Its manifest
  /// declares `[workspace]`, so Cargo never resolves it into an enclosing
  /// workspace.
  pub fn new( lib_rs : &str ) -> Self
  {
    let package = Self::empty();
    package.write( "Cargo.toml", &manifest( "fixture", "[workspace]\n" ) );
    package.write( "src/lib.rs", lib_rs );
    package
  }

  /// An empty directory, for layouts with several manifests: a workspace and
  /// its members, a workspace nested in another.
  pub fn empty() -> Self
  {
    let dir = tempfile::tempdir().expect( "create a temporary directory" );
    // Canonical, as Cargo reports paths: a symlinked temporary directory
    // (macOS's `/var`) would otherwise compare unequal to them.
    let root = dir.path().canonicalize().expect( "canonicalize the temporary directory" );
    Self { dir, root }
  }

  /// The root directory.
  pub fn root( &self ) -> &Path
  {
    &self.root
  }

  /// Write `content` to `relative`, creating its parent directories.
  pub fn write( &self, relative : &str, content : &str ) -> PathBuf
  {
    write_file( &self.root().join( relative ), content )
  }

  /// Create the directory `relative`, with its parents.
  pub fn mkdir( &self, relative : &str ) -> PathBuf
  {
    let path = self.root().join( relative );
    std::fs::create_dir_all( &path ).unwrap_or_else( | error | panic!( "create {}: {error}", path.display() ) );
    path
  }

  /// Write the entry sqlx itself writes for `sql` into the cache directory
  /// `cache_dir`, relative to the package root.
  pub fn cache_entry( &self, cache_dir : &str, sql : &str ) -> PathBuf
  {
    self.write( &format!( "{cache_dir}/{}", entry_name( sql ) ), "{}" )
  }

  /// `sqlx_query::build_script_directives` for this package, with `env` as the
  /// build script's entire environment.
  pub fn directives( &self, env : &[ ( &str, &str ) ] ) -> Vec< String >
  {
    directives_for( self.root(), env )
  }
}

/// `sqlx_query::build_script_directives` for the package at `manifest_dir`,
/// with `env` as the build script's entire environment.
pub fn directives_for( manifest_dir : &Path, env : &[ ( &str, &str ) ] ) -> Vec< String >
{
  sqlx_query::build_script_directives( &options_for( manifest_dir, env ) )
}

/// `sqlx_query::Options::resolve` for `manifest_dir`, with `env` as the entire
/// environment.
pub fn options_for( manifest_dir : &Path, env : &[ ( &str, &str ) ] ) -> sqlx_query::Options
{
  sqlx_query::Options::resolve( manifest_dir.to_path_buf(), | key |
  {
    env.iter().find( | ( name, _ ) | *name == key ).map( | ( _, value ) | ( *value ).to_string() )
  } )
}

/// A `Cargo.toml` for package `name` (version 0.1.0, edition 2021), followed by `extra`.
pub fn manifest( name : &str, extra : &str ) -> String
{
  format!( "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n{extra}" )
}

/// The lowercase hex SHA-256 digest of `bytes`, computed independently of
/// `sqlx_query`'s own hashing.
pub fn sha256_hex( bytes : &[ u8 ] ) -> String
{
  use core::fmt::Write as _;

  let mut hex = String::with_capacity( 64 );
  for byte in Sha256::digest( bytes )
  {
    write!( hex, "{byte:02x}" ).expect( "writing to a String never fails" );
  }
  hex
}

/// The file name sqlx gives `sql`'s cache entry.
pub fn entry_name( sql : &str ) -> String
{
  format!( "query-{}.json", sha256_hex( sql.as_bytes() ) )
}

/// The warning `sqlx_query` prints for a stale call site at `file:line`.
pub fn site_warning( file : &str, line : usize ) -> String
{
  format!( "cargo:warning=sqlx_query: {file}:{line} -- no .sqlx entry matches this query's SQL" )
}

/// Every `cargo:warning=` line among `directives`, in order.
pub fn warnings( directives : &[ String ] ) -> Vec< &str >
{
  directives.iter().filter( | line | line.starts_with( "cargo:warning=" ) ).map( String::as_str ).collect()
}

/// The stale call-site warnings among `directives`, in order.
pub fn site_warnings( directives : &[ String ] ) -> Vec< String >
{
  directives
  .iter()
  .filter( | line | line.ends_with( " -- no .sqlx entry matches this query's SQL" ) )
  .cloned()
  .collect()
}

/// The paths `directives` ask Cargo to watch (`cargo:rerun-if-changed=`).
pub fn watched_paths( directives : &[ String ] ) -> Vec< PathBuf >
{
  directives.iter().filter_map( | line | line.strip_prefix( "cargo:rerun-if-changed=" ) ).map( PathBuf::from ).collect()
}

/// The variables `directives` set for rustc (`cargo:rustc-env=`), as `NAME=value`.
pub fn rustc_env( directives : &[ String ] ) -> Vec< &str >
{
  directives.iter().filter_map( | line | line.strip_prefix( "cargo:rustc-env=" ) ).collect()
}

/// Assert the package's only query site, at `src/lib.rs:line`, is reported
/// stale while `.sqlx/` lacks `entry`, and fresh once `.sqlx/<entry>` exists --
/// proving both that the site is found and which cache entry it is keyed by.
pub fn assert_site_keyed_by( package : &PackageDir, line : usize, entry : &str )
{
  package.mkdir( ".sqlx" );
  let before = package.directives( &[] );
  assert_eq!( site_warnings( &before ), [ site_warning( "src/lib.rs", line ) ], "expected one stale site: {before:#?}" );

  package.write( &format!( ".sqlx/{entry}" ), "{}" );
  let after = package.directives( &[] );
  assert_eq!( site_warnings( &after ), Vec::< String >::new(), "{entry} should match the site: {after:#?}" );
}

/// A throwaway consumer crate: one `sqlx::query!` call site, a `build.rs`
/// calling `sqlx_query::ensure_offline_mode`, a real `SQLite` database with the
/// fixed `t( id )` schema, and an empty `.sqlx/` -- real consumers create it
/// once with `cargo sqlx prepare`; `ensure_offline_mode` never creates it.
///
/// Every build shares the fixture's one target directory, as a developer's
/// repeated `cargo build`s do: a build script Cargo does not re-run replays its
/// previous output, so a missing rerun trigger shows up in these tests.
#[ derive( Debug ) ]
pub struct FixtureCrate
{
  dir : tempfile::TempDir,
}

impl FixtureCrate
{
  /// A fixture whose call site runs `sql`.
  pub async fn new( sql : &str ) -> Self
  {
    let fixture = Self { dir : tempfile::tempdir().expect( "create a temporary directory" ) };
    create_database( &fixture.database_path() ).await;
    std::fs::create_dir_all( fixture.sqlx_dir() ).expect( "create an empty .sqlx/" );
    write_manifest( fixture.dir.path() );
    write_file( &fixture.dir.path().join( "build.rs" ), "fn main() { sqlx_query::ensure_offline_mode(); }\n" );
    fixture.set_query_sql( sql );
    fixture
  }

  /// `DATABASE_URL` for the fixture's `SQLite` database.
  pub fn database_url( &self ) -> String
  {
    format!( "sqlite://{}", self.database_path().display() )
  }

  /// The fixture's `.sqlx` cache directory.
  pub fn sqlx_dir( &self ) -> PathBuf
  {
    self.dir.path().join( ".sqlx" )
  }

  /// The `query-*.json` entries in `.sqlx/`, sorted.
  pub fn cache_entries( &self ) -> Vec< PathBuf >
  {
    let mut entries : Vec< PathBuf > = std::fs::read_dir( self.sqlx_dir() )
    .expect( "read .sqlx/" )
    .map( | entry | entry.expect( "read a .sqlx/ entry" ).path() )
    .filter( | path | path.file_name().is_some_and( | name | name.to_string_lossy().starts_with( "query-" ) ) )
    .collect();
    entries.sort();
    entries
  }

  /// Rewrite the call site to run `sql`, leaving the cache as it is. Passing
  /// the current SQL just touches the source, as `cargo sqlx prepare` does.
  pub fn set_query_sql( &self, sql : &str )
  {
    write_source( self.dir.path(), sql );
  }

  /// Write `content` to `relative` inside the fixture, creating its parent
  /// directories.
  pub fn write( &self, relative : &str, content : &str ) -> PathBuf
  {
    write_file( &self.dir.path().join( relative ), content )
  }

  /// Create the cache entry for the current SQL with a live build --
  /// `SQLX_QUERY_SOFT_WARN=1`, so the empty cache does not force offline mode.
  pub fn bootstrap_cache( &self )
  {
    let output = self.build( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
    assert!( output.status.success(), "bootstrap build failed: {}", String::from_utf8_lossy( &output.stderr ) );
    assert_eq!( self.cache_entries().len(), 1, "the bootstrap build should write exactly one cache entry" );
  }

  /// `cargo build` the fixture with `DATABASE_URL` pointing at its database,
  /// plus `env`, and no other variable `sqlx_query` reads. A build outliving
  /// [`BUILD_BOUND`] -- a reintroduced nested-build deadlock -- is killed and
  /// fails the test instead of stalling the suite.
  ///
  /// `PATH` is inherited on purpose: Cargo needs the toolchain and the C
  /// compiler that builds the bundled `SQLite`, and nothing a fixture build runs
  /// falls back to an interactive program.
  pub fn build( &self, env : &[ ( &str, &str ) ] ) -> Output
  {
    let mut command = Command::new( std::env::var_os( "CARGO" ).unwrap_or_else( || "cargo".into() ) );
    command
    .arg( "build" )
    .arg( "--manifest-path" ).arg( self.dir.path().join( "Cargo.toml" ) )
    .arg( "--target-dir" ).arg( self.dir.path().join( "target" ) )
    .stdout( Stdio::piped() )
    .stderr( Stdio::piped() );
    for var in SQLX_QUERY_ENV_VARS
    {
      command.env_remove( var );
    }
    command.env( "DATABASE_URL", self.database_url() );
    command.envs( env.iter().copied() );
    run_bounded( command )
  }

  fn database_path( &self ) -> PathBuf
  {
    self.dir.path().join( "fixture.sqlite" )
  }
}

/// Run `command` to completion within [`BUILD_BOUND`]. Its output is drained
/// while it runs -- a child blocked on a full pipe never exits, which would
/// read as a deadlock.
fn run_bounded( mut command : Command ) -> Output
{
  let mut child = command.spawn().expect( "spawn cargo" );
  let stdout = drain( child.stdout.take() );
  let stderr = drain( child.stderr.take() );
  let started = Instant::now();

  let status = loop
  {
    if let Some( status ) = child.try_wait().expect( "poll cargo" )
    {
      break status;
    }
    if started.elapsed() > BUILD_BOUND
    {
      let _ = child.kill();
      panic!( "cargo build outlived {BUILD_BOUND:?} -- a nested-build deadlock?" );
    }
    std::thread::sleep( Duration::from_millis( 200 ) );
  };

  Output
  {
    status,
    stdout : stdout.join().expect( "join the stdout reader" ),
    stderr : stderr.join().expect( "join the stderr reader" ),
  }
}

fn drain< R : Read + Send + 'static >( pipe : Option< R > ) -> JoinHandle< Vec< u8 > >
{
  std::thread::spawn( move ||
  {
    let mut buffer = Vec::new();
    if let Some( mut pipe ) = pipe
    {
      pipe.read_to_end( &mut buffer ).expect( "read cargo's output" );
    }
    buffer
  } )
}

async fn create_database( db_path : &Path )
{
  use sqlx::sqlite::SqlitePoolOptions;

  let url = format!( "sqlite://{}?mode=rwc", db_path.display() );
  let pool = SqlitePoolOptions::new()
  .connect( &url )
  .await
  .expect( "failed to create fixture SQLite database" );

  sqlx::query( "CREATE TABLE t ( id INTEGER PRIMARY KEY )" )
  .execute( &pool )
  .await
  .expect( "failed to create fixture schema" );

  pool.close().await;
}

fn write_manifest( dir : &Path )
{
  let sqlx_query_dir = env!( "CARGO_MANIFEST_DIR" );
  write_file( &dir.join( "Cargo.toml" ), &format!(
r#"[package]
name = "sqlx_query_fixture"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
sqlx = {{ version = "0.9", default-features = false, features = [ "sqlite", "runtime-tokio", "tls-rustls-ring", "macros" ] }}

[build-dependencies]
sqlx_query = {{ path = {sqlx_query_dir:?}, features = [ "auto_regen" ] }}
"#
  ) );
}

fn write_source( dir : &Path, sql : &str )
{
  write_file( &dir.join( "src" ).join( "lib.rs" ), &format!(
r"pub async fn run( pool : &sqlx::SqlitePool ) -> Result< (), sqlx::Error >
{{
  let row = sqlx::query!( {sql:?} ).fetch_one( pool ).await?;
  let _ = row;
  Ok( () )
}}
"
  ) );
}

/// Write `content` to `path`, creating its parent directories.
pub fn write_file( path : &Path, content : &str ) -> PathBuf
{
  if let Some( parent ) = path.parent()
  {
    std::fs::create_dir_all( parent ).unwrap_or_else( | error | panic!( "create {}: {error}", parent.display() ) );
  }
  std::fs::write( path, content ).unwrap_or_else( | error | panic!( "write {}: {error}", path.display() ) );
  path.to_path_buf()
}
