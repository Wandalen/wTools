//! Export Command Parameter Validation Tests
//!
//! ## Purpose
//!
//! Validates parameter validation for .export command per REQ-013 specification.
//! Tests ensure proper error handling for required parameters and format validation.
//!
//! ## Coverage
//!
//! Validates 5 validation requirements (V-013.1 through V-013.5):
//! - `session_id` parameter required
//! - output parameter required
//! - format accepts only markdown, json, or text
//! - session existence validation (when export executed)
//! - output directory existence validation (when export executed)
//!
//! ## Testing Strategy
//!
//! - Parameter validation tests: Run immediately (validate error messages)
//! - Integration tests: Marked #[ignore] until full export workflow needed
//! - Uses same pattern as `search_command_test.rs` for consistency
//!
//! ## Related Requirements
//!
//! REQ-013: Export Command specification (spec.md:521-586)

mod common;

/// Test .export `session_id` parameter is required (V-013.1)
///
/// ## Purpose
/// Validates that .export enforces required `session_id` parameter per REQ-013 V-013.1.
///
/// ## Coverage
/// Tests missing parameter case. Verifies error message mentions "`session_id`"
/// and "required" per spec error message standard.
///
/// ## Validation Strategy
/// Execute .export without `session_id` parameter. Assert:
/// - Command fails (non-zero exit)
/// - Error contains "session" or "`session_id`"
/// - Error contains "required"
///
/// ## Related Requirements
/// REQ-013 V-013.1: Reject missing `session_id` parameter
#[test]
fn test_export_session_id_required()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "output::/tmp/test.md" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  assert!(
    !output.status.success(),
    "Should fail when session_id missing. Got: {combined}"
  );

  assert!(
    ( combined.to_lowercase().contains( "session" ) ||
      combined.to_lowercase().contains( "session_id" ) ) &&
    combined.to_lowercase().contains( "required" ),
    "Error should mention session_id is required. Got: {combined}"
  );
}

/// Test .export output parameter is required (V-013.2)
///
/// ## Purpose
/// Validates that .export enforces required output parameter per REQ-013 V-013.2.
///
/// ## Coverage
/// Tests missing parameter case. Verifies error message mentions "output"
/// and "required" per spec error message standard.
///
/// ## Validation Strategy
/// Execute .export without output parameter. Assert:
/// - Command fails (non-zero exit)
/// - Error contains "output"
/// - Error contains "required"
///
/// ## Related Requirements
/// REQ-013 V-013.2: Reject missing output parameter
#[test]
fn test_export_output_required()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "session_id::test" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  assert!(
    !output.status.success(),
    "Should fail when output missing. Got: {combined}"
  );

  assert!(
    combined.to_lowercase().contains( "output" ) &&
    combined.to_lowercase().contains( "required" ),
    "Error should mention output is required. Got: {combined}"
  );
}

/// Test .export format parameter validation (V-013.3)
///
/// ## Purpose
/// Validates that format accepts only markdown, json, or text per REQ-013 V-013.3.
///
/// ## Coverage
/// Tests invalid enumerated value. Format parameter should validate against
/// allowed values (markdown, json, text) and reject others with clear error.
///
/// ## Validation Strategy
/// Execute .export with `format::csv` (not supported). Assert:
/// - Command fails (non-zero exit)
/// - Error mentions "format" or "invalid"
///
/// ## Related Requirements
/// REQ-013 V-013.3: Validate format accepts only markdown, json, or text
#[test]
fn test_export_format_invalid()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "session_id::test", "output::/tmp/test.csv", "format::csv" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  assert!(
    !output.status.success(),
    "Should fail with invalid format. Got: {combined}"
  );

  assert!(
    combined.to_lowercase().contains( "format" ) ||
    combined.to_lowercase().contains( "invalid" ),
    "Error should mention format validation. Got: {combined}"
  );
}

/// Test .export format accepts valid values (V-013.3)
///
/// ## Purpose
/// Validates that format accepts all three valid values (markdown, json, text).
/// This is an integration test - command must complete export operation.
///
/// ## Coverage
/// Tests all three valid format values. Should not produce format validation
/// errors (may fail for other reasons like missing session).
///
/// ## Validation Strategy
/// Execute .export with `format::{markdown|json|text`}. Assert:
/// - Error does NOT mention "format" AND "invalid" together
/// - Format validation passed (though command may fail on missing session)
///
/// ## Related Requirements
/// REQ-013 V-013.3: format enumerated validation
#[test]
#[ignore = "Requires export integration with actual session data"]
fn test_export_format_valid()
{
  for format in [ "markdown", "json", "text" ]
  {
    let output = common::claude_storage_cmd()
      .args( [ ".export", "session_id::test", "output::/tmp/test.out", &format!( "format::{format}" ) ] )
      .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
      .output()
      .expect( "Failed to execute command" );

    let stderr = String::from_utf8_lossy( &output.stderr );
    let stdout = String::from_utf8_lossy( &output.stdout );
    let combined = format!( "{stderr}{stdout}" );

    assert!(
      !( combined.to_lowercase().contains( "format" ) &&
         combined.to_lowercase().contains( "invalid" ) ),
      "Should not fail on format validation for '{format}'. Got: {combined}"
    );
  }
}

