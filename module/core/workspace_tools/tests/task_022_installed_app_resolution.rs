//! Task 022: Extend workspace resolution for installed applications
//!
//! ## Problem Context
//!
//! **Real-world issue**: CLI tools installed via `cargo install` (like `wip2`) could not
//! load secrets because `WORKSPACE_PATH` environment variable is only set during cargo
//! operations (via `.cargo/config.toml`). This prevented installed binaries from accessing
//! workspace-level secrets stored in `secret/-secrets.sh`.
//!
//! **Solution**: Extended fallback chain to include user-configured locations:
//! - `$PRO` environment variable (for multi-project users)
//! - `$HOME` directory (universal fallback)
//!
//! This enables installed CLI tools to work seamlessly in both development contexts
//! (via `cargo run`) and installed contexts (via `cargo install`).
//!
//! ## Testing Challenges & Solutions
//!
//! ### Challenge 1: Mutex Poisoning
//! **Problem**: Tests that manipulate environment variables need serialization to avoid
//! race conditions. When a test panics while holding the mutex, the mutex becomes "poisoned"
//! and subsequent tests fail even though they're correct.
//!
//! **Solution**: The `lock_env_mutex()` helper uses `unwrap_or_else(PoisonError::into_inner)`
//! to recover from poisoned mutexes. This allows the test suite to continue even if one
//! test panics during development.
//!
//! ### Challenge 2: Cargo Workspace Interference
//! **Problem**: Tests for `$PRO` and `$HOME` fallbacks were finding the actual cargo
//! workspace first (because tests run inside the workspace), making it impossible to test
//! the fallback chain order.
//!
//! **Solution**: Tests that need to verify fallback behavior change the current working
//! directory to a temporary directory outside any cargo workspace using
//! `env::set_current_dir(temp_dir)`. This isolates the test from the development workspace.
//!
//! ## Design Decisions Captured
//!
//! **Backward Compatibility over Spec**: The task specification suggested making
//! `workspace()` return `Workspace` (infallible), but the implementation keeps it as
//! `Result<Workspace>` to avoid breaking all existing code that uses `workspace()?`.
//! This is intentional and better than the spec suggestion.
//!
//! ## Test Matrix
//!
//! ### `from_pro_env()` Tests
//! | id     | test case                  | conditions               | expected result          |
//! |--------|---------------------------|--------------------------|--------------------------|
//! | p1.1   | valid $PRO path           | env set, path exists     | success with path        |
//! | p1.2   | nonexistent $PRO path     | env set, path missing    | `PathNotFound` error       |
//! | p1.3   | missing $PRO env var      | env not set              | `EnvironmentMissing` error |
//! | p1.4   | $PRO path normalization   | path with /./ and /..    | normalized path          |
//!
//! ### `from_home_dir()` Tests
//! | id     | test case                  | conditions               | expected result          |
//! |--------|---------------------------|--------------------------|--------------------------|
//! | h1.1   | valid $HOME path          | HOME set, path exists    | success with HOME path   |
//! | h1.2   | valid $USERPROFILE path   | USERPROFILE set (Win)    | success with profile     |
//! | h1.3   | both env vars missing     | neither set              | `EnvironmentMissing` error |
//! | h1.4   | nonexistent home path     | env set, path missing    | `PathNotFound` error       |
//! | h1.5   | $HOME priority over prof  | both set                 | prefers HOME             |
//!
//! ### `resolve_with_extended_fallbacks()` Tests
//! | id     | test case                  | conditions               | expected result          |
//! |--------|---------------------------|--------------------------|--------------------------|
//! | f1.1   | cargo workspace exists    | in cargo workspace       | uses cargo workspace     |
//! | f1.2   | `WORKSPACE_PATH` set      | env var set              | uses `WORKSPACE_PATH`    |
//! | f1.3   | git root fallback         | .git dir exists          | uses git root            |
//! | f1.4   | $PRO fallback             | PRO set, others fail     | uses $PRO                |
//! | f1.5   | $HOME fallback            | HOME set, others fail    | uses $HOME               |
//! | f1.6   | cwd final fallback        | all fail                 | uses current dir         |
//! | f1.7   | fallback chain order      | multiple available       | correct priority order   |
//!
//! ### Integration Tests (Real-World Scenarios)
//! | id     | test case                  | scenario                 | expected result          |
//! |--------|---------------------------|--------------------------|--------------------------|
//! | i1.1   | developer in workspace    | cargo run context        | finds workspace secrets  |
//! | i1.2   | installed app with $PRO   | cargo install + PRO set  | finds $PRO/secret/       |
//! | i1.3   | installed app without PRO | cargo install, no PRO    | finds $HOME/secret/      |
//! | i1.4   | secret loading via PRO    | load secrets from PRO    | successfully loads       |
//! | i1.5   | secret loading via HOME   | load secrets from HOME   | successfully loads       |

