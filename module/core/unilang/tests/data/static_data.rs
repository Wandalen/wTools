//!
//! Tests for the `static_data` module - Static to Dynamic Command Conversion
//!
//! # Why These Tests Exist
//!
//! **Purpose:** Verify that compile-time static command definitions correctly convert
//! to runtime dynamic `CommandDefinition` instances with full type safety preserved.
//!
//! **What We're Protecting Against:**
//!
//! 1. **Conversion bugs:** Static -> Dynamic conversion must preserve all fields and
//!    semantics. Tests catch any mismatches between static and dynamic representations.
//!
//! 2. **Validation bypassing:** Static commands don't have runtime validation (compile-time
//!    only). The conversion must apply dynamic validation rules correctly.
//!
//! 3. **Type mismatches:** Static types use `&'static str`, dynamic uses `String`.
//!    Tests verify the conversion handles lifetimes and allocations correctly.
//!
//! 4. **Enum conversion errors:** `StaticKind` -> `Kind` and `StaticValidationRule` ->
//!    `ValidationRule` conversions must be complete. Missing variant conversions would
//!    cause runtime errors.
//!
//! 5. **Field omissions:** Missing fields in conversion (Issue-088: `auto_help_enabled` was
//!    lost during conversion, breaking `.command.help` generation for all static commands).
//!
//! **How to Interpret Failures:**
//!
//! - **Conversion test fails:** Field mapping broken, check `From<&StaticCommandDefinition>`
//! - **Kind conversion fails:** Missing variant in `StaticKind` -> `Kind` mapping
//! - **Validation rule fails:** Missing variant in `StaticValidationRule` -> `ValidationRule`
//! - **Nested structure fails:** Complex types (`List`, `Map`, `Enum`) not converting correctly
//! - **`auto_help_enabled` fails:** Field not preserved from static to dynamic (Issue-088)
//!
//! **Why This Matters:**
//!
//! The `static_data` system enables compile-time command definition (zero runtime cost),
//! but dynamic systems (registry, help, CLI) need runtime representations. These tests
//! ensure the bridge between compile-time and runtime is correct.
//!
//! # Test Coverage Matrix
//!
//! ## Command Conversion Tests
//! - `test_static_command_definition_conversion` - Comprehensive field validation
//! - `test_static_command_definition_with_empty_arrays` - Empty/minimal commands
//! - `test_auto_help_enabled_conversion_preserves_true` - Issue-088 reproducer (true case)
//! - `test_auto_help_enabled_conversion_preserves_false` - Issue-088 reproducer (false case)
//! - `test_existing_conversion_test_includes_auto_help` - Regression prevention
//!
//! ## Type Conversion Tests
//! - `test_static_kind_conversion_primitives` - All primitive kinds
//! - `test_static_kind_conversion_enum` - Enum kind with choices
//! - `test_static_kind_conversion_list` - List kind with delimiter
//! - `test_static_kind_conversion_map` - Map kind with delimiters
//!
//! ## Validation Rule Conversion Tests
//! - `test_static_validation_rule_conversion` - Min, Max, `MinLength`, `MaxLength`, Pattern, `MinItems`
//!
//! ## Argument Tests
//! - `test_static_argument_attributes_conversion` - Attribute preservation
//! - `test_static_argument_definition_conversion` - Complex arguments with validation
//!

use unilang::static_data::*;

