//! Integration test for path resolution in list command
//!
//! Verifies that `claude_storage .list path::.` correctly finds projects
//! in the current working directory using the smart path resolution.

mod common;

use std::env;

/// Test: .list `path::`. finds projects in current directory
#[test]
fn test_list_path_dot_integration()
{
  // Change to claude_storage directory (where this project's storage should be)
  let test_dir = env::current_dir().unwrap();

  // Run: cargo run -- .list path::.
  let output = common::claude_storage_cmd()
    .args( [ ".list", "path::." ] )
    .current_dir( &test_dir )
    .output()
    .expect( "Failed to execute claude_storage" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  // Debug output for troubleshooting
  if !output.status.success()
  {
    eprintln!( "Command failed:" );
    eprintln!( "stdout: {stdout}" );
    eprintln!( "stderr: {stderr}" );
  }

  // Command should succeed
  assert!
  (
    output.status.success(),
    "Command should succeed, got:\nstdout: {stdout}\nstderr: {stderr}"
  );

  // Should resolve "." to current directory and perform substring matching
  // The exact output depends on whether a project exists, but it should not error
  assert!
  (
    !stdout.contains( "Failed to resolve path parameter" ),
    "Should not fail path resolution: {stdout}"
  );
}

/// Test: .list `path::`.. finds projects in parent directory
#[test]
fn test_list_path_dotdot_integration()
{
  let test_dir = env::current_dir().unwrap();

  let output = common::claude_storage_cmd()
    .args( [ ".list", "path::.." ] )
    .current_dir( &test_dir )
    .output()
    .expect( "Failed to execute claude_storage" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!
  (
    output.status.success(),
    "Command should succeed, got:\nstdout: {stdout}\nstderr: {stderr}"
  );

  assert!
  (
    !stdout.contains( "Failed to resolve path parameter" ),
    "Should not fail path resolution: {stdout}"
  );
}

/// Test: .list `path::`~ finds projects in home directory
#[test]
fn test_list_path_tilde_integration()
{
  let test_dir = env::current_dir().unwrap();

  let output = common::claude_storage_cmd()
    .args( [ ".list", "path::~" ] )
    .current_dir( &test_dir )
    .output()
    .expect( "Failed to execute claude_storage" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!
  (
    output.status.success(),
    "Command should succeed, got:\nstdout: {stdout}\nstderr: {stderr}"
  );

  assert!
  (
    !stdout.contains( "Failed to resolve path parameter" ),
    "Should not fail path resolution: {stdout}"
  );
}

/// Test: .list `path::willbe` still works (backward compatibility)
#[test]
fn test_list_path_pattern_backward_compat()
{
  let test_dir = env::current_dir().unwrap();

  // Verify backward compatibility: pattern matching still works
  let output = common::claude_storage_cmd()
    .args( [ ".list", "path::willbe" ] )
    .current_dir( &test_dir )
    .output()
    .expect( "Failed to execute claude_storage" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!
  (
    output.status.success(),
    "Command should succeed, got:\nstdout: {stdout}\nstderr: {stderr}"
  );

  // Should use pattern matching (no path resolution)
  assert!
  (
    !stdout.contains( "Failed to resolve path parameter" ),
    "Should not fail on pattern: {stdout}"
  );
}

/// Test: .list `path::`~/pro/lib (home + relative path)
#[test]
fn test_list_path_tilde_slash_integration()
{
  let test_dir = env::current_dir().unwrap();

  let output = common::claude_storage_cmd()
    .args( [ ".list", "path::~/pro" ] )
    .current_dir( &test_dir )
    .output()
    .expect( "Failed to execute claude_storage" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!
  (
    output.status.success(),
    "Command should succeed, got:\nstdout: {stdout}\nstderr: {stderr}"
  );

  assert!
  (
    !stdout.contains( "Failed to resolve path parameter" ),
    "Should not fail path resolution: {stdout}"
  );
}

/// Test: .list `path::/absolute/path` works
#[test]
fn test_list_path_absolute_integration()
{
  let test_dir = env::current_dir().unwrap();

  // Use current directory as absolute path
  let abs_path = test_dir.to_string_lossy().to_string();

  let output = common::claude_storage_cmd()
    .args( [ ".list", &format!( "path::{abs_path}" ) ] )
    .current_dir( &test_dir )
    .output()
    .expect( "Failed to execute claude_storage" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!
  (
    output.status.success(),
    "Command should succeed, got:\nstdout: {stdout}\nstderr: {stderr}"
  );

  assert!
  (
    !stdout.contains( "Failed to resolve path parameter" ),
    "Should not fail path resolution: {stdout}"
  );
}