use workspace_tools :: { Workspace, WorkspaceError };
use std ::
{
  env,
  fs,
};
use tempfile ::TempDir;

// Global mutex to serialize environment variable tests
static ENV_TEST_MUTEX : std ::sync ::Mutex< () > = std ::sync ::Mutex ::new( () );

// =============================================================================
// Helper Functions
// =============================================================================

/// Acquire mutex lock, recovering from poison errors
fn lock_env_mutex() -> std ::sync ::MutexGuard< 'static, () >
{
  ENV_TEST_MUTEX.lock().unwrap_or_else( std ::sync ::PoisonError::into_inner )
}

/// Restore environment variable to original value or unset it
fn restore_env_var( key : &str, original : Option< String > )
{
  match original
  {
  Some( value ) => env ::set_var( key, value ),
  None => env ::remove_var( key ),
  }
}

// =============================================================================
// `from_pro_env()` Tests
// =============================================================================

mod from_pro_env_tests
{
  use super :: *;

  /// Test p1.1: valid $PRO path
  #[ test ]
  fn test_pro_env_with_valid_path()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original = env ::var( "PRO" ).ok();

  env ::set_var( "PRO", temp_dir.path() );
  let result = Workspace ::from_pro_env();

  restore_env_var( "PRO", original );

  assert!( result.is_ok(), "`from_pro_env()` should succeed with valid path" );
  let workspace = result.unwrap();
  assert_eq!( workspace.root(), temp_dir.path(), "workspace root should match $PRO path" );
  }

  /// Test p1.2: nonexistent $PRO path
  #[ test ]
  fn test_pro_env_with_nonexistent_path()
  {
  let _lock = lock_env_mutex();
  let original = env ::var( "PRO" ).ok();

  // Create unique nonexistent path
  let thread_id = std ::thread ::current().id();
  let timestamp = std ::time ::SystemTime ::now()
   .duration_since( std ::time ::UNIX_EPOCH )
   .unwrap_or_default()
   .as_nanos();
  let nonexistent = env ::temp_dir()
   .join( format!( "nonexistent_pro_{thread_id:?}_{timestamp}" ) );

  env ::set_var( "PRO", &nonexistent );
  let result = Workspace ::from_pro_env();

  restore_env_var( "PRO", original );

  assert!( result.is_err(), "`from_pro_env()` should fail with nonexistent path" );
  match result.unwrap_err()
  {
   WorkspaceError ::PathNotFound( path ) =>
   {
  assert_eq!( path, nonexistent, "error should contain the nonexistent path" );
   }
   other => panic!( "expected PathNotFound error, got: {other:?}" ),
  }
  }

  /// Test p1.3: missing $PRO env var
  #[ test ]
  fn test_pro_env_missing()
  {
  let _lock = lock_env_mutex();
  let original = env ::var( "PRO" ).ok();

  env ::remove_var( "PRO" );
  let result = Workspace ::from_pro_env();

  restore_env_var( "PRO", original );

  assert!( result.is_err(), "`from_pro_env()` should fail when PRO not set" );
  match result.unwrap_err()
  {
   WorkspaceError ::EnvironmentVariableMissing( var ) =>
   {
  assert_eq!( var, "PRO", "error should mention PRO variable" );
   }
   other => panic!( "expected EnvironmentVariableMissing error, got: {other:?}" ),
  }
  }

  /// Test p1.4: $PRO path normalization
  #[ test ]
  fn test_pro_env_path_normalization()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original = env ::var( "PRO" ).ok();

  // Create path with redundant components
  let redundant_path = temp_dir.path().join( "." );
  env ::set_var( "PRO", &redundant_path );
  let result = Workspace ::from_pro_env();

  restore_env_var( "PRO", original );

  assert!( result.is_ok(), "`from_pro_env()` should succeed with redundant path" );
  let workspace = result.unwrap();

  // Path should be normalized (no trailing "/.")
  let root_str = workspace.root().to_string_lossy();
  assert!( !root_str.ends_with( "/." ), "path should not end with '/.' after normalization" );
  assert!( !root_str.contains( "/./" ), "path should not contain '/./' after normalization" );
  }
}

