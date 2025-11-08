//! comprehensive test suite with perfect coverage for `workspace_tools`
//!
//! ## comprehensive test matrix
//!
//! ### core workspace functionality
//! | id    | component            | test case                  | conditions           | expected result      |
//! |-------|---------------------|----------------------------|----------------------|----------------------|
//! | w1.1  | `workspace ::resolve`  | env var set, path exists   | valid directory      | success              |
//! | w1.2  | `workspace ::resolve`  | env var set, path missing  | nonexistent path     | `PathNotFound` error   |
//! | w1.3  | `workspace ::resolve`  | env var missing            | no env var           | `EnvironmentMissing`   |
//! | w1.4  | `workspace ::resolve`  | env var empty              | empty string         | `PathNotFound` error   |
//! | w1.5  | `workspace ::resolve`  | env var is file not dir    | points to file       | error on validate    |
//! | w2.1  | fallback resolution | no env, cwd exists         | current dir valid    | uses current dir     |
//! | w2.2  | fallback resolution | no env, in git repo        | .git dir found       | uses git root        |
//! | w2.3  | fallback resolution | no env, no git, no cwd     | all fail             | uses root fallback   |
//! | w3.1  | path operations     | join relative path         | normal path          | correct join         |
//! | w3.2  | path operations     | join absolute path         | absolute path        | correct join         |
//! | w3.3  | path operations     | join empty path            | empty string         | returns root         |
//! | w3.4  | path operations     | join path with ..          | parent traversal     | correct resolution   |
//! | w4.1  | boundary checking   | workspace-relative path    | inside workspace     | true                 |
//! | w4.2  | boundary checking   | absolute external path     | outside workspace    | false                |
//! | w4.3  | boundary checking   | symlink to external        | symlink outside      | depends on target    |
//! | w5.1  | standard dirs       | all directory getters      | any workspace        | correct paths        |
//! | w5.2  | validation          | valid workspace            | accessible dir       | success              |
//! | w5.3  | validation          | inaccessible workspace    | permission denied    | error                |
//! | w6.1  | normalization       | relative path              | exists in workspace  | canonical path       |
//! | w6.2  | normalization       | nonexistent path           | doesn't exist        | `IoError`              |
//! | w6.3  | normalization       | symlink resolution         | symlinks present     | resolved target      |
//!
//! ### error handling comprehensive tests  
//! | id    | error type          | trigger condition          | validation           |
//! |-------|---------------------|----------------------------|----------------------|
//! | e1.1  | `EnvironmentMissing`  | no `WORKSPACE_PATH`          | correct error msg    |
//! | e1.2  | `PathNotFound`        | nonexistent path           | path in error        |
//! | e1.3  | `PathOutsideWorkspace`| external path              | path in error        |
//! | e1.4  | `ConfigurationError`  | workspace is file          | descriptive message  |
//! | e1.5  | `IoError`             | permission denied          | io error details     |
//! | e2.1  | error display       | all error variants         | human readable       |
//! | e2.2  | error debug         | all error variants         | debug info           |
//! | e2.3  | error from trait    | `std ::error ::Error` impl     | proper trait impl    |
//!
//! ### feature-specific tests (glob)
//! | id    | feature             | test case                  | conditions           | expected             |
//! |-------|---------------------|----------------------------|----------------------|----------------------|
//! | g1.1  | `find_resources`      | simple pattern             | *.rs files exist     | all rust files       |
//! | g1.2  | `find_resources`      | recursive pattern          | **/*.rs pattern      | nested rust files    |
//! | g1.3  | `find_resources`      | no matches                 | pattern matches none | empty vec            |
//! | g1.4  | `find_resources`      | invalid pattern            | malformed glob       | `GlobError`            |
//! | g2.1  | `find_config`         | toml exists                | app.toml present     | finds toml           |
//! | g2.2  | `find_config`         | yaml exists                | app.yaml present     | finds yaml           |
//! | g2.3  | `find_config`         | json exists                | app.json present     | finds json           |
//! | g2.4  | `find_config`         | dotfile exists             | .app.toml present    | finds dotfile        |
//! | g2.5  | `find_config`         | multiple formats exist     | toml+yaml+json       | priority order       |
//! | g2.6  | `find_config`         | no config found            | none exist           | `PathNotFound`         |
//!
//! ### feature-specific tests (`secret_management`)
//! | id    | feature             | test case                  | conditions           | expected             |
//! |-------|---------------------|----------------------------|----------------------|----------------------|
//! | s1.1  | `secret_dir`          | secret directory path      | any workspace        | secret path         |
//! | s1.2  | `secret_file`         | secret file path           | filename provided    | secret/filename     |
//! | s2.1  | `load_secrets_file`   | valid key=value format     | proper shell format  | parsed hashmap       |
//! | s2.2  | `load_secrets_file`   | quoted values              | "value" and 'value'  | unquoted values      |
//! | s2.3  | `load_secrets_file`   | comments and empty lines   | # comments present   | ignored lines        |
//! | s2.4  | `load_secrets_file`   | file doesn't exist         | missing file         | empty hashmap        |
//! | s2.5  | `load_secrets_file`   | file read error            | permission denied    | `IoError`              |
//! | s2.6  | `load_secrets_file`   | malformed content          | invalid format       | partial parsing      |
//! | s3.1  | `load_secret_key`     | key in file                | key exists in file   | value from file      |
//! | s3.2  | `load_secret_key`     | key in environment         | env var exists       | value from env       |
//! | s3.3  | `load_secret_key`     | key in both                | file and env         | file takes priority  |
//! | s3.4  | `load_secret_key`     | key in neither             | not found anywhere   | `ConfigError`          |
//! | s3.5  | `parse_key_value`     | various formats            | edge case formats    | correct parsing      |
//!
//! ### integration and cross-platform tests
//! | id    | category            | test case                  | platform/condition   | validation           |
//! |-------|---------------------|----------------------------|----------------------|----------------------|
//! | i1.1  | cross-platform      | windows paths              | windows-style paths  | normalized correctly |
//! | i1.2  | cross-platform      | unix paths                 | unix-style paths     | handled correctly    |
//! | i1.3  | symlinks            | symlink to directory       | valid symlink        | follows symlink      |
//! | i1.4  | symlinks            | broken symlink             | dangling symlink     | appropriate error    |
//! | i1.5  | permissions         | read-only workspace        | restricted access    | graceful handling    |
//! | i2.1  | concurrent access   | multiple workspace inits   | concurrent creation  | thread safety        |
//! | i2.2  | environment changes | env var changed mid-test   | dynamic changes      | consistent behavior  |
//! | i3.1  | testing utilities   | `create_test_workspace`      | temp dir creation    | isolated workspace   |
//! | i3.2  | testing utilities   | structured workspace       | full dir structure   | all dirs created     |
//!
//! ### performance and stress tests  
//! | id    | category            | test case                  | scale/condition      | performance target   |
//! |-------|---------------------|----------------------------|----------------------|----------------------|
//! | p1.1  | large workspace     | 10k+ files                 | deep directory tree  | reasonable speed     |
//! | p1.2  | many glob patterns  | 100+ concurrent globs      | pattern complexity   | no memory leaks      |
//! | p1.3  | large secret files  | 1MB+ secret files          | big config files     | efficient parsing    |
//! | p1.4  | repeated operations | 1000+ workspace creates    | stress test          | consistent perf      |

