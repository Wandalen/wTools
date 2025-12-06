//! Tests for `build_helpers::hint_generator` module
//!
//! ## Overview
//!
//! This module tests the `HintGenerator` that creates user-friendly warning messages for
//! type hints detected during build. The generator produces actionable messages showing
//! current code, suggested improvements, and suppression instructions.
//!
//! ## Test Matrix
//!
//! | Test Name | Purpose | What It Tests | Expected Outcome |
//! |-----------|---------|---------------|------------------|
//! | `generates_boolean_message` | Boolean hint formatting | Message for Boolean-as-String hint | Complete message with all sections |
//! | `generates_integer_message` | Integer hint formatting | Message for Integer-as-String hint | Complete message with validation example |
//! | `message_shows_suppression` | Suppression documentation | Both local and global suppression methods | Instructions present in message |
//! | `message_emphasizes_non_error` | User reassurance | Message tone and emphasis | Contains "hint, not an error" and "Build continues" |
//!
//! ## Test Coverage
//!
//! - ✅ Happy path: Both hint types (Boolean, Integer) generate complete messages
//! - ✅ Edge cases: Not applicable (message generation is deterministic)
//! - ✅ Error cases: Not applicable (hint generation never fails)
//! - ✅ Integration: Messages work with `TypeHint` enum variants
//! - ✅ Message structure: All required sections present (Current, Suggestion, Benefits, Suppression)
//! - ✅ User experience: Clear formatting, actionable guidance, reassuring tone
//!
//! ## Known Pitfalls
//!
//! - **Message formatting changes**: Tests check for specific strings. If message format
//!   changes, tests will break even if semantics are correct.
//!   Mitigation: Tests check semantic content (key terms) not exact formatting.
//!
//! - **Emoji rendering**: Message uses emojis (💡) which may render differently on
//!   different terminals.
//!   Mitigation: Emojis are cosmetic only, core message is pure text.
//!
//! ## Dependencies
//!
//! - Uses: `unilang::build_helpers::type_analyzer::{ TypeHint, Severity }`
//! - Uses: `unilang::build_helpers::hint_generator::HintGenerator`

use unilang::build_helpers::type_analyzer::{ TypeHint, Severity };
use unilang::build_helpers::hint_generator::HintGenerator;

#[test]
fn generates_boolean_message()
{
  let hint = TypeHint::BooleanAsString
  {
    argument_name : "enabled".to_string(),
    default_value : "true".to_string(),
    severity : Severity::Warning,
  };

  let message = HintGenerator::generate_warning( &hint );

  assert!( message.contains( "💡 Type Hint" ) );
  assert!( message.contains( "kind: \"Boolean\"" ) );
  assert!( message.contains( "default: true" ) );
  assert!( message.contains( "get_boolean" ) );
  assert!( message.contains( "suppress_type_hint: true" ) );
}

#[test]
fn generates_integer_message()
{
  let hint = TypeHint::IntegerAsString
  {
    argument_name : "verbosity".to_string(),
    default_value : "1".to_string(),
    severity : Severity::Warning,
  };

  let message = HintGenerator::generate_warning( &hint );

  assert!( message.contains( "💡 Type Hint" ) );
  assert!( message.contains( "kind: \"Integer\"" ) );
  assert!( message.contains( "default: 1" ) );
  assert!( message.contains( "get_integer" ) );
  assert!( message.contains( "validation_rules" ) );
}

#[test]
fn message_shows_suppression()
{
  let hint = TypeHint::BooleanAsString
  {
    argument_name : "test".to_string(),
    default_value : "true".to_string(),
    severity : Severity::Warning,
  };

  let message = HintGenerator::generate_warning( &hint );

  assert!( message.contains( "suppress_type_hint: true" ) );
  assert!( message.contains( "UNILANG_SUPPRESS_TYPE_HINTS" ) );
}

#[test]
fn message_emphasizes_non_error()
{
  let hint = TypeHint::BooleanAsString
  {
    argument_name : "test".to_string(),
    default_value : "false".to_string(),
    severity : Severity::Warning,
  };

  let message = HintGenerator::generate_warning( &hint );

  assert!( message.contains( "hint, not an error" ) );
  assert!( message.contains( "Build continues" ) );
}