#[test]
fn test_static_command_definition_conversion()
{
  static STATIC_ARG: StaticArgumentDefinition = StaticArgumentDefinition {
    name: "test_arg",
    kind: StaticKind::String,
    attributes: StaticArgumentAttributes {
      optional: true,
      multiple: false,
      default: Some("default_value"),
      sensitive: false,
      interactive: false,
    },
    hint: "test hint",
    description: "test description",
    validation_rules: &[],
    aliases: &["alias1", "alias2"],
    tags: &["tag1", "tag2"],
  };

  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition {
    name: ".test_command",
    namespace: ".test",
    description: "A test command",
    hint: "Test hint",
    arguments: &[STATIC_ARG],
    routine_link: Some("test.routine"),
    status: "stable",
    version: "1.0.0",
    tags: &["test", "example"],
    aliases: &["tc", "test"],
    permissions: &["user", "admin"],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[".test_command arg::value"],
    auto_help_enabled: true,
  };

  let dynamic_cmd: unilang::data::CommandDefinition = (&STATIC_CMD).into();

  assert_eq!(dynamic_cmd.name().as_str(), ".test_command");
  assert_eq!(dynamic_cmd.namespace(), ".test");
  assert_eq!(dynamic_cmd.description(), "A test command");
  assert_eq!(dynamic_cmd.hint(), "Test hint");
  assert!(matches!(dynamic_cmd.status(), unilang::data::CommandStatus::Active));
  assert_eq!(dynamic_cmd.version().as_str(), "1.0.0");
  assert_eq!(dynamic_cmd.tags(), &vec!["test".to_string(), "example".to_string()]);
  assert_eq!(dynamic_cmd.aliases(), &vec!["tc".to_string(), "test".to_string()]);
  assert_eq!(dynamic_cmd.permissions(), &vec!["user".to_string(), "admin".to_string()]);
  assert!(dynamic_cmd.idempotent());
  assert_eq!(dynamic_cmd.deprecation_message(), "");
  assert_eq!(dynamic_cmd.http_method_hint(), "GET");
  assert_eq!(dynamic_cmd.examples(), &vec![".test_command arg::value".to_string()]);
  assert_eq!(dynamic_cmd.routine_link(), Some(&"test.routine".to_string()));

  assert_eq!(dynamic_cmd.arguments().len(), 1);
  let arg = &dynamic_cmd.arguments()[0];
  assert_eq!(arg.name, "test_arg");
  assert_eq!(arg.hint, "test hint");
  assert_eq!(arg.description, "test description");
  assert_eq!(arg.aliases, vec!["alias1", "alias2"]);
  assert_eq!(arg.tags, vec!["tag1", "tag2"]);
  assert!(arg.attributes.optional);
  assert!(!arg.attributes.multiple);
  assert_eq!(arg.attributes.default, Some("default_value".to_string()));
  assert!(!arg.attributes.sensitive);
  assert!(!arg.attributes.interactive);

  // Issue-088: Verify auto_help_enabled is preserved during conversion
  assert!(dynamic_cmd.auto_help_enabled(), "auto_help_enabled should be preserved from static definition");
  assert!(dynamic_cmd.has_auto_help(), "has_auto_help() should match auto_help_enabled value");
}

#[test]
fn test_static_kind_conversion_primitives()
{
  // Test primitive types
  let string_kind: unilang::data::Kind = (&StaticKind::String).into();
  assert!(matches!(string_kind, unilang::data::Kind::String));

  let integer_kind: unilang::data::Kind = (&StaticKind::Integer).into();
  assert!(matches!(integer_kind, unilang::data::Kind::Integer));

  let float_kind: unilang::data::Kind = (&StaticKind::Float).into();
  assert!(matches!(float_kind, unilang::data::Kind::Float));

  let boolean_kind: unilang::data::Kind = (&StaticKind::Boolean).into();
  assert!(matches!(boolean_kind, unilang::data::Kind::Boolean));

  let path_kind: unilang::data::Kind = (&StaticKind::Path).into();
  assert!(matches!(path_kind, unilang::data::Kind::Path));

  let file_kind: unilang::data::Kind = (&StaticKind::File).into();
  assert!(matches!(file_kind, unilang::data::Kind::File));

  let directory_kind: unilang::data::Kind = (&StaticKind::Directory).into();
  assert!(matches!(directory_kind, unilang::data::Kind::Directory));

  let url_kind: unilang::data::Kind = (&StaticKind::Url).into();
  assert!(matches!(url_kind, unilang::data::Kind::Url));

  let datetime_kind: unilang::data::Kind = (&StaticKind::DateTime).into();
  assert!(matches!(datetime_kind, unilang::data::Kind::DateTime));

  let pattern_kind: unilang::data::Kind = (&StaticKind::Pattern).into();
  assert!(matches!(pattern_kind, unilang::data::Kind::Pattern));

  let json_string_kind: unilang::data::Kind = (&StaticKind::JsonString).into();
  assert!(matches!(json_string_kind, unilang::data::Kind::JsonString));

  let object_kind: unilang::data::Kind = (&StaticKind::Object).into();
  assert!(matches!(object_kind, unilang::data::Kind::Object));
}

#[test]
fn test_static_kind_conversion_enum()
{
  let static_enum = StaticKind::Enum(&["red", "green", "blue"]);
  let dynamic_kind: unilang::data::Kind = (&static_enum).into();
  
  if let unilang::data::Kind::Enum(choices) = dynamic_kind {
    assert_eq!(choices, vec!["red", "green", "blue"]);
  } else {
    panic!("Expected Enum kind");
  }
}