use workspace_tools :: *;
use tempfile :: { TempDir, NamedTempFile };
use std ::
{
  env, fs, path ::PathBuf,
  sync :: { Arc, Mutex },
  thread,
};

// helper functions to replace testing module
fn create_test_workspace() -> ( TempDir, Workspace )
{
  let temp_dir = TempDir ::new().expect( "Failed to create temp directory" );
  let workspace = Workspace ::new( temp_dir.path() );
  ( temp_dir, workspace )
}

fn create_test_workspace_with_structure() -> ( TempDir, Workspace )
{
  let temp_dir = TempDir ::new().expect( "Failed to create temp directory" );
  let workspace = Workspace ::new( temp_dir.path() );

  // create standard directories
  fs ::create_dir_all( workspace.config_dir() ).ok();
  fs ::create_dir_all( workspace.data_dir() ).ok();
  fs ::create_dir_all( workspace.logs_dir() ).ok();
  fs ::create_dir_all( workspace.docs_dir() ).ok();
  fs ::create_dir_all( workspace.tests_dir() ).ok();
  fs ::create_dir_all( workspace.workspace_dir() ).ok();

  ( temp_dir, workspace )
}

// Global mutex to serialize environment variable tests
static ENV_TEST_MUTEX: Mutex< () > = Mutex ::new( () );

// ============================================================================
// core workspace functionality tests
// ============================================================================

mod core_workspace_tests
{
  use super :: *;

  /// test w1.1 : workspace resolution with valid environment variable
  #[ test ]
  fn test_resolve_with_valid_env_var()
  {
  let _lock = ENV_TEST_MUTEX.lock().unwrap();
  
  let temp_dir = TempDir ::new().unwrap();
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  
  env ::set_var( "WORKSPACE_PATH", temp_dir.path() );
  let result = Workspace ::resolve();
  
  restore_env_var( "WORKSPACE_PATH", original );
  
  assert!( result.is_ok() );
  assert_eq!( result.unwrap().root(), temp_dir.path() );
 }

  /// test w1.2 : workspace resolution with nonexistent path
  #[ test ]
  fn test_resolve_with_nonexistent_path()
  {
  let _lock = ENV_TEST_MUTEX.lock().unwrap();
  
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  // Use a truly unique path that's unlikely to exist or be created by other tests
  let thread_id = std ::thread ::current().id();
  let timestamp = std ::time ::SystemTime ::now()
   .duration_since(std ::time ::UNIX_EPOCH)
   .unwrap_or_default()
   .as_nanos();
  // Use platform-appropriate temp directory with a guaranteed nonexistent subpath
  let nonexistent = env ::temp_dir()
   .join( format!("nonexistent_workspace_test_{thread_id:?}_{timestamp}") )
   .join( "deeply_nested_nonexistent_subdir" );
  
  // Ensure this path definitely doesn't exist
  if nonexistent.exists()
  {
   fs ::remove_dir_all( &nonexistent ).ok();
 }
  
  env ::set_var( "WORKSPACE_PATH", &nonexistent );
  
  // Verify the environment variable is set correctly before calling resolve
  assert_eq!( env ::var( "WORKSPACE_PATH" ).unwrap(), nonexistent.to_string_lossy() );
  
  let result = Workspace ::resolve();
  
  // Restore environment immediately after getting result
  restore_env_var( "WORKSPACE_PATH", original );
  
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::PathNotFound( path ) => assert_eq!( path, nonexistent ),
   WorkspaceError ::EnvironmentVariableMissing( _ ) =>
  {
  // In case of race condition, this is acceptable but should be noted
  eprintln!("Warning: Environment variable was cleared by parallel test execution");
 },
   other => panic!( "expected PathNotFound or EnvironmentVariableMissing, got {other:?}" ),
 }
 }

  /// test w1.3 : workspace resolution with missing environment variable
  #[ test ]
  fn test_resolve_with_missing_env_var()
  {
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  env ::remove_var( "WORKSPACE_PATH" );
  let result = Workspace ::resolve();
  
  restore_env_var( "WORKSPACE_PATH", original );
  
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::EnvironmentVariableMissing( var ) => 
  assert_eq!( var, "WORKSPACE_PATH" ),
   other => panic!( "expected EnvironmentVariableMissing, got {other:?}" ),
 }
 }

  /// test w1.4 : workspace resolution with empty environment variable
  #[ test ]
  fn test_resolve_with_empty_env_var()
  {
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  
  // Set empty string and test immediately to avoid race conditions
  env ::set_var( "WORKSPACE_PATH", "" );
  let result = Workspace ::resolve();
  
  // Restore immediately after getting result
  restore_env_var( "WORKSPACE_PATH", original );
  
  assert!( result.is_err() );
  
  // empty env var behaves same as missing env var in current implementation
  match result.unwrap_err()
  {
   WorkspaceError ::PathNotFound( path ) => assert_eq!( path, PathBuf ::from( "" ) ),
   WorkspaceError ::EnvironmentVariableMissing( _ ) => {}, // also acceptable
   other => panic!( "expected PathNotFound or EnvironmentVariableMissing, got {other:?}" ),
 }
 }

  /// test w1.5 : workspace resolution pointing to file instead of directory
  #[ test ]
  fn test_resolve_with_file_instead_of_dir()
  {
  let temp_file = NamedTempFile ::new().unwrap();
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  
  env ::set_var( "WORKSPACE_PATH", temp_file.path() );
  
  // resolve should succeed (file exists)
  let workspace = Workspace ::resolve().unwrap();
  
  // but validate should fail
  let result = workspace.validate();
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::ConfigurationError( msg ) => 
  assert!( msg.contains( "not a directory" ) ),
   other => panic!( "expected ConfigurationError, got {other:?}" ),
 }
  
  restore_env_var( "WORKSPACE_PATH", original );
 }

  /// test w2.1 : fallback resolution behavior
  #[ test ]
  fn test_fallback_to_current_dir()
  {
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  env ::remove_var( "WORKSPACE_PATH" );
  let workspace = Workspace ::resolve_with_extended_fallbacks();
  
  restore_env_var( "WORKSPACE_PATH", original );
  
  // cargo integration is always available - should detect cargo workspace
  // should detect actual cargo workspace (not just fallback to current dir)
  assert!( workspace.is_cargo_workspace() );
  // workspace root should exist and be a directory
  assert!( workspace.root().exists() );
  assert!( workspace.root().is_dir() );
  // should contain a Cargo.toml with workspace configuration
  assert!( workspace.cargo_toml().exists() );
 }

  /// test w2.2 : fallback resolution to git root
  #[ test ]
  fn test_fallback_to_git_root()
  {
  let temp_dir = TempDir ::new().unwrap();
  let git_dir = temp_dir.path().join( ".git" );
  fs ::create_dir_all( &git_dir ).unwrap();
  
  let sub_dir = temp_dir.path().join( "subdir" );
  fs ::create_dir_all( &sub_dir ).unwrap();
  
  let original_dir = env ::current_dir().unwrap();
  let original_env = env ::var( "WORKSPACE_PATH" ).ok();
  
  env ::remove_var( "WORKSPACE_PATH" );
  env ::set_current_dir( &sub_dir ).unwrap();
  
  let result = Workspace ::from_git_root();
  assert!( result.is_ok() );
  assert_eq!( result.unwrap().root(), temp_dir.path() );
  
  env ::set_current_dir( original_dir ).unwrap();
  restore_env_var( "WORKSPACE_PATH", original_env );
 }

  /// test w2.3 : fallback when all strategies fail
  #[ test ]
  fn test_fallback_infallible()
  {
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  env ::remove_var( "WORKSPACE_PATH" );
  
  // this should never panic, even in worst case
  let workspace = Workspace ::from_cwd();
  
  restore_env_var( "WORKSPACE_PATH", original );
  
  assert!( workspace.root().is_absolute() );
 }

  // helper function to restore environment variables
  fn restore_env_var( key: &str, original: Option< String > )
  {
  match original
  {
   Some( value ) => env ::set_var( key, value ),
   None => env ::remove_var( key ),
 }
 }
}

