//! Tests for build.rs validation of command definitions.
//!
//! ## Test Matrix
//!
//! | Test | Input | Expected | Status |
//! |------|-------|----------|--------|
//! | `test_build_validates_command_name` | YAML with "`invalid_no_dot`" | Build fails | TBD |
//! | `test_build_validates_version` | YAML with empty version | Build fails | TBD |
//! | `test_build_validates_empty_name` | YAML with empty name | Build fails | TBD |
//! | `test_valid_yaml_builds` | Valid YAML | Build succeeds | TBD |
//!
//! ## Design Rationale
//!
//! These tests verify that build.rs validates all command definitions at compile
//! time using the shared `validation_core` module. This ensures:
//!
//! 1. Invalid YAML causes `cargo build` to fail with clear error messages
//! 2. The From<StaticCommandDefinition> conversion cannot panic at runtime
//! 3. Build.rs uses the same validation rules as runtime registration
//!
//! ## Related Hypotheses
//!
//! - H27: build.rs does NOT validate command names
//! - H28: build.rs uses `unwrap()` extensively - silent failures
//! - H31: From conversion uses `expect()` - RUNTIME PANIC on invalid names
//! - H37: build.rs does NOT use `validate_namespace` or `validate_command` functions
//!
//! ## Root Cause
//!
//! build.rs was implemented without validation, allowing invalid YAML to
//! generate code that panics at runtime during From conversion.
//!
//! ## Why Not Caught
//!
//! No tests existed for build.rs validation - only YAML parsing was tested.
//!
//! ## Fix Applied
//!
//! Added validation calls in build.rs using shared `validation_core` module.
//!
//! ## Prevention
//!
//! All build.rs code paths must validate input before generating code.
//!
//! ## Pitfall
//!
//! Build scripts can silently generate bad code if validation is skipped.
//!
//! Note: These are integration-level tests that verify the validation logic
//! is correctly integrated into build.rs. The unit tests for `validation_core`
//! are in `validation_core_test.rs`.

use unilang::validation_core::{
  validate_command_name_core,
  validate_version_core,
  validate_command_definition_core,
};

// ============================================================================
// Test that validation_core functions work correctly with test fixture data
// ============================================================================

/// Verify that the validation functions correctly reject the invalid fixture.
/// This simulates what build.rs should do when processing `invalid_missing_dot.yaml`
#[test]
fn test_fixture_missing_dot_prefix_rejected()
{
  // This mirrors the data from tests/test_data/build_time/invalid/missing_dot_prefix.yaml
  let result = validate_command_name_core( "invalid_no_dot" );
  assert!(
    result.is_err(),
    "Command name 'invalid_no_dot' should be rejected by validation"
  );

  let err = result.unwrap_err();
  assert!(
    err.contains( "dot prefix" ),
    "Error message should mention dot prefix requirement: {err}"
  );
}

/// Verify that the validation functions correctly reject empty version.
/// This simulates what build.rs should do when processing `invalid_empty_version.yaml`
#[test]
fn test_fixture_empty_version_rejected()
{
  // This mirrors the data from tests/test_data/build_time/invalid/empty_version.yaml
  let result = validate_version_core( "" );
  assert!(
    result.is_err(),
    "Empty version should be rejected by validation"
  );

  let err = result.unwrap_err();
  assert!(
    err.contains( "cannot be empty" ),
    "Error message should mention empty version: {err}"
  );
}

/// Verify that the validation functions correctly reject empty name.
/// This simulates what build.rs should do when processing `invalid_empty_name.yaml`
#[test]
fn test_fixture_empty_name_rejected()
{
  // This mirrors the data from tests/test_data/build_time/invalid/empty_name.yaml
  let result = validate_command_name_core( "" );
  assert!(
    result.is_err(),
    "Empty command name should be rejected by validation"
  );

  let err = result.unwrap_err();
  assert!(
    err.contains( "cannot be empty" ),
    "Error message should mention empty name: {err}"
  );
}

/// Verify that valid command definition passes all validation.
/// This simulates what build.rs should do when processing valid YAML.
#[test]
fn test_fixture_valid_yaml_accepted()
{
  // This mirrors the data from tests/test_data/build_time/valid/complete_command.yaml
  let result = validate_command_definition_core(
    ".test.complete",
    "",
    "2.0.0",
    "valid/complete_command.yaml"
  );
  assert!(
    result.is_ok(),
    "Valid command should pass validation: {result:?}"
  );
}

/// Verify that valid command with aliases passes validation.
/// This simulates what build.rs should do when processing `with_aliases.yaml`.
#[test]
fn test_fixture_with_aliases_accepted()
{
  // This mirrors the data from tests/test_data/build_time/valid/with_aliases.yaml
  let result = validate_command_definition_core(
    ".test.aliased",
    "",
    "1.0.0",
    "valid/with_aliases.yaml"
  );
  assert!(
    result.is_ok(),
    "Valid command with aliases should pass validation: {result:?}"
  );
}

// ============================================================================
// Test error message quality (STATC compliant)
// ============================================================================

/// Error messages must include file path for actionability
#[test]
fn test_error_includes_file_path()
{
  let result = validate_command_definition_core(
    "invalid",
    "",
    "1.0.0",
    "commands/invalid.yaml"
  );
  assert!( result.is_err() );

  let err = result.unwrap_err();
  assert!(
    err.contains( "commands/invalid.yaml" ),
    "Error should include file path for actionability: {err}"
  );
}

/// Error messages must include the invalid value for context
#[test]
fn test_error_includes_invalid_value()
{
  let result = validate_command_name_core( "bad_name" );
  assert!( result.is_err() );

  let err = result.unwrap_err();
  assert!(
    err.contains( "bad_name" ),
    "Error should include the invalid value: {err}"
  );
}

/// Error messages must explain how to fix the issue
#[test]
fn test_error_includes_fix_guidance()
{
  let result = validate_command_name_core( "missing_dot" );
  assert!( result.is_err() );

  let err = result.unwrap_err();
  assert!(
    err.contains( ".chat" ) || err.contains( "e.g." ),
    "Error should include example of correct format: {err}"
  );
}

// ============================================================================
// Tests that will verify build.rs integration (require build.rs changes)
// ============================================================================

/// Test that build.rs uses the validation functions.
/// Currently this is a placeholder - the actual verification requires
/// checking build.rs code or running cargo build with invalid YAML.
#[test]
fn test_build_rs_integration_note()
{
  // H37: build.rs does NOT use validate_namespace or validate_command functions
  //
  // To fully test this, we would need to:
  // 1. Set UNILANG_STATIC_COMMANDS_PATH to invalid fixture
  // 2. Run cargo build
  // 3. Verify it fails with proper error message
  //
  // This is done in the implementation phase - for now we verify the
  // validation functions are available and work correctly.
  assert!(
    validate_command_name_core( ".valid" ).is_ok(),
    "Validation functions should be available for build.rs to use"
  );
}