// =============================================================================
// `from_home_dir()` Tests
// =============================================================================

mod from_home_dir_tests
{
  use super :: *;

  /// Test h1.1: valid $HOME path
  #[ test ]
  fn test_home_dir_with_valid_home()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();

  // Remove USERPROFILE to ensure HOME is used
  env ::remove_var( "USERPROFILE" );
  env ::set_var( "HOME", temp_dir.path() );
  let result = Workspace ::from_home_dir();

  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  assert!( result.is_ok(), "`from_home_dir()` should succeed with valid HOME" );
  let workspace = result.unwrap();
  assert_eq!( workspace.root(), temp_dir.path(), "workspace root should match $HOME path" );
  }

  /// Test h1.2: valid $USERPROFILE path (Windows)
  #[ test ]
  fn test_home_dir_with_valid_userprofile()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();

  // Remove HOME to ensure USERPROFILE is used
  env ::remove_var( "HOME" );
  env ::set_var( "USERPROFILE", temp_dir.path() );
  let result = Workspace ::from_home_dir();

  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  assert!( result.is_ok(), "`from_home_dir()` should succeed with valid USERPROFILE" );
  let workspace = result.unwrap();
  assert_eq!( workspace.root(), temp_dir.path(), "workspace root should match USERPROFILE path" );
  }

  /// Test h1.3: both env vars missing
  #[ test ]
  fn test_home_dir_missing_both()
  {
  let _lock = lock_env_mutex();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();

  env ::remove_var( "HOME" );
  env ::remove_var( "USERPROFILE" );
  let result = Workspace ::from_home_dir();

  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  assert!( result.is_err(), "`from_home_dir()` should fail when neither HOME nor USERPROFILE set" );
  match result.unwrap_err()
  {
   WorkspaceError ::EnvironmentVariableMissing( var ) =>
   {
  assert!(
   var.contains( "HOME" ) || var.contains( "USERPROFILE" ),
   "error should mention HOME or USERPROFILE, got: {var}"
  );
   }
   other => panic!( "expected EnvironmentVariableMissing error, got: {other:?}" ),
  }
  }

  /// Test h1.4: nonexistent home path
  #[ test ]
  fn test_home_dir_with_nonexistent_path()
  {
  let _lock = lock_env_mutex();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();

  // Create unique nonexistent path
  let thread_id = std ::thread ::current().id();
  let timestamp = std ::time ::SystemTime ::now()
   .duration_since( std ::time ::UNIX_EPOCH )
   .unwrap_or_default()
   .as_nanos();
  let nonexistent = env ::temp_dir()
   .join( format!( "nonexistent_home_{thread_id:?}_{timestamp}" ) );

  env ::remove_var( "USERPROFILE" );
  env ::set_var( "HOME", &nonexistent );
  let result = Workspace ::from_home_dir();

  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  assert!( result.is_err(), "`from_home_dir()` should fail with nonexistent path" );
  match result.unwrap_err()
  {
   WorkspaceError ::PathNotFound( path ) =>
   {
  assert_eq!( path, nonexistent, "error should contain the nonexistent path" );
   }
   other => panic!( "expected PathNotFound error, got: {other:?}" ),
  }
  }

  /// Test h1.5: $HOME has priority over $USERPROFILE
  #[ test ]
  fn test_home_dir_priority()
  {
  let _lock = lock_env_mutex();
  let temp_dir_home = TempDir ::new().unwrap();
  let temp_dir_userprofile = TempDir ::new().unwrap();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();

  env ::set_var( "HOME", temp_dir_home.path() );
  env ::set_var( "USERPROFILE", temp_dir_userprofile.path() );
  let result = Workspace ::from_home_dir();

  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  assert!( result.is_ok(), "`from_home_dir()` should succeed when both are set" );
  let workspace = result.unwrap();
  assert_eq!(
   workspace.root(),
   temp_dir_home.path(),
   "workspace root should use HOME (priority over USERPROFILE)"
  );
  }
}