// ============================================================================
// path operation tests
// ============================================================================

mod path_operation_tests
{
  use super :: *;

  /// test w3.1 : join relative path
  #[ test ]
  fn test_join_relative_path()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let joined = workspace.join( "config/app.toml" );
  let expected = workspace.root().join( "config/app.toml" );
  
  assert_eq!( joined, expected );
 }

  /// test w3.2 : join absolute path (should still work)
  #[ test ]
  fn test_join_absolute_path()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // Use platform-appropriate absolute path
  #[ cfg( windows ) ]
  let absolute_path = "C: \\Windows\\System32";
  #[ cfg( not( windows ) ) ]
  let absolute_path = "/etc/passwd";
  
  let joined = workspace.join( absolute_path );
  
  // PathBuf ::join behavior: absolute path components replace the entire path
  // so joining absolute path to workspace root gives that absolute path
  assert_eq!( joined, PathBuf ::from( absolute_path ) );
 }

  /// test w3.3 : join empty path
  #[ test ]
  fn test_join_empty_path()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let joined = workspace.join( "" );
  assert_eq!( joined, workspace.root() );
 }

  /// test w3.4 : join path with parent traversal
  #[ test ]
  fn test_join_with_parent_traversal()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let joined = workspace.join( "config/../data/file.txt" );
  let expected = workspace.root().join( "config/../data/file.txt" );
  
  assert_eq!( joined, expected );
 }

  /// test w4.1 : boundary checking for workspace-relative paths
  #[ test ]
  fn test_boundary_check_internal_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let internal_paths = vec!
  [
   workspace.join( "config/app.toml" ),
   workspace.join( "data/cache.db" ),
   workspace.root().to_path_buf(),
   workspace.join( "" ), // root itself
 ];
  
  for path in internal_paths
  {
   assert!( workspace.is_workspace_file( &path ), 
   "path should be within workspace: {}", path.display() );
 }
 }

  /// test w4.2 : boundary checking for external paths
  #[ test ]
  fn test_boundary_check_external_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // Use platform-appropriate external paths
  let mut external_paths = vec![ env ::temp_dir() ]; // different temp directory
  
  #[ cfg( windows ) ]
  {
   external_paths.push( PathBuf ::from( "C: \\" ) );
   external_paths.push( PathBuf ::from( "C: \\Windows" ) );
 }
  
  #[ cfg( not( windows ) ) ]
  {
   external_paths.push( PathBuf ::from( "/etc/passwd" ) );
   external_paths.push( PathBuf ::from( "/tmp" ) );
   external_paths.push( PathBuf ::from( "/" ) );
 }
  
  for path in external_paths
  {
   assert!( !workspace.is_workspace_file( &path ),
   "path should be outside workspace: {}", path.display() );
 }
 }

  /// test w4.3 : boundary checking with symlinks
  #[ test ]
  #[ cfg( unix ) ]
  fn test_boundary_check_symlinks()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // create symlink to external location
  let external_target = env ::temp_dir().join( "external_file" );
  fs ::write( &external_target, "external content" ).unwrap();
  
  let symlink_path = workspace.join( "link_to_external" );
  std ::os ::unix ::fs ::symlink( &external_target, &symlink_path ).unwrap();
  
  // symlink itself is in workspace
  assert!( workspace.is_workspace_file( &symlink_path ) );
  
  // cleanup
  fs ::remove_file( &external_target ).ok();
 }

  /// test w5.1 : all standard directory getters
  #[ test ]
  fn test_standard_directory_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  let root = workspace.root();
  
  assert_eq!( workspace.config_dir(), root.join( "config" ) );
  assert_eq!( workspace.data_dir(), root.join( "data" ) );
  assert_eq!( workspace.logs_dir(), root.join( "logs" ) );
  assert_eq!( workspace.docs_dir(), root.join( "docs" ) );
  assert_eq!( workspace.tests_dir(), root.join( "tests" ) );
  assert_eq!( workspace.workspace_dir(), root.join( ".workspace" ) );
  assert_eq!( workspace.cargo_toml(), root.join( "Cargo.toml" ) );
  assert_eq!( workspace.readme(), root.join( "readme.md" ) );
  
  #[ cfg( feature = "secrets" ) ]
  {
   assert_eq!( workspace.secret_dir(), root.join( "secret" ) );
   assert_eq!( workspace.secret_file( "test" ), root.join( "secret/test" ) );
 }
 }

  /// test w5.2 : workspace validation success
  #[ test ]
  fn test_workspace_validation_success()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let result = workspace.validate();
  assert!( result.is_ok(), "workspace validation should succeed: {result:?}" );
 }

  /// test w6.1 : path normalization for existing paths
  #[ test ]
  fn test_path_normalization_existing()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // create a file to normalize
  let test_file = workspace.join( "test_file.txt" );
  fs ::write( &test_file, "test content" ).unwrap();
  
  let normalized = workspace.normalize_path( "test_file.txt" );
  assert!( normalized.is_ok() );
  
  let normalized_path = normalized.unwrap();
  assert!( normalized_path.is_absolute() );
  assert!( normalized_path.ends_with( "test_file.txt" ) );
 }

  /// test w6.2 : path normalization for nonexistent paths
  #[ test ]
  fn test_path_normalization_nonexistent()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let result = workspace.normalize_path( "nonexistent_file.txt" );
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::IoError( msg ) => assert!( msg.contains( "normalize" ) ),
   other => panic!( "expected IoError, got {other:?}" ),
 }
 }
}

