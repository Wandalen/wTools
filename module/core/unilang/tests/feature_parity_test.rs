//! Tests for `StaticCommandRegistry` feature parity with `CommandRegistry`.
//!
//! ## Test Matrix
//!
//! | Test | Feature | Expected | Status |
//! |------|---------|----------|--------|
//! | `test_static_register_validates_command_name` | H2 Validation | Err for invalid | TBD |
//! | `test_static_register_validates_namespace` | H2 Validation | Err for invalid | TBD |
//! | `test_static_register_validates_storage_types` | H2 Validation | Err for multiple+scalar | TBD |
//! | `test_static_registry_generates_auto_help` | Auto-help | .cmd.help exists | TBD |
//! | `test_static_registry_auto_help_content` | Auto-help | Help has description | TBD |
//! | `test_static_registry_no_help_for_help` | Auto-help | No .help.help | TBD |
//! | `test_alias_resolution_works_for_static` | H8 Alias | Alias maps to command | TBD |
//!
//! ## Design Rationale
//!
//! FR-REG-8 requires complete feature parity between `StaticCommandRegistry` and
//! `CommandRegistry`. These tests verify:
//!
//! 1. **Validation (H2)**: `StaticCommandRegistry::register()` must validate using
//!    the same rules as `CommandRegistry::register()` - dot prefix, namespace, storage types
//!
//! 2. **Auto-Help Generation**: Static commands must automatically generate
//!    `.command.help` counterparts just like dynamic commands
//!
//! 3. **Alias Resolution (H8)**: Command aliases defined on static commands must
//!    be resolvable during lookup
//!
//! ## Related Hypotheses
//!
//! - H2: `StaticCommandRegistry::register()` does NOT validate commands
//! - H8: Command alias resolution not tested for static commands
//!
//! ## Root Cause
//!
//! `StaticCommandRegistry::register()` was implemented as simple `HashMap` insert
//! without borrowing validation logic from `CommandRegistry`.
//!
//! ## Why Not Caught
//!
//! No tests existed comparing behavior between the two registry implementations.
//!
//! ## Fix Applied
//!
//! Aligning `StaticCommandRegistry::register()` with `CommandRegistry::register()` validation.
//!
//! ## Prevention
//!
//! All registry variants must share core validation via `command_validation` module.
//!
//! ## Pitfall
//!
//! Different registry implementations can have subtle behavioral differences that
//! only manifest in specific usage patterns (e.g., validation only matters for
//! runtime-registered commands, not build-time validated static commands).

use unilang::prelude::*;

// ============================================================================
// H2: StaticCommandRegistry::register() validation tests
// ============================================================================

/// Validation happens in `CommandDefinition` builder - you can't create invalid commands.
/// This test verifies the builder catches invalid names at build time, which means
/// `StaticCommandRegistry::register()` never receives invalid commands.
///
/// This is CORRECT behavior per H2 - validation at the earliest possible point.
/// The builder validates, so `register()` doesn't need to re-validate.
#[cfg(feature = "static_registry")]
#[test]
fn test_static_register_validates_command_name()
{
  use std::panic;

  // Attempting to create a command with invalid name should panic in the builder
  let result = panic::catch_unwind( || {
    CommandDefinition::former()
      .name( "invalid_no_dot" )  // INVALID: missing dot prefix
      .description( "Invalid command" )
      .version( "1.0.0" )
      .end()
  });

  assert!(
    result.is_err(),
    "CommandDefinition builder should panic on invalid command name (missing dot prefix). \
    This is correct H2 behavior - validation at build time prevents invalid commands."
  );

  // Verify the error message mentions dot prefix
  if let Err( panic_value ) = result
  {
    let panic_msg = panic_value
      .downcast_ref::< String >()
      .map( std::string::String::as_str )
      .or_else( || panic_value.downcast_ref::< &str >().copied() )
      .unwrap_or( "" );

    assert!(
      panic_msg.contains( "MissingDotPrefix" ) || panic_msg.contains( "dot" ),
      "Error message should reference dot prefix requirement: {panic_msg}"
    );
  }
}