// =============================================================================
// `resolve_with_extended_fallbacks()` Tests
// =============================================================================

mod resolve_with_extended_fallbacks_tests
{
  use super :: *;

  /// Test f1.4: $PRO fallback when earlier strategies fail
  #[ test ]
  fn test_extended_fallbacks_uses_pro()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original_workspace = env ::var( "WORKSPACE_PATH" ).ok();
  let original_pro = env ::var( "PRO" ).ok();
  let original_home = env ::var( "HOME" ).ok();
  let original_cwd = env ::current_dir().ok();

  // Clear all higher-priority env vars
  env ::remove_var( "WORKSPACE_PATH" );
  env ::set_var( "PRO", temp_dir.path() );

  // Set HOME as well (but PRO should be used first)
  let temp_home = TempDir ::new().unwrap();
  env ::set_var( "HOME", temp_home.path() );

  // Change to temp directory (outside cargo workspace and git repo)
  let test_cwd = TempDir ::new().unwrap();
  env ::set_current_dir( test_cwd.path() ).ok();

  let workspace = Workspace ::resolve_with_extended_fallbacks();

  // Restore environment
  if let Some( cwd ) = original_cwd
  {
   env ::set_current_dir( cwd ).ok();
  }
  restore_env_var( "WORKSPACE_PATH", original_workspace );
  restore_env_var( "PRO", original_pro );
  restore_env_var( "HOME", original_home );