#[test]
fn test_static_kind_conversion_list()
{
  static ITEM_KIND: StaticKind = StaticKind::String;
  let static_list = StaticKind::List(&ITEM_KIND, Some(','));
  let dynamic_kind: unilang::data::Kind = (&static_list).into();

  if let unilang::data::Kind::List(inner_kind, delimiter) = dynamic_kind {
    assert!(matches!(*inner_kind, unilang::data::Kind::String));
    assert_eq!(delimiter, Some(','));
  } else {
    panic!("Expected List kind");
  }
}

#[test]
fn test_static_kind_conversion_map()
{
  static KEY_KIND: StaticKind = StaticKind::String;
  static VALUE_KIND: StaticKind = StaticKind::Integer;
  let static_map = StaticKind::Map(&KEY_KIND, &VALUE_KIND, Some(','), Some('='));
  let dynamic_kind: unilang::data::Kind = (&static_map).into();

  if let unilang::data::Kind::Map(k_kind, v_kind, entry_delim, kv_delim) = dynamic_kind {
    assert!(matches!(*k_kind, unilang::data::Kind::String));
    assert!(matches!(*v_kind, unilang::data::Kind::Integer));
    assert_eq!(entry_delim, Some(','));
    assert_eq!(kv_delim, Some('='));
  } else {
    panic!("Expected Map kind");
  }
}

#[test]
fn test_static_validation_rule_conversion()
{
  // Test Min rule
  let min_rule = StaticValidationRule::Min(10.0);
  let dynamic_rule: unilang::data::ValidationRule = (&min_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::Min(10.0)));

  // Test Max rule
  let max_rule = StaticValidationRule::Max(100.0);
  let dynamic_rule: unilang::data::ValidationRule = (&max_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::Max(100.0)));

  // Test MinLength rule
  let min_length_rule = StaticValidationRule::MinLength(5);
  let dynamic_rule: unilang::data::ValidationRule = (&min_length_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::MinLength(5)));

  // Test MaxLength rule
  let max_length_rule = StaticValidationRule::MaxLength(50);
  let dynamic_rule: unilang::data::ValidationRule = (&max_length_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::MaxLength(50)));

  // Test Pattern rule
  let pattern_rule = StaticValidationRule::Pattern(r"\d+");
  let dynamic_rule: unilang::data::ValidationRule = (&pattern_rule).into();
  if let unilang::data::ValidationRule::Pattern(pattern) = dynamic_rule {
    assert_eq!(pattern, r"\d+");
  } else {
    panic!("Expected Pattern validation rule");
  }

  // Test MinItems rule
  let min_items_rule = StaticValidationRule::MinItems(3);
  let dynamic_rule: unilang::data::ValidationRule = (&min_items_rule).into();
  assert!(matches!(dynamic_rule, unilang::data::ValidationRule::MinItems(3)));
}

#[test]
fn test_static_argument_attributes_conversion()
{
  let static_attrs = StaticArgumentAttributes {
    optional: true,
    multiple: false,
    default: Some("test_default"),
    sensitive: true,
    interactive: false,
  };

  let dynamic_attrs: unilang::data::ArgumentAttributes = (&static_attrs).into();

  assert!(dynamic_attrs.optional);
  assert!(!dynamic_attrs.multiple);
  assert_eq!(dynamic_attrs.default, Some("test_default".to_string()));
  assert!(dynamic_attrs.sensitive);
  assert!(!dynamic_attrs.interactive);
}

#[test]
fn test_static_argument_definition_conversion()
{
  static VALIDATION_RULES: [StaticValidationRule; 2] = [
    StaticValidationRule::Min(0.0),
    StaticValidationRule::MaxLength(100),
  ];

  static STATIC_ARG: StaticArgumentDefinition = StaticArgumentDefinition {
    name: "complex_arg",
    kind: StaticKind::Float,
    attributes: StaticArgumentAttributes {
      optional: false,
      multiple: true,
      default: None,
      sensitive: false,
      interactive: true,
    },
    hint: "Complex argument hint",
    description: "A complex argument for testing",
    validation_rules: &VALIDATION_RULES,
    aliases: &["ca", "complex"],
    tags: &["complex", "test"],
  };

  let dynamic_arg: unilang::data::ArgumentDefinition = (&STATIC_ARG).into();

  assert_eq!(dynamic_arg.name, "complex_arg");
  assert!(matches!(dynamic_arg.kind, unilang::data::Kind::Float));
  assert!(!dynamic_arg.attributes.optional);
  assert!(dynamic_arg.attributes.multiple);
  assert_eq!(dynamic_arg.attributes.default, None);
  assert!(!dynamic_arg.attributes.sensitive);
  assert!(dynamic_arg.attributes.interactive);
  assert_eq!(dynamic_arg.hint, "Complex argument hint");
  assert_eq!(dynamic_arg.description, "A complex argument for testing");
  assert_eq!(dynamic_arg.aliases, vec!["ca", "complex"]);
  assert_eq!(dynamic_arg.tags, vec!["complex", "test"]);
  assert_eq!(dynamic_arg.validation_rules.len(), 2);
}