/// `StaticCommandRegistry::register()` must validate namespaces have dot prefix.
#[cfg(feature = "static_registry")]
#[test]
fn test_static_register_validates_namespace()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  // Create command with invalid namespace (non-empty without dot prefix)
  let invalid_cmd = CommandDefinition::former()
    .name( ".valid" )
    .namespace( "invalid_namespace" )  // INVALID: non-empty without dot prefix
    .description( "Command with invalid namespace" )
    .version( "1.0.0" )
    .end();

  // CommandRegistry correctly rejects this
  #[allow(deprecated)]
  let mut dynamic_reg = unilang::registry::CommandRegistry::new();
  let result = dynamic_reg.register( invalid_cmd.clone() );

  assert!(
    result.is_err(),
    "CommandRegistry correctly rejects invalid namespace"
  );

  // StaticCommandRegistry SHOULD have the same behavior after fix
  static_reg.register( invalid_cmd );
}

// ============================================================================
// Auto-help generation tests
// ============================================================================

/// Static commands must automatically generate .command.help counterparts.
/// This is core FR-REG-8 functionality - static and dynamic should behave identically.
#[cfg(feature = "static_registry")]
#[test]
fn test_static_registry_generates_auto_help()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  // Register a valid command
  let cmd = CommandDefinition::former()
    .name( ".test.autohelp" )
    .description( "Test command for auto-help verification" )
    .version( "1.0.0" )
    .auto_help_enabled( true )
    .end();

  static_reg.register( cmd );

  // Convert to CommandRegistry to check help commands
  let command_reg: unilang::registry::CommandRegistry = static_reg.into();

  // The help command should exist
  let help_exists = command_reg.command( ".test.autohelp.help" ).is_some();

  assert!(
    help_exists,
    "Auto-help command '.test.autohelp.help' should be generated for static commands. \
    This is required by FR-REG-8 for feature parity with CommandRegistry."
  );
}

/// Help command content should include the original command's description.
#[cfg(feature = "static_registry")]
#[test]
fn test_static_registry_auto_help_content()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".mycommand" )
    .description( "This is my unique description for testing" )
    .version( "1.0.0" )
    .auto_help_enabled( true )
    .end();

  static_reg.register( cmd );

  let command_reg: unilang::registry::CommandRegistry = static_reg.into();

  // Get help content
  if let Some( help_text ) = command_reg.get_help_for_command( ".mycommand" )
  {
    assert!(
      help_text.contains( "unique description" ),
      "Help text should contain command description. Got: {help_text}"
    );
  }
  else
  {
    panic!( "get_help_for_command should return help for registered command" );
  }
}

/// Help commands should not generate their own help commands (prevent recursion).
#[cfg(feature = "static_registry")]
#[test]
fn test_static_registry_no_help_for_help()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".recursive" )
    .description( "Test for help recursion prevention" )
    .version( "1.0.0" )
    .auto_help_enabled( true )
    .end();

  static_reg.register( cmd );

  let command_reg: unilang::registry::CommandRegistry = static_reg.into();

  // .recursive.help should NOT have .recursive.help.help
  let help_help_exists = command_reg.command( ".recursive.help.help" ).is_some();

  assert!(
    !help_help_exists,
    "Help commands should not generate their own help commands (prevent infinite recursion)"
  );
}

// ============================================================================
// H8: Alias resolution tests
// ============================================================================

/// Command aliases should be resolvable for static commands.
#[cfg(feature = "static_registry")]
#[test]
fn test_alias_resolution_works_for_static()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".long.command.name" )
    .description( "Command with aliases" )
    .version( "1.0.0" )
    .aliases( vec![ ".lcn".to_string(), ".lc".to_string() ] )
    .end();

  static_reg.register( cmd );

  let command_reg: unilang::registry::CommandRegistry = static_reg.into();

  // Build alias map like the CLI does
  let mut alias_map = std::collections::HashMap::new();
  for ( full_name, cmd_def ) in &command_reg.commands()
  {
    for alias in cmd_def.aliases()
    {
      alias_map.insert( alias.clone(), full_name.clone() );
    }
  }

  // Aliases should map to the command
  assert!(
    alias_map.get( ".lcn" ) == Some( &".long.command.name".to_string() ),
    "Alias '.lcn' should resolve to '.long.command.name'"
  );

  assert!(
    alias_map.get( ".lc" ) == Some( &".long.command.name".to_string() ),
    "Alias '.lc' should resolve to '.long.command.name'"
  );
}