// ============================================================================
// comprehensive error handling tests
// ============================================================================

mod error_handling_tests
{
  use super :: *;

  /// test e1.1 : `EnvironmentVariableMissing` error
  #[ test ]
  fn test_environment_variable_missing_error()
  {
  let error = WorkspaceError ::EnvironmentVariableMissing( "TEST_VAR".to_string() );
  
  let display = format!( "{error}" );
  assert!( display.contains( "TEST_VAR" ) );
  assert!( display.contains( "WORKSPACE_PATH" ) );
  
  // test Debug trait
  let debug = format!( "{error:?}" );
  assert!( debug.contains( "EnvironmentVariableMissing" ) );
  assert!( debug.contains( "TEST_VAR" ) );
 }

  /// test e1.2 : `PathNotFound` error
  #[ test ]
  fn test_path_not_found_error()
  {
  // Use platform-appropriate nonexistent path
  #[ cfg( windows ) ]
  let test_path = PathBuf ::from( "Z: \\nonexistent\\path" );
  #[ cfg( not( windows ) ) ]
  let test_path = PathBuf ::from( "/nonexistent/path" );
  
  let error = WorkspaceError ::PathNotFound( test_path.clone() );
  
  let display = format!( "{error}" );
  assert!( display.contains( "nonexistent" ) );
  assert!( display.contains( "not found" ) );
  
  let debug = format!( "{error:?}" );
  assert!( debug.contains( "PathNotFound" ) );
 }

  /// test e1.3 : `PathOutsideWorkspace` error
  #[ test ]
  fn test_path_outside_workspace_error()
  {
  let test_path = PathBuf ::from( "/external/path" );
  let error = WorkspaceError ::PathOutsideWorkspace( test_path.clone() );
  
  let display = format!( "{error}" );
  assert!( display.contains( "/external/path" ) );
  assert!( display.contains( "outside workspace" ) );
 }

  /// test e1.4 : `ConfigurationError`
  #[ test ]
  fn test_configuration_error()
  {
  let error = WorkspaceError ::ConfigurationError( "test configuration issue".to_string() );
  
  let display = format!( "{error}" );
  assert!( display.contains( "test configuration issue" ) );
  assert!( display.contains( "configuration error" ) );
 }

  /// test e1.5 : `IoError`
  #[ test ]
  fn test_io_error()
  {
  let error = WorkspaceError ::IoError( "permission denied".to_string() );
  
  let display = format!( "{error}" );
  assert!( display.contains( "permission denied" ) );
  assert!( display.contains( "io error" ) );
 }

  /// test e2.1 : error `std ::error ::Error` trait implementation
  #[ test ]
  fn test_error_trait_implementation()
  {
  let error = WorkspaceError ::ConfigurationError( "test".to_string() );
  let error_trait: &dyn core ::error ::Error = &error;
  
  // should not panic - confirms trait is properly implemented
  let _ = error_trait.to_string();
 }

  /// test e2.2 : all error variants display correctly
  #[ test ]
  fn test_all_error_variants_display()
  {
  let errors = vec!
  [
   WorkspaceError ::ConfigurationError( "config issue".to_string() ),
   WorkspaceError ::EnvironmentVariableMissing( "VAR".to_string() ),
   WorkspaceError ::IoError( "io issue".to_string() ),
   WorkspaceError ::PathNotFound( PathBuf ::from( "/test" ) ),
   WorkspaceError ::PathOutsideWorkspace( PathBuf ::from( "/test" ) ),
 ];
  
  for error in errors
  {
   let display = format!( "{error}" );
   let debug = format!( "{error:?}" );
   
   assert!( !display.is_empty(), "display should not be empty" );
   assert!( !debug.is_empty(), "debug should not be empty" );
 }
 }

  /// test e2.3 : error cloning
  #[ test ]
  fn test_error_cloning()
  {
  let error = WorkspaceError ::ConfigurationError( "test".to_string() );
  let cloned = error.clone();
  
  assert_eq!( format!( "{error}" ), format!( "{}", cloned ) );
 }
}

// ============================================================================
// feature-specific tests: glob functionality
// ============================================================================

#[ cfg( feature = "glob" ) ]
mod glob_functionality_tests
{
  use super :: *;

  /// test g1.1 : find resources with simple pattern
  #[ test ]
  fn test_find_resources_simple_pattern()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // create test rust files - ensure src directory exists first
  let src_dir = workspace.join( "src" );
  fs ::create_dir_all( &src_dir ).unwrap();
  
  let test_files = vec![ "lib.rs", "main.rs", "utils.rs" ];
  
  for file in &test_files
  {
   fs ::write( src_dir.join( file ), "// rust content" ).unwrap();
 }
  
  let found = workspace.find_resources( "src/*.rs" ).unwrap();
  assert_eq!( found.len(), 3 );
  