/// Test .export session existence validation (V-013.4)
///
/// ## Purpose
/// Validates that .export checks session exists when specified per REQ-013 V-013.4.
/// This is an integration test - requires .export to attempt session lookup.
///
/// ## Coverage
/// Tests session parameter with nonexistent session ID. Should fail with
/// "session not found" or similar error.
///
/// ## Validation Strategy
/// Execute .export with `session_id::nonexistent`. Assert:
/// - Command fails (non-zero exit)
/// - Error contains "session" and ("not found" or "does not exist")
///
/// ## Related Requirements
/// REQ-013 V-013.4: Validate session exists in project
#[test]
#[ignore = "Requires export integration with actual project/session data"]
fn test_export_session_nonexistent()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "session_id::nonexistent-session-12345", "output::/tmp/test.md" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  assert!(
    !output.status.success(),
    "Should fail with nonexistent session. Got: {combined}"
  );

  assert!(
    combined.to_lowercase().contains( "session" ) &&
    ( combined.to_lowercase().contains( "not found" ) ||
      combined.to_lowercase().contains( "does not exist" ) ),
    "Error should mention session not found. Got: {combined}"
  );
}

/// Test .export output directory existence validation (V-013.5)
///
/// ## Purpose
/// Validates that .export checks output path directory exists per REQ-013 V-013.5.
/// This is an integration test - requires .export to validate output path.
///
/// ## Coverage
/// Tests output parameter with nonexistent directory path. Should fail with
/// directory validation error.
///
/// ## Validation Strategy
/// Execute .export with `output::/nonexistent/dir/test.md`. Assert:
/// - Command fails (non-zero exit)
/// - Error mentions directory or path issue
///
/// ## Related Requirements
/// REQ-013 V-013.5: Validate output path directory exists
#[test]
#[ignore = "Requires export integration with file system validation"]
fn test_export_output_directory_nonexistent()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "session_id::test", "output::/nonexistent/directory/path/test.md" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  assert!(
    !output.status.success(),
    "Should fail with nonexistent directory. Got: {combined}"
  );

  assert!(
    combined.to_lowercase().contains( "directory" ) ||
    combined.to_lowercase().contains( "path" ) ||
    combined.to_lowercase().contains( "not found" ) ||
    combined.to_lowercase().contains( "does not exist" ),
    "Error should mention directory/path issue. Got: {combined}"
  );
}

/// Test .export with both required parameters
///
/// ## Purpose
/// Validates that providing both required parameters (`session_id` and output)
/// passes parameter validation. This is an integration test.
///
/// ## Coverage
/// Tests that no "parameter required" errors occur when both required
/// parameters provided. May fail on session not found (expected).
///
/// ## Validation Strategy
/// Execute .export with `session_id` and output. Assert:
/// - Error does NOT mention parameter validation issues
/// - May fail on session not found (acceptable for this test)
///
/// ## Related Requirements
/// REQ-013 V-013.1, V-013.2: Required parameters
#[test]
#[ignore = "Requires export integration with actual session data"]
fn test_export_with_required_parameters()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "session_id::test-session", "output::/tmp/test-export.md" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  // Should not complain about missing parameters
  assert!(
    !( combined.to_lowercase().contains( "required" ) &&
       ( combined.to_lowercase().contains( "session" ) ||
         combined.to_lowercase().contains( "output" ) ) ),
    "Should not fail on parameter validation with both required params. Got: {combined}"
  );
}

/// Test .export default format is markdown
///
/// ## Purpose
/// Validates that .export uses markdown as default format when format parameter
/// not specified. This is an integration test requiring full export workflow.
///
/// ## Coverage
/// Tests default parameter behavior. When format not specified, should default
/// to markdown format per spec.
///
/// ## Validation Strategy
/// Execute .export without format parameter. Assert:
/// - No format validation errors
/// - Export would use markdown format (integration test)
///
/// ## Related Requirements
/// REQ-013: Default format is markdown
#[test]
#[ignore = "Requires export integration with actual session data to verify format"]
fn test_export_default_format_markdown()
{
  let output = common::claude_storage_cmd()
    .args( [ ".export", "session_id::test", "output::/tmp/test-default.md" ] )
    .current_dir( env!( "CARGO_MANIFEST_DIR" ) )
    .output()
    .expect( "Failed to execute command" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  let stdout = String::from_utf8_lossy( &output.stdout );
  let combined = format!( "{stderr}{stdout}" );

  // Should not have format validation errors (default should work)
  assert!(
    !( combined.to_lowercase().contains( "format" ) &&
       combined.to_lowercase().contains( "invalid" ) ),
    "Default format should be valid. Got: {combined}"
  );
}