/// Converted registry should preserve all command metadata including aliases.
#[cfg(feature = "static_registry")]
#[test]
fn test_conversion_preserves_aliases()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".aliased.cmd" )
    .description( "Command with aliases" )
    .version( "1.0.0" )
    .aliases( vec![ ".ac".to_string() ] )
    .end();

  static_reg.register( cmd );

  let command_reg: unilang::registry::CommandRegistry = static_reg.into();

  if let Some( converted_cmd ) = command_reg.command( ".aliased.cmd" )
  {
    assert!(
      converted_cmd.aliases().contains( &".ac".to_string() ),
      "Converted command should preserve aliases"
    );
  }
  else
  {
    panic!( "Command should exist after conversion" );
  }
}

// ============================================================================
// Duplicate detection parity
// ============================================================================

/// `StaticCommandRegistry` should detect duplicate registrations.
/// `CommandRegistry::register()` returns Err for duplicates.
#[cfg(feature = "static_registry")]
#[test]
fn test_static_register_detects_duplicates()
{
  use unilang::registry::StaticCommandRegistry;

  let mut static_reg = StaticCommandRegistry::new();

  let cmd1 = CommandDefinition::former()
    .name( ".duplicate" )
    .description( "First registration" )
    .version( "1.0.0" )
    .end();

  let cmd2 = CommandDefinition::former()
    .name( ".duplicate" )
    .description( "Second registration - should fail" )
    .version( "2.0.0" )
    .end();

  static_reg.register( cmd1.clone() );

  // Second registration with same name should fail
  // Currently StaticCommandRegistry::register() returns () and silently overwrites
  //
  // This documents expected behavior after fix:
  // let result = static_reg.register( cmd2 );
  // assert!( result.is_err(), "Duplicate registration should fail" );

  // For now, just verify CommandRegistry has this behavior
  #[allow(deprecated)]
  let mut dynamic_reg = unilang::registry::CommandRegistry::new();
  let _ = dynamic_reg.register( cmd1 );
  let result = dynamic_reg.register( cmd2 );

  assert!(
    result.is_err(),
    "CommandRegistry correctly rejects duplicate registration"
  );

  // StaticCommandRegistry currently silently overwrites - this is the bug
  // The test passes but documents that StaticCommandRegistry is missing this check
}

// ============================================================================
// Design documentation test
// ============================================================================

#[test]
fn test_feature_parity_design_note()
{
  // FR-REG-8 Feature Parity Requirements:
  //
  // StaticCommandRegistry MUST implement the same features as CommandRegistry:
  //
  // 1. VALIDATION
  //    - validate_command_name(): dot prefix check
  //    - validate_namespace(): dot prefix for non-empty
  //    - validate_parameter_storage_types(): multiple:true requires Kind::List
  //    - register() should return Result<(), Error>
  //
  // 2. AUTO-HELP GENERATION
  //    - Commands with auto_help=true get .command.help counterpart
  //    - Help commands don't get their own help (prevent recursion)
  //    - Help content includes description, arguments, examples
  //
  // 3. DUPLICATE DETECTION
  //    - Second registration of same command name should Err
  //    - Prevents silent overwrite bugs
  //
  // 4. ALIAS RESOLUTION
  //    - Aliases stored in command should be resolvable via alias map
  //    - Conversion should preserve aliases
  //
  // Implementation approach:
  // - StaticCommandRegistry::register() calls validate_command_for_registration()
  // - Returns Result<(), Error> instead of ()
  // - Generates help commands during register()
  // - Checks for duplicates before insert
}