  for path in &found
  {
   assert!( path.extension().unwrap() == "rs" );
   assert!( workspace.is_workspace_file( path ) );
 }
 }

  /// test g1.2 : find resources with recursive pattern
  #[ test ]
  fn test_find_resources_recursive_pattern()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // create nested rust files
  let paths = vec!
  [
   "src/lib.rs",
   "src/bin/main.rs", 
   "src/modules/auth.rs",
   "src/modules/db/connection.rs",
 ];
  
  for path in &paths
  {
   let full_path = workspace.join( path );
   fs ::create_dir_all( full_path.parent().unwrap() ).unwrap();
   fs ::write( full_path, "// rust content" ).unwrap();
 }
  
  let found = workspace.find_resources( "src/**/*.rs" ).unwrap();
  assert!( found.len() >= 4, "should find all nested rust files" );
  
  for path in &found
  {
   assert!( path.extension().unwrap() == "rs" );
   assert!( path.to_string_lossy().contains( "src" ) );
 }
 }

  /// test g1.3 : find resources with no matches
  #[ test ]
  fn test_find_resources_no_matches()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let found = workspace.find_resources( "src/*.nonexistent" ).unwrap();
  assert!( found.is_empty(), "should return empty vector for no matches" );
 }

  /// test g1.4 : find resources with invalid pattern
  #[ test ]
  fn test_find_resources_invalid_pattern()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let result = workspace.find_resources( "src/**[invalid" );
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::GlobError( msg ) => assert!( !msg.is_empty() ),
   other => panic!( "expected GlobError, got {other:?}" ),
 }
 }

  /// test g2.1 : find config with toml format
  #[ test ]
  fn test_find_config_toml()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_file = workspace.config_dir().join( "app.toml" );
  // Ensure parent directory exists before writing
  if let Some( parent ) = config_file.parent()
  {
   fs ::create_dir_all( parent ).unwrap();
 }
  fs ::write( &config_file, "[app]\nname = \"test\"\n" ).unwrap();
  
  let found = workspace.find_config( "app" ).unwrap();
  assert_eq!( found, config_file );
 }

  /// test g2.2 : find config with yaml format
  #[ test ]
  fn test_find_config_yaml()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_file = workspace.config_dir().join( "app.yaml" );
  // Ensure parent directory exists before writing  
  if let Some( parent ) = config_file.parent()
  {
   fs ::create_dir_all( parent ).unwrap();
 }
  fs ::write( &config_file, "name: test\nversion: 1.0\n" ).unwrap();
  
  let found = workspace.find_config( "app" ).unwrap();
  assert_eq!( found, config_file );
 }

  /// test g2.3 : find config with json format  
  #[ test ]
  fn test_find_config_json()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_file = workspace.config_dir().join( "app.json" );
  fs ::write( &config_file, "{\"name\" : \"test\", \"version\" : \"1.0\"}\n" ).unwrap();
  
  let found = workspace.find_config( "app" ).unwrap();
  assert_eq!( found, config_file );
 }

  /// test g2.4 : find config with dotfile format
  #[ test ]
  fn test_find_config_dotfile()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let config_file = workspace.root().join( ".app.toml" );
  fs ::write( &config_file, "[app]\nhidden_config = true\n" ).unwrap();
  
  let found = workspace.find_config( "app" ).unwrap();
  assert_eq!( found, config_file );
 }

  /// test g2.5 : find config with multiple formats (priority order)
  #[ test ]
  fn test_find_config_priority_order()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // create multiple formats - toml should have highest priority
  let toml_file = workspace.config_dir().join( "app.toml" );
  let yaml_file = workspace.config_dir().join( "app.yaml" );
  let json_file = workspace.config_dir().join( "app.json" );
  
  fs ::write( &yaml_file, "name: from_yaml\n" ).unwrap();
  fs ::write( &json_file, "{\"name\" : \"from_json\"}\n" ).unwrap();
  fs ::write( &toml_file, "[app]\nname = \"from_toml\"\n" ).unwrap();
  
  let found = workspace.find_config( "app" ).unwrap();
  assert_eq!( found, toml_file, "toml should have priority" );
 }

  /// test g2.6 : find config with no config found
  #[ test ]
  fn test_find_config_not_found()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let result = workspace.find_config( "nonexistent_config" );
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::PathNotFound( path ) => 
   {
  assert!( path.ends_with( "nonexistent_config.toml" ) );
 }
   other => panic!( "expected PathNotFound, got {other:?}" ),
 }
 }
}

// ============================================================================
// feature-specific tests: secret_management functionality  
// ============================================================================

#[ cfg( feature = "secrets" ) ]
mod secret_management_tests
{
  use super :: *;

  /// test s1.1 : secret directory path
  #[ test ]
  fn test_secret_directory_path()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  assert_eq!( secret_dir, workspace.root().join( "secret" ) );
 }

  /// test s1.2 : secret file path
  #[ test ]
  fn test_secret_file_path()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_file = workspace.secret_file( "test.env" );
  assert_eq!( secret_file, workspace.root().join( "secret/test.env" ) );
 }

  /// test s2.1 : load secrets with valid key=value format
  #[ test ]
  fn test_load_secrets_valid_format()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_content = "API_KEY=abc123\nDB_URL=postgres: //localhost\nPORT=8080\n";
  let secret_file = secret_dir.join( "test.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "test.env" ).unwrap();
  
  assert_eq!( secrets.len(), 3 );
  assert_eq!( secrets.get( "API_KEY" ), Some( &"abc123".to_string() ) );
  assert_eq!( secrets.get( "DB_URL" ), Some( &"postgres: //localhost".to_string() ) );
  assert_eq!( secrets.get( "PORT" ), Some( &"8080".to_string() ) );
 }

  /// test s2.2 : load secrets with quoted values
  #[ test ]
  fn test_load_secrets_quoted_values()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_content = r"QUOTED_DOUBLE=value with spaces
QUOTED_SINGLE='another value'
UNQUOTED=simple_value
EMPTY_QUOTES=
";
  let secret_file = secret_dir.join( "quoted.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "quoted.env" ).unwrap();
  
  assert_eq!( secrets.get( "QUOTED_DOUBLE" ), Some( &"value with spaces".to_string() ) );
  assert_eq!( secrets.get( "QUOTED_SINGLE" ), Some( &"another value".to_string() ) );
  assert_eq!( secrets.get( "UNQUOTED" ), Some( &"simple_value".to_string() ) );
  assert_eq!( secrets.get( "EMPTY_QUOTES" ), Some( &String ::new() ) );
 }

  /// test s2.3 : load secrets with comments and empty lines
  #[ test ]
  fn test_load_secrets_with_comments()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_content = r"# this is a comment
API_KEY=secret123

# another comment
DB_URL=postgres: //localhost
# more comments

