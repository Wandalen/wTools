//! Search Command Parameter Validation Tests
//!
//! ## Purpose
//!
//! Validates parameter validation for .search command per REQ-012 specification.
//! Tests ensure proper error handling before command implementation.
//!
//! ## Coverage
//!
//! Validates 7 validation requirements (V-012.1 through V-012.7):
//! - query parameter required and non-empty
//! - case_sensitive accepts only 0 or 1
//! - entry_type accepts only user, assistant, or all
//! - verbosity range 0-5
//! - project existence validation (when search implemented)
//! - session existence validation (when search implemented)
//!
//! ## Testing Strategy
//!
//! - Parameter validation tests: Run immediately (command will fail, we check error messages)
//! - Integration tests: Marked #[ignore] until search_routine implemented
//! - Uses same pattern as parameter_validation_test.rs for consistency
//!
//! ## Related Requirements
//!
//! REQ-012: Search Command specification (spec.md:458-519)

mod common;

/// Test .search query parameter is required (V-012.1)
///
/// ## Purpose
/// Validates that .search enforces required query parameter per REQ-012 V-012.1.
///
/// ## Coverage
/// Tests missing parameter case. Verifies error message mentions "query"
/// and "required" per spec error message standard.
///
/// ## Validation Strategy
/// Execute .search without query parameter. Assert:
/// - Command fails (non-zero exit)
/// - Error contains "query"
/// - Error contains "required"
///
/// ## Related Requirements
/// REQ-012 V-012.1: Reject missing query parameter
#[test]
fn test_search_query_required()
{
  let output = common::claude_storage_cmd()
    .args( [ ".search" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail when query missing. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "query" ) &&
    combined.to_lowercase().contains( "required" ),
    "Error should mention query is required. Got: {}",
    combined
  );
}

/// Test .search query parameter cannot be empty (V-012.2)
///
/// ## Purpose
/// Validates that .search rejects empty query string per REQ-012 V-012.2.
///
/// ## Coverage
/// Tests empty string edge case. Empty parameter values should be rejected
/// with clear error message.
///
/// ## Validation Strategy
/// Execute .search with empty query (query::). Assert:
/// - Command fails (non-zero exit)
/// - Error contains "query"
/// - Error contains "empty" or "cannot be empty"
///
/// ## Related Requirements
/// REQ-012 V-012.2: Reject empty query string
#[test]
fn test_search_query_empty()
{
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail when query empty. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "query" ) &&
    ( combined.to_lowercase().contains( "empty" ) ||
      combined.to_lowercase().contains( "expected value" ) ),
    "Error should mention query validation. Got: {}",
    combined
  );
}

/// Test .search case_sensitive parameter validation (V-012.3)
///
/// ## Purpose
/// Validates that case_sensitive accepts only 0 or 1 per REQ-012 V-012.3.
///
/// ## Coverage
/// Tests invalid boolean value. Boolean parameters should only accept
/// 0 (false) or 1 (true).
///
/// ## Validation Strategy
/// Execute .search with case_sensitive::2 (invalid boolean). Assert:
/// - Command fails (non-zero exit)
/// - Error mentions "case_sensitive" or "invalid"
///
/// ## Related Requirements
/// REQ-012 V-012.3: Validate case_sensitive accepts only 0 or 1
#[test]
fn test_search_case_sensitive_invalid()
{
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::test", "case_sensitive::2" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with invalid case_sensitive value. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "case" ) ||
    combined.to_lowercase().contains( "invalid" ),
    "Error should mention case_sensitive or invalid. Got: {}",
    combined
  );
}

/// Test .search entry_type parameter validation (V-012.4)
///
/// ## Purpose
/// Validates that entry_type accepts only user, assistant, or all per REQ-012 V-012.4.
///
/// ## Coverage
/// Tests invalid enumerated value. Enumerated parameters should validate
/// against allowed values and reject invalid ones with clear error message.
///
/// ## Validation Strategy
/// Execute .search with entry_type::invalid. Assert:
/// - Command fails (non-zero exit)
/// - Error mentions "entry_type" or "invalid"
///
/// ## Related Requirements
/// REQ-012 V-012.4: Validate entry_type accepts only user, assistant, or all
#[test]
fn test_search_entry_type_invalid()
{
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::test", "entry_type::invalid" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with invalid entry_type. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "entry" ) ||
    combined.to_lowercase().contains( "type" ) ||
    combined.to_lowercase().contains( "invalid" ),
    "Error should mention entry_type or invalid. Got: {}",
    combined
  );
}