#[test]
fn test_static_command_definition_with_empty_arrays()
{
  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition {
    name: ".minimal_command",
    namespace: ".minimal",
    description: "Minimal command",
    hint: "Minimal hint",
    arguments: &[],
    routine_link: None,
    status: "experimental",
    version: "0.1.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: false,
    deprecation_message: "Deprecated for testing",
    http_method_hint: "POST",
    examples: &[],
    auto_help_enabled: true,
  };

  let dynamic_cmd: unilang::data::CommandDefinition = (&STATIC_CMD).into();

  assert_eq!(dynamic_cmd.name().as_str(), ".minimal_command");
  assert_eq!(dynamic_cmd.namespace(), ".minimal");
  assert!(dynamic_cmd.arguments().is_empty());
  assert_eq!(dynamic_cmd.routine_link(), None);
  assert!(matches!(dynamic_cmd.status(), unilang::data::CommandStatus::Deprecated { .. }));
  assert_eq!(dynamic_cmd.version().as_str(), "0.1.0");
  assert!(dynamic_cmd.tags().is_empty());
  assert!(dynamic_cmd.aliases().is_empty());
  assert!(dynamic_cmd.permissions().is_empty());
  assert!(!dynamic_cmd.idempotent());
  assert_eq!(dynamic_cmd.deprecation_message(), "Deprecated for testing");
  assert_eq!(dynamic_cmd.http_method_hint(), "POST");
  assert!(dynamic_cmd.examples().is_empty());
}

//
// Issue-088: auto_help_enabled Lost During Static-to-Dynamic Conversion
//

/// Test that `auto_help_enabled` is preserved during Static→Dynamic conversion (true case)
///
/// # Root Cause
///
/// The `From<&StaticCommandDefinition> for CommandDefinition` implementation at
/// `src/static_data.rs:609` hardcodes `auto_help_enabled: false` instead of reading
/// from the source struct field. This happens because:
///
/// 1. `StaticCommandDefinition` struct is missing the `auto_help_enabled` field entirely
/// 2. Build script (`build.rs:553-627`) doesnt extract the field from YAML
/// 3. Conversion has no source value to read, so it hardcodes `false`
///
/// # Why Not Caught
///
/// The existing conversion test (`test_static_command_definition_conversion`) validates
/// many fields but omitted `auto_help_enabled`. No test verified that ALL struct fields
/// are preserved during conversion, allowing this critical field to be silently lost.
///
/// # Fix Applied
///
/// (To be documented after fix implementation)
///
/// 1. Added `auto_help_enabled: bool` field to `StaticCommandDefinition` struct
/// 2. Updated `build.rs` to extract `auto_help_enabled` from YAML (defaults to true)
/// 3. Updated conversion to copy field: `static_cmd.auto_help_enabled` instead of hardcoded `false`
///
/// # Prevention
///
/// This test validates that the `auto_help_enabled` value is preserved for both
/// explicit true and explicit false values during static-to-dynamic conversion.
/// Future struct changes must include corresponding test coverage.
///
/// # Pitfall
///
/// **Silent Field Loss in Conversions:** Any field in `StaticCommandDefinition` that
/// isnt explicitly copied in the `From<&StaticCommandDefinition>` impl will be lost
/// or defaulted, silently breaking user YAML configuration with no compile-time or
/// runtime errors. Always verify ALL fields are tested in conversion tests.
///
/// **Impact:** Affects all `StaticCommandRegistry` users (v0.35+) - YAML declares
/// `auto_help_enabled: true` but runtime receives `false`, breaking `.command.help`
/// generation for willbe, `will_crates`, wflow, wplan, and external projects.
// test_kind: bug_reproducer(issue-088)
#[test]
fn test_auto_help_enabled_conversion_preserves_true()
{
  // This test will FAIL initially because:
  // 1. StaticCommandDefinition missing auto_help_enabled field (wont compile)
  // 2. After field added: Conversion hardcodes false (test will fail)
  // 3. After fix: Test passes

  static STATIC_CMD_WITH_HELP: StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".crates.list",
    namespace : ".crates",
    description : "List all crates in workspace",
    hint : "Lists crates",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[".crates.list"],
    auto_help_enabled : true, // ← Field doesnt exist yet, will fail to compile
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_CMD_WITH_HELP ).into();

  // CRITICAL ASSERTION: Verify auto_help_enabled is preserved
  assert!
  (
    dynamic_cmd.auto_help_enabled(),
    "Expected auto_help_enabled to be true (from static definition), but conversion returned false. \
     This breaks .command.help generation for all commands with auto_help_enabled: true in YAML."
  );

  // Verify related method also works
  assert!
  (
    dynamic_cmd.has_auto_help(),
    "has_auto_help() should return true when auto_help_enabled is true"
  );
}

