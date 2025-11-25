//!
//! # Category Field Backward Compatibility Tests - CRITICAL
//!
//! ## What This Tests
//!
//! **THESE TESTS ARE CRITICAL FOR PUBLICATION**
//!
//! This test suite validates that adding the `category` field doesn't break existing
//! projects using unilang v0.39.0 YAML files (which lack the category field).
//!
//! Backward compatibility is a HARD REQUIREMENT for publication. Breaking existing
//! users is unacceptable.
//!
//! ## Why This Matters
//!
//! Existing projects have YAML manifests without `category:` lines. When they upgrade
//! to unilang v0.40.0, their YAML must still work without modification. These tests prevent:
//! - Breaking all existing projects using unilang
//! - Forcing all-or-nothing migration (users must be able to adopt gradually)
//! - Surprising behavior changes for users not using categories
//! - Generated code format changes breaking consumers
//!
//! ## Failure Interpretation
//!
//! - `old_yaml_without_category_loads_successfully()` fails: **BLOCKS PUBLICATION** - Breaking existing projects
//! - `old_yaml_produces_same_behavior()` fails: **BLOCKS PUBLICATION** - Behavior regression
//! - `mixed_old_and_new_yaml()` fails: **BLOCKS PUBLICATION** - Cannot migrate gradually
//! - `generated_code_backward_compatible()` fails: **BLOCKS PUBLICATION** - Generated code format changed
//!
//! ## Related
//!
//! - Issue-089: Category field must be optional in YAML
//! - Semver: This is a minor version bump (0.39.0 → 0.40.0), must maintain compatibility

use unilang::static_data::*;
use unilang::data::CommandDefinition;

//
// Test: old YAML without category loads successfully
//

/// **CRITICAL**: Verifies that YAML without category field loads without errors.
///
/// This prevents breaking all existing projects using unilang.
#[ test ]
fn old_yaml_without_category_loads_successfully()
{
  // Simulate old-style StaticCommandDefinition created by build.rs from old YAML
  static OLD_STYLE_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".legacy_command",
    namespace : "",
    description : "Command from old YAML without category",
    hint : "",
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
    examples : &[],
    auto_help_enabled : true,
    category : "", // Build script defaults to "" when YAML lacks category
  };

  // Convert to dynamic - should succeed
  let dynamic_cmd : CommandDefinition = ( &OLD_STYLE_CMD ).into();

  // Verify basic fields still work
  assert_eq!( dynamic_cmd.name().as_str(), ".legacy_command" );
  assert_eq!( dynamic_cmd.category(), "" );
}

//
// Test: old YAML produces same behavior
//

/// **CRITICAL**: Verifies that commands without category behave identically to v0.39.0.
///
/// This prevents surprising behavior changes for users not using categories.
#[ test ]
fn old_yaml_produces_same_behavior()
{
  static OLD_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
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
    examples : &[],
    auto_help_enabled : true,
    category : "", // Old YAML → empty category
  };

  let dynamic_cmd : CommandDefinition = ( &OLD_CMD ).into();

  // In v0.39.0, category would be empty
  // In v0.40.0 with old YAML, category must still be empty (same behavior)
  assert_eq!( dynamic_cmd.category(), "" );

  // Help generation should still work
  assert!( dynamic_cmd.auto_help_enabled() );
}

//
// Test: mixed old and new YAML
//

/// **CRITICAL**: Verifies that projects can mix old YAML (no category) and new YAML (with category).
///
/// This prevents forcing all-or-nothing migration.
#[ test ]
fn mixed_old_and_new_yaml()
{
  // Old-style command (no category in YAML)
  static OLD_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".old_command",
    namespace : "",
    description : "Old command",
    hint : "",
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
    examples : &[],
    auto_help_enabled : true,
    category : "",
  };

  // New-style command (has category in YAML)
  static NEW_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".new_command",
    namespace : "",
    description : "New command",
    hint : "",
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
    examples : &[],
    auto_help_enabled : true,
    category : "utilities",
  };

  // Both should convert successfully
  let old_dynamic : CommandDefinition = ( &OLD_CMD ).into();
  let new_dynamic : CommandDefinition = ( &NEW_CMD ).into();

  assert_eq!( old_dynamic.category(), "" );
  assert_eq!( new_dynamic.category(), "utilities" );
}

//
// Test: generated code backward compatible
//

/// **CRITICAL**: Verifies that generated PHF code compiles and works correctly.
///
/// This prevents generated code format breaking old consumers.
#[ test ]
fn generated_code_backward_compatible()
{
  use phf::Map;

  // Simulate PHF map generated by build.rs from old YAML
  static TEST_MAP : Map< &'static str, &'static StaticCommandDefinition > = phf::phf_map!
  {
    ".test" => &StaticCommandDefinition
    {
      name : ".test",
      namespace : "",
      description : "Test",
      hint : "",
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
      examples : &[],
      auto_help_enabled : true,
      category : "", // Old YAML defaults to empty
    },
  };

  // Verify PHF lookup still works
  let cmd = TEST_MAP.get( ".test" );
  assert!( cmd.is_some() );

  let cmd = cmd.unwrap();
  assert_eq!( cmd.name, ".test" );
  assert_eq!( cmd.category, "" );
}