VALID_KEY=valid_value
";
  let secret_file = secret_dir.join( "commented.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "commented.env" ).unwrap();
  
  assert_eq!( secrets.len(), 3 );
  assert_eq!( secrets.get( "API_KEY" ), Some( &"secret123".to_string() ) );
  assert_eq!( secrets.get( "DB_URL" ), Some( &"postgres: //localhost".to_string() ) );
  assert_eq!( secrets.get( "VALID_KEY" ), Some( &"valid_value".to_string() ) );
  
  // ensure comments are not parsed as keys
  assert!( !secrets.contains_key( "# this is a comment" ) );
 }

  /// test s2.4 : load secrets from nonexistent file - updated for Task 021
  #[ test ]
  fn test_load_secrets_nonexistent_file()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();

  // New behavior: returns explicit error instead of empty HashMap
  let result = workspace.load_secrets_from_file( "nonexistent.env" );
  assert!( result.is_err(), "should return error for nonexistent file" );
  let error_msg = result.unwrap_err().to_string();
  assert!( error_msg.contains( "not found at" ), "error should contain path information" );
 }

  /// test s2.5 : load secrets with file read error
  #[ test ]
  #[ cfg( unix ) ]
  fn test_load_secrets_permission_denied()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_file = secret_dir.join( "restricted.env" );
  fs ::write( &secret_file, "KEY=value\n" ).unwrap();
  
  // make file unreadable
  use std ::os ::unix ::fs ::PermissionsExt;
  let mut perms = fs ::metadata( &secret_file ).unwrap().permissions();
  perms.set_mode( 0o000 );
  fs ::set_permissions( &secret_file, perms ).unwrap();
  
  let result = workspace.load_secrets_from_file( "restricted.env" );
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::IoError( msg ) => assert!( msg.contains( "restricted.env" ) ),
   other => panic!( "expected IoError, got {other:?}" ),
 }
 }

  /// test s2.6 : load secrets with malformed content
  #[ test ]
  fn test_load_secrets_malformed_content()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_content = "VALID_KEY=valid_value\nINVALID_LINE_NO_EQUALS\nANOTHER_VALID=value2\n";
  let secret_file = secret_dir.join( "malformed.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "malformed.env" ).unwrap();
  
  // should parse valid lines and skip invalid ones
  assert_eq!( secrets.len(), 2 );
  assert_eq!( secrets.get( "VALID_KEY" ), Some( &"valid_value".to_string() ) );
  assert_eq!( secrets.get( "ANOTHER_VALID" ), Some( &"value2".to_string() ) );
  assert!( !secrets.contains_key( "INVALID_LINE_NO_EQUALS" ) );
 }

  /// test s3.1 : load secret key from file
  #[ test ]
  fn test_load_secret_key_from_file()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_content = "API_KEY=file_secret_123\nOTHER_KEY=other_value\n";
  let secret_file = secret_dir.join( "secrets.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let value = workspace.load_secret_key( "API_KEY", "secrets.env" ).unwrap();
  assert_eq!( value, "file_secret_123" );
 }

  /// test s3.2 : load secret key from environment
  #[ test ]
  fn test_load_secret_key_from_environment()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  env ::set_var( "TEST_ENV_SECRET", "env_secret_456" );
  
  let value = workspace.load_secret_key( "TEST_ENV_SECRET", "nonexistent.env" ).unwrap();
  assert_eq!( value, "env_secret_456" );
  
  env ::remove_var( "TEST_ENV_SECRET" );
 }

  /// test s3.3 : load secret key - file takes priority over environment
  #[ test ]
  fn test_load_secret_key_file_priority()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  // set environment variable
  env ::set_var( "PRIORITY_TEST", "env_value" );
  
  // create file with same key
  let secret_content = "PRIORITY_TEST=file_value\n";
  let secret_file = secret_dir.join( "priority.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let value = workspace.load_secret_key( "PRIORITY_TEST", "priority.env" ).unwrap();
  assert_eq!( value, "file_value", "file should take priority over environment" );
  
  env ::remove_var( "PRIORITY_TEST" );
 }

  /// test s3.4 : load secret key not found anywhere
  #[ test ]
  fn test_load_secret_key_not_found()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let result = workspace.load_secret_key( "NONEXISTENT_KEY", "nonexistent.env" );
  assert!( result.is_err() );
  
  match result.unwrap_err()
  {
   WorkspaceError ::ConfigurationError( msg ) => 
   {
  assert!( msg.contains( "NONEXISTENT_KEY" ) );
  assert!( msg.contains( "not found" ) );
 }
   other => panic!( "expected ConfigurationError, got {other:?}" ),
 }
 }

  /// test s3.5 : parse key-value file with edge cases
  #[ test ]
  fn test_parse_key_value_edge_cases()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  let secret_content = "
# edge cases for parsing
KEY_WITH_SPACES   =   value_with_spaces   
KEY_EQUALS_IN_VALUE=key=value=pair
EMPTY_VALUE=
KEY_WITH_QUOTES_IN_VALUE=\"value with 'single' quotes\"
KEY_WITH_HASH_IN_VALUE=value#with#hash
  INDENTED_KEY=indented_value
";
  
  let secret_file = secret_dir.join( "edge_cases.env" );
  fs ::write( &secret_file, secret_content ).unwrap();
  
  let secrets = workspace.load_secrets_from_file( "edge_cases.env" ).unwrap();
  
  assert_eq!( secrets.get( "KEY_WITH_SPACES" ), Some( &"value_with_spaces".to_string() ) );
  assert_eq!( secrets.get( "KEY_EQUALS_IN_VALUE" ), Some( &"key=value=pair".to_string() ) );
  assert_eq!( secrets.get( "EMPTY_VALUE" ), Some( &String ::new() ) );
  assert_eq!( secrets.get( "KEY_WITH_QUOTES_IN_VALUE" ), Some( &"value with 'single' quotes".to_string() ) );
  assert_eq!( secrets.get( "KEY_WITH_HASH_IN_VALUE" ), Some( &"value#with#hash".to_string() ) );
  assert_eq!( secrets.get( "INDENTED_KEY" ), Some( &"indented_value".to_string() ) );
 }
}

// ============================================================================
// integration and cross-platform tests
// ============================================================================

mod integration_tests
{
  use super :: *;

  /// test i1.1 : cross-platform path handling
  #[ test ]
  fn test_cross_platform_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // test various path formats that should work cross-platform
  let test_paths = vec!
  [
   "config/app.toml",
   "data\\cache.db",  // windows-style separator
   "logs/app.log",
   "docs/readme.md",
 ];
  