/// Test that `auto_help_enabled`: false is preserved during conversion
///
/// # Root Cause
///
/// See `test_auto_help_enabled_conversion_preserves_true` for complete root cause analysis.
///
/// # Why Not Caught
///
/// Same as above - existing conversion test didnt validate `auto_help_enabled` field.
///
/// # Fix Applied
///
/// (To be documented after fix)
///
/// # Prevention
///
/// Validates that explicit `false` values are also preserved. Help commands themselves
/// should have `auto_help_enabled: false` to prevent recursive help generation.
///
/// # Pitfall
///
/// See `test_auto_help_enabled_conversion_preserves_true` for detailed pitfall analysis.
// test_kind: bug_reproducer(issue-088)
#[test]
fn test_auto_help_enabled_conversion_preserves_false()
{
  // This test will PASS initially (hardcoded false matches expected false)
  // After fix: Should still pass (correctly preserving explicit false)

  static STATIC_HELP_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".crates.list.help",
    namespace : ".crates",
    description : "Help for .crates.list command",
    hint : "Show help",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[ "help" ],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : false, // ← Prevent recursive help (field doesnt exist yet)
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_HELP_CMD ).into();

  // Verify auto_help_enabled is false (prevents recursive help generation)
  assert!
  (
    !dynamic_cmd.auto_help_enabled(),
    "Expected auto_help_enabled to be false for help commands (prevents recursion)"
  );

  assert!
  (
    !dynamic_cmd.has_auto_help(),
    "has_auto_help() should return false when auto_help_enabled is false"
  );
}

/// Verify that existing conversion test now checks `auto_help_enabled`
///
/// # Root Cause
///
/// See primary bug reproducer tests above for root cause analysis.
///
/// # Why Not Caught
///
/// The existing comprehensive conversion test at line 40 (`test_static_command_definition_conversion`)
/// validates many fields but completely omitted `auto_help_enabled`, allowing the bug to slip through.
///
/// # Fix Applied
///
/// (To be documented after fix - update existing test to include `auto_help_enabled` assertion)
///
/// # Prevention
///
/// This test serves as a reminder that the main conversion test MUST validate
/// `auto_help_enabled`. Future field additions must be added to the comprehensive test.
///
/// # Pitfall
///
/// **Incomplete Test Coverage:** Even comprehensive-looking tests can miss critical
/// fields. Systematic verification of ALL struct fields is required. Consider using
/// struct field count assertions or compile-time checks to ensure completeness.
// test_kind: bug_reproducer(issue-088)
#[test]
fn test_existing_conversion_test_includes_auto_help()
{
  // This test documents the requirement that test_static_command_definition_conversion
  // must be updated to validate auto_help_enabled field

  // TODO: After fix, update line 66 test (test_static_command_definition_conversion)
  // to include: assert!(dynamic_cmd.auto_help_enabled(), "auto_help_enabled should be preserved");

  // For now, verify the field exists and is accessible on CommandDefinition via builder
  use unilang::data::CommandName;

  let name = CommandName::new( ".test" ).expect( "valid command name" );
  let cmd = unilang::data::CommandDefinition::new( name, "Test command".to_string() )
    .with_auto_help( true );

  assert!(cmd.auto_help_enabled(), "CommandDefinition should support auto_help_enabled via with_auto_help");
  assert!(cmd.has_auto_help(), "has_auto_help() should return true when auto_help_enabled is true");
}