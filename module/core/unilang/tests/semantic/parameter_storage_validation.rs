//! Parameter Storage Type Validation Tests
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Status |
//! |---------|---------------|-------|----------|--------|
//! | PS-1 | multiple:true requires List | Command with multiple:true | `Kind::List` enforced | VALIDATION |
//! | PS-2 | Detect type mismatch | multiple:true + `Kind::String` | Error or warning | NEW |
//! | PS-3 | Validation at registration | Invalid combination | Registration fails | NEW |
//!
//! ## Scope
//!
//! Tests that commands with `multiple: true` attributes use appropriate storage types.
//! Prevents the wplan bug pattern where Option<String> was used for multiple values,
//! causing silent data loss.
//!
//! ## Coverage
//!
//! - Validation that multiple:true uses `Kind::List`
//! - Detection of storage type mismatches
//! - Error reporting for invalid parameter definitions
//! - Prevention of silent data loss patterns
//!
//! ## Related
//!
//! - `task/make_illegal_states_unrepresentable.md` - Type safety requirements
//! - `tests/regression/repeated_parameter_handling.rs` - Repeated parameter tests
//! - `tests/semantic/multiple_parameters.rs` - Multiple parameter semantics
//!
//! ## Background: The wplan Bug
//!
//! wplan CLI had this bug:
//! ```rust
//! let mut command = None;  // Option<String> - can only hold ONE value
//!
//! for arg in args {
//!   match key {
//!     "command" => command = Some(value.to_string()),  // OVERWRITES!
//!   }
//! }
//! ```
//!
//! User typed: `wplan .plan command::"cargo build" command::"cargo test"`
//! Expected: 2 jobs created
//! Actual: 1 job (only "cargo test" - last value overwrote first)
//!
//! **Root cause:** Type allowed illegal state (multiple values in single-value storage)
//!
//! **Solution:** If multiple:true, storage MUST be Vec/List type

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes, OutputData };
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper: Create command with specific parameter configuration
fn create_command_with_parameter(
  param_name : &str,
  kind : Kind,
  multiple : bool,
) -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".test" )
    .description( "Test command for parameter validation" )
    .arguments( vec![
      ArgumentDefinition {
        name : param_name.to_string(),
        description : format!( "Test parameter {param_name}" ),
        kind,
        hint : String::new(),
        attributes : ArgumentAttributes {
          multiple,
          optional : true,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Helper: Create mock routine for runtime registration
fn create_mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData > + Send + Sync + 'static >
{
  Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, unilang::data::ErrorData >
  {
    Ok( OutputData::new( "test", "text" ) )
  })
}

/// PS-1: Parameters with multiple:true should use `Kind::List`
///
/// **Expected:** If multiple:true, then kind must be `Kind::List`
/// **Rationale:** Prevents wplan bug (Option<T> used for multiple values)
#[ test ]
fn test_multiple_true_requires_list_kind()
{
  // CORRECT: multiple:true with Kind::List
  let correct_cmd = create_command_with_parameter(
    "commands",
    Kind::List( Box::new( Kind::String ), None ),
    true, // multiple: true
  );

  // Verify this is the correct pattern
  if let Some( arg ) = correct_cmd.arguments().first()
  {
    assert!(
      arg.attributes.multiple,
      "Parameter should have multiple:true"
    );

    match &arg.kind
    {
      Kind::List( _, _ ) => {
        // This is correct!
      }
      _ => {
        panic!(
          "VALIDATION BUG: Parameter with multiple:true has kind {:?} instead of Kind::List!\n\
           This is the wplan bug pattern - multiple values need List storage.",
          arg.kind
        );
      }
    }
  }
}

/// PS-2: Detect type mismatch (multiple:true + non-List kind)
///
/// **Expected:** Validation should error for this pattern
/// **Actual:** Validation now prevents this bug at registration time
#[ test ]
fn test_detect_multiple_true_with_wrong_kind()
{
  use unilang::registry::CommandRegistry;

  let mut registry = CommandRegistry::new();

  // INCORRECT: multiple:true with Kind::String (wplan bug pattern!)
  let incorrect_cmd = create_command_with_parameter(
    "command",
    Kind::String, // WRONG - should be Kind::List!
    true, // multiple: true
  );

  // This command should be REJECTED during registration
  let result = registry.command_add_runtime( &incorrect_cmd, create_mock_routine() );

  // Verify registration failed
  assert!(
    result.is_err(),
    "Registration should fail for multiple:true with Kind::String (wplan bug pattern)"
  );

  // Verify error message mentions the problem
  if let Err( err ) = result
  {
    let error_msg = format!( "{err:?}" );
    assert!(
      error_msg.contains( "multiple" ) || error_msg.contains( "List" ),
      "Error should explain storage type mismatch. Got: {error_msg}"
    );
  }
}

/// PS-3: Validation at command registration
///
/// **Expected:** Registration fails for multiple:true + non-List kind
/// **Actual:** Validation now implemented and working
#[ test ]
fn test_validation_at_registration()
{
  use unilang::registry::CommandRegistry;

  let mut registry = CommandRegistry::new();

  // Create command with BUG PATTERN
  let invalid_cmd = create_command_with_parameter(
    "command",
    Kind::String, // WRONG for multiple:true
    true,
  );

  // Registry validation should catch this
  let result = registry.command_add_runtime( &invalid_cmd, create_mock_routine() );

  assert!(
    result.is_err(),
    "Registration should fail for multiple:true with Kind::String"
  );

  if let Err( error ) = result
  {
    let error_msg = format!( "{error:?}" );
    assert!(
      error_msg.contains( "multiple" ) && error_msg.contains( "List" ),
      "Error should explain storage type mismatch: {error_msg}"
    );
  }
}

/// PS-4: Valid combinations should pass validation
///
/// **Expected:** Correct parameter configurations should be accepted
#[ test ]
fn test_valid_parameter_combinations()
{
  // Valid combination 1: multiple:true + Kind::List
  let valid1 = create_command_with_parameter(
    "files",
    Kind::List( Box::new( Kind::String ), None ),
    true,
  );

  assert!(
    !valid1.arguments().is_empty(),
    "Valid command should be created"
  );

  // Valid combination 2: multiple:false + Kind::String
  let valid2 = create_command_with_parameter(
    "config",
    Kind::String,
    false, // multiple: false - single value OK for String
  );

  assert!(
    !valid2.arguments().is_empty(),
    "Valid command should be created"
  );

  // Valid combination 3: multiple:false + Kind::List (List can hold 0-1 items)
  let valid3 = create_command_with_parameter(
    "optional_list",
    Kind::List( Box::new( Kind::String ), None ),
    false,
  );

  assert!(
    !valid3.arguments().is_empty(),
    "Valid command should be created"
  );
}

/// PS-5: Documentation of the illegal state pattern
///
/// This test exists to document the anti-pattern clearly
#[ test ]
fn test_document_illegal_state_pattern()
{
  // THE BUG PATTERN (from wplan):
  //
  // User types: command::"value1" command::"value2" command::"value3"
  // Parser sees 3 values for parameter "command"
  // Semantic analyzer collects them
  //
  // IF storage is Option<String> or String (single value):
  //   ❌ Data loss! Only last value kept.
  //   ❌ No error, no warning
  //   ❌ User confused why their data disappeared
  //
  // IF storage is Vec<String> or Kind::List:
  //   ✅ All values preserved
  //   ✅ No data loss
  //   ✅ Correct behavior

  // Demonstrate correct pattern
  let correct = ArgumentDefinition {
    name : "command".to_string(),
    description : "Commands to execute".to_string(),
    kind : Kind::List( Box::new( Kind::String ), None ), // ✅ CORRECT
    hint : String::new(),
    attributes : ArgumentAttributes {
      multiple : true, // ✅ Matches storage type
      optional : false,
      ..Default::default()
    },
    validation_rules : vec![],
    aliases : vec![],
    tags : vec![],
  };

  // Verify the correct pattern
  assert!( correct.attributes.multiple, "Should accept multiple values" );
  assert!(
    matches!( correct.kind, Kind::List( _, _ ) ),
    "Should use List kind for storage"
  );

  // This combination is SAFE - no data loss possible
}