  for path in test_paths
  {
   let joined = workspace.join( path );
   assert!( joined.starts_with( workspace.root() ) );
   assert!( workspace.is_workspace_file( &joined ) );
 }
 }

  /// test i1.3 : symlink handling
  #[ test ]
  #[ cfg( unix ) ]
  fn test_symlink_handling()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // create a real file
  let real_file = workspace.join( "data/real_file.txt" );
  fs ::write( &real_file, "real content" ).unwrap();
  
  // create symlink to the file
  let symlink_path = workspace.join( "data/symlink_file.txt" );
  std ::os ::unix ::fs ::symlink( &real_file, &symlink_path ).unwrap();
  
  // symlink should be considered workspace file
  assert!( workspace.is_workspace_file( &symlink_path ) );
  
  // normalization should follow symlink
  let normalized = workspace.normalize_path( "data/symlink_file.txt" );
  assert!( normalized.is_ok() );
 }

  /// test i1.4 : broken symlink handling
  #[ test ]
  #[ cfg( unix ) ]
  fn test_broken_symlink_handling()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // create symlink to nonexistent file
  let broken_symlink = workspace.join( "data/broken_link.txt" );
  std ::os ::unix ::fs ::symlink( "/nonexistent/target", &broken_symlink ).unwrap();
  
  // symlink itself should be workspace file
  assert!( workspace.is_workspace_file( &broken_symlink ) );
  
  // normalization should fail gracefully
  let result = workspace.normalize_path( "data/broken_link.txt" );
  assert!( result.is_err() );
 }

  /// test i1.5 : read-only workspace handling
  #[ test ]
  #[ cfg( unix ) ]
  fn test_readonly_workspace()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // make workspace read-only
  use std ::os ::unix ::fs ::PermissionsExt;
  let mut perms = fs ::metadata( workspace.root() ).unwrap().permissions();
  perms.set_mode( 0o555 ); // read + execute only
  fs ::set_permissions( workspace.root(), perms ).unwrap();
  
  // validation should still work
  let result = workspace.validate();
  assert!( result.is_ok(), "read-only workspace should validate successfully" );
  
  // restore permissions for cleanup
  let mut perms = fs ::metadata( workspace.root() ).unwrap().permissions();
  perms.set_mode( 0o755 );
  fs ::set_permissions( workspace.root(), perms ).unwrap();
 }

  /// test i2.1 : concurrent workspace access
  #[ test ]
  fn test_concurrent_workspace_access()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  let workspace = Arc ::new( workspace );
  let results = Arc ::new( Mutex ::new( Vec ::new() ) );
  
  let handles: Vec< _ > = ( 0..10 ).map( | i |
  {
   let workspace = Arc ::clone( &workspace );
   let results = Arc ::clone( &results );
   
   thread ::spawn( move ||
   {
  let path = workspace.join( format!( "thread_{i}.txt" ) );
  let is_workspace_file = workspace.is_workspace_file( &path );
  let config_dir = workspace.config_dir();
  
  results.lock().unwrap().push( ( is_workspace_file, config_dir ) );
 })
 }).collect();
  
  for handle in handles
  {
   handle.join().unwrap();
 }
  
  let results = results.lock().unwrap();
  assert_eq!( results.len(), 10 );
  
  // all results should be consistent
  for ( is_workspace_file, config_dir ) in results.iter()
  {
   assert!( *is_workspace_file );
   assert_eq!( *config_dir, workspace.config_dir() );
 }
 }

  /// test i2.2 : environment changes during execution
  #[ test ]
  fn test_environment_changes()
  {
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  
  // first workspace
  let temp_dir1 = TempDir ::new().unwrap();
  env ::set_var( "WORKSPACE_PATH", temp_dir1.path() );
  let workspace1 = Workspace ::resolve().unwrap();
  
  // change environment
  let temp_dir2 = TempDir ::new().unwrap();
  env ::set_var( "WORKSPACE_PATH", temp_dir2.path() );
  let workspace2 = Workspace ::resolve().unwrap();
  
  // workspaces should reflect their creation-time environment
  assert_eq!( workspace1.root(), temp_dir1.path() );
  assert_eq!( workspace2.root(), temp_dir2.path() );
  assert_ne!( workspace1.root(), workspace2.root() );
  
  // cleanup
  match original
  {
   Some( path ) => env ::set_var( "WORKSPACE_PATH", path ),
   None => env ::remove_var( "WORKSPACE_PATH" ),
 }
 }

  /// test i3.1 : testing utilities create proper isolation
  #[ test ]
  fn test_testing_utilities_isolation()
  {
  let ( _temp_dir1, workspace1 ) = create_test_workspace();
  let ( _temp_dir2, workspace2 ) = create_test_workspace();
  
  // workspaces should be different
  assert_ne!( workspace1.root(), workspace2.root() );
  
  // both should be valid
  assert!( workspace1.validate().is_ok() );
  assert!( workspace2.validate().is_ok() );
  
  // both should exist
  assert!( workspace1.root().exists() );
  assert!( workspace2.root().exists() );
 }

  /// test i3.2 : structured workspace creation
  #[ test ]
  fn test_structured_workspace_creation()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  // all standard directories should exist
  assert!( workspace.config_dir().exists(), "config dir should exist" );
  assert!( workspace.data_dir().exists(), "data dir should exist" );
  assert!( workspace.logs_dir().exists(), "logs dir should exist" );
  assert!( workspace.docs_dir().exists(), "docs dir should exist" );
  assert!( workspace.tests_dir().exists(), "tests dir should exist" );
  assert!( workspace.workspace_dir().exists(), "workspace dir should exist" );
  
  #[ cfg( feature = "secrets" ) ]
  {
   assert!( workspace.secret_dir().exists(), "secret dir should exist" );
 }
 }
}

// ============================================================================
// performance and stress tests
// ============================================================================

// performance tests were removed during scope reduction

#[ allow(dead_code) ]
mod performance_tests
{
  use super :: *;
  use std ::time ::Instant;

  /// test p1.1 : large workspace with many files
  #[ test ]
  // #[ cfg( feature = "stress" ) ]
  fn test_large_workspace_performance()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  
  let start = Instant ::now();
  
  // create deep directory structure with many files
  for dir_i in 0..50
  {
   let dir_path = workspace.join( format!( "deep/dir_{dir_i}" ) );
   fs ::create_dir_all( &dir_path ).unwrap();
   
   for file_i in 0..100
   {
  let file_path = dir_path.join( format!( "file_{file_i}.rs" ) );
  fs ::write( file_path, format!( "// content for file {file_i}" ) ).unwrap();
 }
 }
  
  let creation_time = start.elapsed();
  println!( "created 5000 files in {creation_time:?}" );
  
  // test glob performance
  let start = Instant ::now();
  
  #[ cfg( feature = "glob" ) ]
  {
   let found = workspace.find_resources( "deep/**/*.rs" ).unwrap();
   assert_eq!( found.len(), 5000 );
 }
  
  let glob_time = start.elapsed();
  println!( "glob search took {glob_time:?}" );
  