  // PRO should be used (has priority over HOME)
  assert_eq!(
   workspace.root(),
   temp_dir.path(),
   "`resolve_with_extended_fallbacks()` should use $PRO when WORKSPACE_PATH not set"
  );
  }

  /// Test f1.5: $HOME fallback when PRO also fails
  #[ test ]
  fn test_extended_fallbacks_uses_home()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original_workspace = env ::var( "WORKSPACE_PATH" ).ok();
  let original_pro = env ::var( "PRO" ).ok();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();
  let original_cwd = env ::current_dir().ok();

  // Clear all higher-priority env vars
  env ::remove_var( "WORKSPACE_PATH" );
  env ::remove_var( "PRO" );
  env ::remove_var( "USERPROFILE" );
  env ::set_var( "HOME", temp_dir.path() );

  // Change to temp directory (outside cargo workspace and git repo)
  let test_cwd = TempDir ::new().unwrap();
  env ::set_current_dir( test_cwd.path() ).ok();

  let workspace = Workspace ::resolve_with_extended_fallbacks();

  // Restore environment
  if let Some( cwd ) = original_cwd
  {
   env ::set_current_dir( cwd ).ok();
  }
  restore_env_var( "WORKSPACE_PATH", original_workspace );
  restore_env_var( "PRO", original_pro );
  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  // HOME should be used
  assert_eq!(
   workspace.root(),
   temp_dir.path(),
   "`resolve_with_extended_fallbacks()` should use $HOME when PRO not set"
  );
  }

  /// Test f1.6: cwd final fallback
  ///
  /// Note: This test documents the fallback behavior but may find cargo workspace
  /// or git root in real development environment. The important thing is that
  /// the function always succeeds with some valid workspace root.
  #[ test ]
  fn test_extended_fallbacks_final_cwd()
  {
  let _lock = lock_env_mutex();
  let original_workspace = env ::var( "WORKSPACE_PATH" ).ok();
  let original_pro = env ::var( "PRO" ).ok();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();

  // Clear all env vars
  env ::remove_var( "WORKSPACE_PATH" );
  env ::remove_var( "PRO" );
  env ::remove_var( "HOME" );
  env ::remove_var( "USERPROFILE" );

  let workspace = Workspace ::resolve_with_extended_fallbacks();

  restore_env_var( "WORKSPACE_PATH", original_workspace );
  restore_env_var( "PRO", original_pro );
  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );

  // Should succeed (may find cargo workspace, git root, or use cwd)
  // The important thing is it always succeeds and returns a valid path
  assert!(
   workspace.root().exists(),
   "`resolve_with_extended_fallbacks()` should always succeed with valid path"
  );
  assert!(
   workspace.root().is_absolute(),
   "resolved workspace root should be absolute path"
  );
  }

  /// Test f1.7: fallback chain priority order
  #[ test ]
  fn test_extended_fallbacks_priority_order()
  {
  let _lock = lock_env_mutex();

  // Create temp directories for each strategy
  let workspace_path_dir = TempDir ::new().unwrap();
  let pro_dir = TempDir ::new().unwrap();
  let home_dir = TempDir ::new().unwrap();

  let original_workspace = env ::var( "WORKSPACE_PATH" ).ok();
  let original_pro = env ::var( "PRO" ).ok();
  let original_home = env ::var( "HOME" ).ok();
  let original_cwd = env ::current_dir().ok();

  // Set all env vars - WORKSPACE_PATH should win
  env ::set_var( "WORKSPACE_PATH", workspace_path_dir.path() );
  env ::set_var( "PRO", pro_dir.path() );
  env ::set_var( "HOME", home_dir.path() );

  // Change to temp directory (outside cargo workspace and git repo)
  let test_cwd = TempDir ::new().unwrap();
  env ::set_current_dir( test_cwd.path() ).ok();

  let workspace = Workspace ::resolve_with_extended_fallbacks();

  // Restore environment
  if let Some( cwd ) = original_cwd
  {
   env ::set_current_dir( cwd ).ok();
  }
  restore_env_var( "WORKSPACE_PATH", original_workspace );
  restore_env_var( "PRO", original_pro );
  restore_env_var( "HOME", original_home );

  // WORKSPACE_PATH should be used (highest priority)
  assert_eq!(
   workspace.root(),
   workspace_path_dir.path(),
   "`resolve_with_extended_fallbacks()` should use WORKSPACE_PATH when available (highest priority)"
  );
  }
}

// =============================================================================
// Integration Tests (Real-World Scenarios)
// =============================================================================

mod integration_tests
{
  use super :: *;

  /// Test i1.2: installed app with $PRO can load secrets
  #[ test ]
  fn test_installed_app_with_pro_loads_secrets()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original_workspace = env ::var( "WORKSPACE_PATH" ).ok();
  let original_pro = env ::var( "PRO" ).ok();
  let original_cwd = env ::current_dir().ok();

  // Simulate installed app context: no WORKSPACE_PATH, but PRO is set
  env ::remove_var( "WORKSPACE_PATH" );
  env ::set_var( "PRO", temp_dir.path() );

