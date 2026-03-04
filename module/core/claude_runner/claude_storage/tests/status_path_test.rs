//! Status Command Path Parameter Tests
//!
//! ## Purpose
//!
//! Validates path parameter functionality for .status command.
//! Tests ensure proper handling of default path, custom paths, and error cases.
//!
//! ## Coverage
//!
//! Validates path parameter behavior:
//! - Default path usage (no parameter specified)
//! - Custom path support (path:: parameter)
//! - Nonexistent path handling (error case)
//! - Empty path validation (error case)
//! - Path with verbosity interaction (parameter combination)
//!
//! ## Testing Strategy
//!
//! - Feature tests: Run immediately (verify existing path parameter functionality)
//! - Uses real filesystem with tempfile crate for integration testing
//! - Follows same pattern as search_command_test.rs and export_command_test.rs
//!
//! ## Related Requirements
//!
//! .status path parameter documentation (spec.md:272-276)

mod common;

use std::fs;
use tempfile::TempDir;

/// Test .status uses default path when no path parameter specified
///
/// ## Purpose
/// Validates that .status command works with default ~/.claude/ path
/// when path parameter is not provided.
///
/// ## Coverage
/// Tests default parameter behavior. Should successfully execute using
/// default storage location without requiring explicit path parameter.
///
/// ## Validation Strategy
/// Execute .status without path parameter. Assert:
/// - Command succeeds (zero exit)
/// - Output contains expected status information
///
/// ## Related Requirements
/// .status path parameter: default behavior uses ~/.claude/
#[test]
#[ignore = "Integration test - depends on actual ~/.claude/ storage state"]
fn test_status_default_path()
{
  let output = common::claude_storage_cmd()
    .args( [ ".status" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    output.status.success(),
    "Should succeed with default path. Got: {}",
    combined
  );

  // Should show storage information (projects count, etc.)
  assert!(
    combined.to_lowercase().contains( "project" ) ||
    combined.to_lowercase().contains( "storage" ),
    "Should show storage information. Got: {}",
    combined
  );
}

/// Test .status accepts custom path parameter
///
/// ## Purpose
/// Validates that .status command can use custom storage path via path:: parameter.
///
/// ## Coverage
/// Tests custom path parameter functionality. Should successfully execute
/// using specified storage location instead of default.
///
/// ## Validation Strategy
/// Setup: Create temp directory with projects/ subdirectory structure
/// Execute .status with path::{temp_dir}. Assert:
/// - Command succeeds (zero exit)
/// - Output shows 0 projects (empty storage)
///
/// ## Related Requirements
/// .status path parameter: accepts custom storage location
#[test]
fn test_status_custom_path()
{
  // Create temp directory structure
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  let storage_path = temp_dir.path();
  let projects_dir = storage_path.join( "projects" );
  fs::create_dir_all( &projects_dir ).expect( "Failed to create projects dir" );

  let output = common::claude_storage_cmd()
    .args( [ ".status", &format!( "path::{}", storage_path.display() ) ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    output.status.success(),
    "Should succeed with custom path. Got: {}",
    combined
  );

  // Should report 0 projects in empty storage
  assert!(
    combined.contains( "0" ) && combined.to_lowercase().contains( "project" ),
    "Should show 0 projects in empty storage. Got: {}",
    combined
  );
}

/// Test .status handles nonexistent path gracefully
///
/// ## Purpose
/// Validates that .status command succeeds with nonexistent path and reports
/// empty storage (0 projects) rather than failing.
///
/// ## Coverage
/// Tests graceful handling of nonexistent paths. Command should succeed and
/// report 0 projects for nonexistent paths, allowing users to check status
/// of new/empty storage locations.
///
/// ## Validation Strategy
/// Execute .status with path::/nonexistent/path/12345. Assert:
/// - Command succeeds (zero exit)
/// - Reports 0 projects (empty storage)
///
/// ## Related Requirements
/// .status path parameter: gracefully handles nonexistent paths
#[test]
fn test_status_nonexistent_path()
{
  let output = common::claude_storage_cmd()
    .args( [ ".status", "path::/nonexistent/path/12345" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    output.status.success(),
    "Should succeed with nonexistent path. Got: {}",
    combined
  );

  // Should report 0 projects for nonexistent path
  assert!(
    combined.contains( "0" ) && combined.to_lowercase().contains( "project" ),
    "Should show 0 projects for nonexistent path. Got: {}",
    combined
  );
}

/// Test .status rejects empty path parameter
///
/// ## Purpose
/// Validates that .status command rejects empty path parameter value.
///
/// ## Coverage
/// Tests empty string edge case. Empty parameter values should be rejected
/// with clear error message.
///
/// ## Validation Strategy
/// Execute .status with path:: (empty value). Assert:
/// - Command fails (non-zero exit)
/// - Error mentions "path" or "expected value"
///
/// ## Related Requirements
/// .status path parameter: rejects empty path values
#[test]
fn test_status_empty_path()
{
  let output = common::claude_storage_cmd()
    .args( [ ".status", "path::" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with empty path. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "path" ) ||
    combined.to_lowercase().contains( "expected value" ),
    "Error should mention path validation. Got: {}",
    combined
  );
}

/// Test .status path::. resolves to current directory (Finding #014)
///
/// ## Root Cause
/// status_routine at line 74 passes path directly to Storage::with_root() without
/// resolving special path markers. While list_routine uses resolve_path_parameter(),
/// status_routine does not, causing ".", "..", "~" to be used literally.
///
/// ## Why Not Caught
/// Existing tests used only explicit full paths or temp directories. No tests
/// exercised special path markers (".", "..", "~") in the path parameter.
///
/// ## Fix Applied
/// Added resolve_path_parameter() call in status_routine before passing to
/// Storage::with_root(), consistent with list_routine pattern.
///
/// ## Prevention
/// When multiple commands share similar parameters, ensure they use the same
/// helper functions. The resolve_path_parameter() helper exists specifically
/// for this purpose but was not consistently applied.
///
/// ## Pitfall
/// Adding new commands by copying existing code without understanding shared
/// utilities leads to inconsistent behavior between commands.
#[test]
fn test_status_path_dot_resolves_to_cwd()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = common::claude_storage_cmd()
    .args( [ ".status", "path::." ] )
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute command" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );
  let combined = format!( "{}{}", stdout, stderr );

  // Bug behavior: Shows Storage: "."
  // Fixed behavior: Shows resolved path like Storage: "/home/.../claude_storage"

  let has_literal_dot = combined.contains( r#"Storage: ".""# );

  assert!(
    !has_literal_dot,
    "Bug: path::. not resolved, shows literal '.' in Storage.\n\
    Expected: Resolved absolute path\n\
    Got: {}",
    combined
  );
}

/// Test .status path::~ resolves to home directory (Finding #014)
///
/// ## Purpose
/// Validates that status_routine resolves ~ to home directory.
#[test]
fn test_status_path_tilde_resolves_to_home()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = common::claude_storage_cmd()
    .args( [ ".status", "path::~" ] )
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute command" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );
  let combined = format!( "{}{}", stdout, stderr );

  // Bug behavior: Shows Storage: "~"
  // Fixed behavior: Shows resolved path like Storage: "/home/user"

  let has_literal_tilde = combined.contains( r#"Storage: "~""# );

  assert!(
    !has_literal_tilde,
    "Bug: path::~ not resolved, shows literal '~' in Storage.\n\
    Expected: Resolved home directory path\n\
    Got: {}",
    combined
  );
}

/// Test .status path parameter works with verbosity parameter
///
/// ## Purpose
/// Validates that .status command correctly handles path and verbosity
/// parameters together, ensuring no parameter interaction issues.
///
/// ## Coverage
/// Tests parameter combination. Path and verbosity should work independently
/// without conflicts.
///
/// ## Validation Strategy
/// Setup: Create temp directory with projects/ subdirectory
/// Execute .status with path::{temp} and verbosity::2. Assert:
/// - Command succeeds (zero exit)
/// - Output shows storage information
/// - Both parameters are respected
///
/// ## Related Requirements
/// .status path parameter: works with other parameters
#[test]
fn test_status_path_with_verbosity()
{
  // Create temp directory structure
  let temp_dir = TempDir::new().expect( "Failed to create temp dir" );
  let storage_path = temp_dir.path();
  let projects_dir = storage_path.join( "projects" );
  fs::create_dir_all( &projects_dir ).expect( "Failed to create projects dir" );

  let output = common::claude_storage_cmd()
    .args( [ ".status", &format!( "path::{}", storage_path.display() ), "verbosity::2" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    output.status.success(),
    "Should succeed with path and verbosity. Got: {}",
    combined
  );

  // Should show storage information
  assert!(
    combined.to_lowercase().contains( "project" ) ||
    combined.to_lowercase().contains( "storage" ),
    "Should show storage information. Got: {}",
    combined
  );
}