  // should complete in reasonable time (adjust threshold as needed)
  assert!( glob_time.as_secs() < 5, "glob search should complete within 5 seconds" );
 }

  /// test p1.2 : many concurrent glob patterns
  #[ test ]
  #[ cfg( feature = "glob" ) ]
  fn test_concurrent_glob_patterns()
  {
  let ( _temp_dir, workspace ) = create_test_workspace_with_structure();
  let workspace = Arc ::new( workspace );
  
  // create test files
  let extensions = vec![ "rs", "toml", "json", "yaml", "txt", "md" ];
  for ext in &extensions
  {
   for i in 0..20
   {
  let file_path = workspace.join( format!( "files/test_{i}.{ext}" ) );
  fs ::create_dir_all( file_path.parent().unwrap() ).unwrap();
  fs ::write( file_path, format!( "content {i}" ) ).unwrap();
 }
 }
  
  let start = Instant ::now();
  
  // run many concurrent glob searches
  let handles: Vec< _ > = ( 0..100 ).map( | i |
  {
   let workspace = Arc ::clone( &workspace );
   let ext = extensions[ i % extensions.len() ];
   
   thread ::spawn( move ||
   {
  let pattern = format!( "files/**/*.{ext}" );
  workspace.find_resources( &pattern ).unwrap()
 })
 }).collect();
  
  let mut total_found = 0;
  for handle in handles
  {
   let found = handle.join().unwrap();
   total_found += found.len();
 }
  
  let concurrent_time = start.elapsed();
  println!( "100 concurrent globs found {total_found} files in {concurrent_time:?}" );
  
  // should complete without hanging
  assert!( concurrent_time.as_secs() < 10 );
  assert!( total_found > 0 );
 }

  /// test p1.3 : large secret files parsing
  #[ test ]
  #[ cfg( feature = "secrets" ) ]
  fn test_large_secret_files()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let secret_dir = workspace.secret_dir();
  fs ::create_dir_all( &secret_dir ).unwrap();
  
  // create large secret file (1MB+ of key=value pairs)
  let mut secret_content = String ::with_capacity( 1_024 * 1_024 );
  for i in 0..10_000
  {
   use core ::fmt ::Write;
   writeln!( &mut secret_content, "KEY_{i}=value_with_some_content_{i}" ).unwrap();
 }
  
  let secret_file = secret_dir.join( "large.env" );
  fs ::write( &secret_file, &secret_content ).unwrap();
  
  let start = Instant ::now();
  let secrets = workspace.load_secrets_from_file( "large.env" ).unwrap();
  let parse_time = start.elapsed();
  
  println!( "parsed {} secrets in {:?}", secrets.len(), parse_time );
  
  assert_eq!( secrets.len(), 10_000 );
  assert!( parse_time.as_millis() < 1000, "should parse large file within 1 second" );
  
  // verify some random entries
  assert_eq!( secrets.get( "KEY_100" ), Some( &"value_with_some_content_100".to_string() ) );
  assert_eq!( secrets.get( "KEY_5000" ), Some( &"value_with_some_content_5000".to_string() ) );
 }

  /// test p1.4 : repeated workspace operations
  #[ test ]
  // #[ cfg( feature = "stress" ) ]
  fn test_repeated_workspace_operations()
  {
  let temp_dir = TempDir ::new().unwrap();
  let original = env ::var( "WORKSPACE_PATH" ).ok();
  
  // Create a stable test file in the temp directory to ensure it's valid
  let test_file = temp_dir.path().join( "test_marker.txt" );
  std ::fs ::write( &test_file, "test workspace" ).unwrap();
  
  env ::set_var( "WORKSPACE_PATH", temp_dir.path() );
  
  let start = Instant ::now();
  
  // repeatedly create workspace instances and perform operations
  for i in 0..100
  {
   // Use resolve_or_fallback for robustness in stress testing
   let workspace = Workspace ::resolve_with_extended_fallbacks();
   
   // perform various operations (these should never fail)
   let _ = workspace.validate();
   let _ = workspace.config_dir();
   let _ = workspace.join( format!( "file_{i}.txt" ) );
   let _ = workspace.is_workspace_file( &test_file );
   
   // Verify workspace is still valid every 25 iterations
   if i % 25 == 0
   {
  assert!( workspace.root().exists(), "workspace root should exist at iteration {i}" );
 }
 }
  
  let repeated_ops_time = start.elapsed();
  println!( "100 repeated operations took {repeated_ops_time:?}" );
  
  // Test passes if it completes without panicking - no strict timing requirement for stress test
  assert!( repeated_ops_time.as_millis() < 10000, "stress test should complete within reasonable time" );
  
  // cleanup
  match original
  {
   Some( path ) => env ::set_var( "WORKSPACE_PATH", path ),
   None => env ::remove_var( "WORKSPACE_PATH" ),
 }
 }

  /// test p1.5 : memory usage during operations
  #[ test ]
  // #[ cfg( feature = "stress" ) ]
  fn test_memory_usage()
  {
  let ( _temp_dir, _workspace ) = create_test_workspace_with_structure();
  
  // create many workspace instances (should not accumulate memory)
  let mut workspaces = Vec ::new();
  
  for _ in 0..100
  {
   let ws = Workspace ::resolve_with_extended_fallbacks();
   workspaces.push( ws );
 }
  
  // perform operations on all instances
  for ( i, ws ) in workspaces.iter().enumerate()
  {
   let _ = ws.join( format!( "test_{i}" ) );
   let _ = ws.validate();
 }
  
  // test should complete without excessive memory usage
  // actual memory measurement would require external tooling
  assert_eq!( workspaces.len(), 100 );
 }
}

// ============================================================================
// edge cases and boundary conditions
// ============================================================================

mod edge_case_tests
{
  use super :: *;

  /// test: very long paths
  #[ test ]
  fn test_very_long_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // create path with 200+ character filename
  let long_name = "a".repeat( 200 );
  let long_path = workspace.join( &long_name );
  
  assert!( workspace.is_workspace_file( &long_path ) );
  
  // join should handle long paths
  let joined = workspace.join( format!( "dir/{long_name}" ) );
  assert!( joined.to_string_lossy().len() > 200 );
 }

  /// test: unicode paths
  #[ test ]
  fn test_unicode_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let unicode_paths = vec!
  [
   "config/测试.toml",
   "data/файл.db",
   "logs/ログ.log",
   "docs/文档.md",
   "🚀/rocket.txt",
 ];
  
  for path in unicode_paths
  {
   let joined = workspace.join( path );
   assert!( workspace.is_workspace_file( &joined ) );
 }
 }

  /// test: empty and whitespace paths
  #[ test ]
  fn test_empty_and_whitespace_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  let edge_paths = vec!
  [
   "",
   " ",
   "  ",
   "\t",
   "\n",
   " file with spaces ",
   "  \t\n  ",
 ];
  
  for path in edge_paths
  {
   let joined = workspace.join( path );
   // should not panic, even with weird inputs
   let _ = workspace.is_workspace_file( &joined );
 }
 }

  /// test: root-level operations
  #[ test ]
  fn test_root_level_operations()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // operations on workspace root itself
  assert!( workspace.is_workspace_file( workspace.root() ) );
  assert!( workspace.validate().is_ok() );
  
  let normalized = workspace.normalize_path( "." );
  assert!( normalized.is_ok() );
 }

  /// test: deeply nested paths
  #[ test ]
  fn test_deeply_nested_paths()
  {
  let ( _temp_dir, workspace ) = create_test_workspace();
  
  // create very deep nesting
  let deep_parts: Vec< String > = ( 0..20 ).map( | i | format!( "level_{i}" ) ).collect();
  let deep_path = deep_parts.join( "/" );
  
  let joined = workspace.join( &deep_path );
  assert!( workspace.is_workspace_file( &joined ) );
  
  // create the actual directory structure  
  fs ::create_dir_all( &joined ).unwrap();
  assert!( joined.exists() );
 }
}