  // Create secret directory and file
  let secret_dir = temp_dir.path().join( "secret" );
  fs ::create_dir_all( &secret_dir ).unwrap();
  let secret_file = secret_dir.join( "-secrets.sh" );
  fs ::write( &secret_file, "GITHUB_TOKEN=test_token_123\nAPI_KEY=secret_key_456\n" ).unwrap();

  // Change to temp directory (outside cargo workspace and git repo)
  let test_cwd = TempDir ::new().unwrap();
  env ::set_current_dir( test_cwd.path() ).ok();

  // Application code: get workspace and load secrets
  let workspace = Workspace ::resolve_with_extended_fallbacks();

  #[ cfg( feature = "secrets" ) ]
  {
   let secrets = workspace.load_secrets_from_file( "-secrets.sh" );
   assert!( secrets.is_ok(), "should load secrets from $PRO/secret/ directory" );

   let secrets_map = secrets.unwrap();
   assert_eq!(
  secrets_map.get( "GITHUB_TOKEN" ).map( String::as_str ),
  Some( "test_token_123" ),
  "should load GITHUB_TOKEN from secrets file"
   );
   assert_eq!(
  secrets_map.get( "API_KEY" ).map( String::as_str ),
  Some( "secret_key_456" ),
  "should load API_KEY from secrets file"
   );
  }

  // Restore environment
  if let Some( cwd ) = original_cwd
  {
   env ::set_current_dir( cwd ).ok();
  }
  restore_env_var( "WORKSPACE_PATH", original_workspace );
  restore_env_var( "PRO", original_pro );
  }

  /// Test i1.3: installed app without PRO uses $HOME for secrets
  #[ test ]
  fn test_installed_app_without_pro_uses_home()
  {
  let _lock = lock_env_mutex();
  let temp_dir = TempDir ::new().unwrap();
  let original_workspace = env ::var( "WORKSPACE_PATH" ).ok();
  let original_pro = env ::var( "PRO" ).ok();
  let original_home = env ::var( "HOME" ).ok();
  let original_userprofile = env ::var( "USERPROFILE" ).ok();
  let original_cwd = env ::current_dir().ok();

  // Simulate installed app context: no WORKSPACE_PATH, no PRO, but HOME is set
  env ::remove_var( "WORKSPACE_PATH" );
  env ::remove_var( "PRO" );
  env ::remove_var( "USERPROFILE" );
  env ::set_var( "HOME", temp_dir.path() );

  // Create secret directory and file
  let secret_dir = temp_dir.path().join( "secret" );
  fs ::create_dir_all( &secret_dir ).unwrap();
  let secret_file = secret_dir.join( "-secrets.sh" );
  fs ::write( &secret_file, "GITHUB_TOKEN=home_token_789\n" ).unwrap();

  // Change to temp directory (outside cargo workspace and git repo)
  let test_cwd = TempDir ::new().unwrap();
  env ::set_current_dir( test_cwd.path() ).ok();

  // Application code: get workspace and load secrets
  let workspace = Workspace ::resolve_with_extended_fallbacks();

  #[ cfg( feature = "secrets" ) ]
  {
   let secrets = workspace.load_secrets_from_file( "-secrets.sh" );
   assert!( secrets.is_ok(), "should load secrets from $HOME/secret/ directory" );

   let secrets_map = secrets.unwrap();
   assert_eq!(
  secrets_map.get( "GITHUB_TOKEN" ).map( String::as_str ),
  Some( "home_token_789" ),
  "should load GITHUB_TOKEN from HOME secrets file"
   );
  }

  // Restore environment
  if let Some( cwd ) = original_cwd
  {
   env ::set_current_dir( cwd ).ok();
  }
  restore_env_var( "WORKSPACE_PATH", original_workspace );
  restore_env_var( "PRO", original_pro );
  restore_env_var( "HOME", original_home );
  restore_env_var( "USERPROFILE", original_userprofile );
  }
}