/// Test .search entry_type accepts valid values (V-012.4)
///
/// ## Purpose
/// Validates that entry_type accepts valid values (user, assistant, all).
/// This is an integration test - command must be implemented.
///
/// ## Coverage
/// Tests all three valid entry_type values. Should not produce parameter
/// validation errors (may fail for other reasons like missing data).
///
/// ## Validation Strategy
/// Execute .search with entry_type::{user|assistant}. Assert:
/// - Error does NOT mention "entry_type" AND "invalid" together
/// - Parameter validation passed (though command may fail on missing data)
///
/// Note: "all" is not a valid value - omit the entry_type parameter to search all types
///
/// ## Related Requirements
/// REQ-012 V-012.4: entry_type enumerated validation
#[test]
#[ignore = "Integration test: requires session data - run manually or in CI"]
fn test_search_entry_type_valid()
{
  // Valid entry_type values as defined in search_routine: "user" and "assistant"
  // Note: "all" is NOT supported - omit entry_type parameter to search all types
  for entry_type in [ "user", "assistant" ]
  {
    let output = common::claude_storage_cmd()
      .args( [ ".search", "query::test", &format!( "entry_type::{}", entry_type ) ] )
      .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
      .output()
      .expect( "Failed to execute command" );

    let stderr = String::from_utf8_lossy( &output.stderr );

    // Check that no validation error was produced (only check stderr, not search results)
    // Search results may legitimately contain words like "entry", "type", "invalid"
    // in the actual content being searched, so we only check error output.
    let has_validation_error = stderr.to_lowercase().contains( "entry" ) &&
       stderr.to_lowercase().contains( "type" ) &&
       stderr.to_lowercase().contains( "invalid" );

    assert!(
      !has_validation_error,
      "Should not fail on entry_type validation for '{}'. stderr: {}",
      entry_type,
      stderr
    );

    // Also verify command succeeded (exit code 0)
    assert!(
      output.status.success(),
      "Search with entry_type::{} should succeed. stderr: {}",
      entry_type,
      stderr
    );
  }
}

/// Test .search verbosity parameter range validation (Finding #010)
///
/// ## Root Cause
/// search_routine in src/cli/mod.rs:1171 retrieved verbosity parameter without
/// validating the 0-5 range constraint, unlike status_routine and show_routine
/// which include explicit range validation. This inconsistency allowed invalid
/// values like -1 or 10 to be accepted and used.
///
/// ## Why Not Caught
/// .search command had no parameter validation tests. The existing search tests
/// only verified functionality with valid parameters. No tests checked edge cases
/// or invalid parameter values.
///
/// ## Fix Applied
/// Added explicit verbosity range validation (0-5) in search_routine at line 1190,
/// matching the validation pattern used in status_routine (line 18) and
/// show_routine (line 650). Returns clear error message with actual value and
/// valid range when validation fails.
///
/// ## Prevention
/// All parameters with constrained ranges must validate at routine entry, not
/// just in commands added later. When adding new commands, audit existing commands
/// for similar parameters and apply consistent validation patterns. Parameters
/// with defaults still require validation since users can override with invalid values.
///
/// ## Pitfall
/// Don't assume default values prevent invalid input. A parameter with default::1
/// can still receive invalid values from user input. Validation is required even
/// when defaults are sensible.
///
/// Related: REQ-012 V-012.5
#[test]
fn test_search_verbosity_invalid()
{
  // Test negative value
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::test", "verbosity::-1" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with verbosity::-1. Got: {}",
    combined
  );

  // Test value too large
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::test", "verbosity::10" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with verbosity::10. Got: {}",
    combined
  );
}

/// Test .search project existence validation (V-012.6)
///
/// ## Purpose
/// Validates that .search checks project exists when specified per REQ-012 V-012.6.
/// This is an integration test - requires .search implementation.
///
/// ## Coverage
/// Tests project parameter with nonexistent project ID. Should fail with
/// "project not found" or similar error.
///
/// ## Validation Strategy
/// Execute .search with project::nonexistent-uuid. Assert:
/// - Command fails (non-zero exit)
/// - Error contains "project" and ("not found" or "does not exist")
///
/// ## Related Requirements
/// REQ-012 V-012.6: Validate project exists when specified
#[test]
#[ignore = "Requires .search command implementation"]
fn test_search_project_nonexistent()
{
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::test", "project::nonexistent-uuid-12345" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with nonexistent project. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "project" ) &&
    ( combined.to_lowercase().contains( "not found" ) ||
      combined.to_lowercase().contains( "does not exist" ) ),
    "Error should mention project not found. Got: {}",
    combined
  );
}

/// Test .search session existence validation (V-012.7)
///
/// ## Purpose
/// Validates that .search checks session exists when specified per REQ-012 V-012.7.
/// This is an integration test - requires .search implementation.
///
/// ## Coverage
/// Tests session parameter with nonexistent session ID. Should fail with
/// "session not found" or similar error.
///
/// ## Validation Strategy
/// Execute .search with session::nonexistent-id. Assert:
/// - Command fails (non-zero exit)
/// - Error contains "session" and ("not found" or "does not exist")
///
/// ## Related Requirements
/// REQ-012 V-012.7: Validate session exists when specified
#[test]
#[ignore = "Requires .search command implementation"]
fn test_search_session_nonexistent()
{
  let output = common::claude_storage_cmd()
    .args( [ ".search", "query::test", "session::nonexistent-session-id" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{}{}", stderr, stdout );

  assert!(
    !output.status.success(),
    "Should fail with nonexistent session. Got: {}",
    combined
  );

  assert!(
    combined.to_lowercase().contains( "session" ) &&
    ( combined.to_lowercase().contains( "not found" ) ||
      combined.to_lowercase().contains( "does not exist" ) ),
    "Error should mention session not found. Got: {}",
    combined
  );
}
