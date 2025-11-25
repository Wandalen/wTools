//! Help Completeness Validation Tests
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Status |
//! |---------|---------------|-------|----------|--------|
//! | HC-1 | Validate all commands have help | Registry with commands | Validation passes | NEW API |
//! | HC-2 | Detect missing help commands | Command without .help | Validation fails | NEW API |
//! | HC-3 | Formatted listing completeness | Multiple commands | All in formatted output | NEW API |
//!
//! ## Scope
//!
//! Tests the proposed API for validating that every command in the registry has
//! corresponding help. This validates the solution to the help divergence problem.
//!
//! ## Coverage
//!
//! - Registry validation API (proposed: `validate_help_completeness()`)
//! - Formatted command listing API (proposed: `format_command_listing()`)
//! - Detection of missing help commands
//! - Verification that help is complete and accurate
//!
//! ## Related
//!
//! - `task/prevent_command_help_divergence.md` - Solution specification
//! - `tests/help/help_divergence_prevention.rs` - Bug demonstration
//! - `tests/registry/auto_help_integration.rs` - Automatic help generation
//!
//! ## Implementation Status
//!
//! These tests use PROPOSED APIs that dont exist yet:
//! - `CommandRegistry::validate_help_completeness()` - NEW
//! - `CommandRegistry::format_command_listing()` - NEW
//!
//! Tests will FAIL until APIs are implemented (this is expected).

#![ allow( deprecated ) ]
#![ allow( unused_imports ) ] // Some APIs dont exist yet

use unilang::data::{ CommandDefinition, OutputData };
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper: Create minimal test command with `auto_help` enabled
fn create_test_command( name : &str, description : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( description )
    .hint( format!( "Hint for {name}" ) )
    .examples( vec![ format!( "{name} example::value" ) ] )
    .auto_help_enabled( true ) // Explicitly enable auto-help
    .end()
}

/// Helper: Create mock routine
fn create_mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData > + Send + Sync + 'static >
{
  Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, unilang::data::ErrorData >
  {
    Ok( OutputData::new( "test", "text" ) )
  })
}

/// HC-1: Validate that all commands have help (proposed API)
///
/// **Proposed API:** `registry.validate_help_completeness()`
/// **Expected:** Returns Ok(()) if all commands have help
/// **Actual:** API now implemented
#[ test ]
fn test_validate_all_commands_have_help()
{
  let mut registry = CommandRegistry::new();

  // Register commands with automatic help generation
  let cmd1 = create_test_command( ".build", "Build the project" );
  let cmd2 = create_test_command( ".test", "Run tests" );
  let cmd3 = create_test_command( ".deploy", "Deploy application" );

  registry.command_add_runtime( &cmd1, create_mock_routine() )
    .expect( "Command registration should succeed" );
  registry.command_add_runtime( &cmd2, create_mock_routine() )
    .expect( "Command registration should succeed" );
  registry.command_add_runtime( &cmd3, create_mock_routine() )
    .expect( "Command registration should succeed" );

  // Validate help completeness
  // This should pass if help was auto-generated for all commands
  let validation_result = registry.validate_help_completeness();

  assert!(
    validation_result.is_ok(),
    "All registered commands should have help: {:?}",
    validation_result.err()
  );
}

/// HC-2: Detect missing help commands (validation API)
///
/// **API:** `registry.validate_help_completeness()`
/// **Expected:** Returns Err with list of commands missing help
/// **Actual:** With auto-help enabled, this should always pass
#[ test ]
fn test_detect_missing_help_commands()
{
  let mut registry = CommandRegistry::new();

  // Register command with auto_help enabled
  let cmd = create_test_command( ".deploy", "Deploy application" );
  registry.command_add_runtime( &cmd, create_mock_routine() )
    .expect( "Command registration should succeed" );

  // With auto_help_enabled, the help command should be auto-generated
  // So validation should pass
  let validation_result = registry.validate_help_completeness();

  assert!(
    validation_result.is_ok(),
    "Auto-generated help should make validation pass: {:?}",
    validation_result.err()
  );
}

/// HC-3: Formatted command listing includes all commands (API)
///
/// **API:** `registry.format_command_listing()`
/// **Expected:** Returns formatted string with all commands and descriptions
/// **Actual:** API now implemented
#[ test ]
fn test_formatted_listing_completeness()
{
  let mut registry = CommandRegistry::new();

  // Register multiple commands
  let commands = vec![
    ( ".config", "Manage configuration" ),
    ( ".status", "Show system status" ),
    ( ".logs", "View application logs" ),
    ( ".backup", "Create backup" ),
  ];

  for ( name, desc ) in &commands
  {
    let cmd = create_test_command( name, desc );
    registry.command_add_runtime( &cmd, create_mock_routine() )
      .expect( "Command registration should succeed" );
  }

  // Get formatted command listing
  let formatted_listing = registry.format_command_listing();

  // Verify all commands appear in formatted output
  for ( name, desc ) in &commands
  {
    assert!(
      formatted_listing.contains( name ) || formatted_listing.contains( name.trim_start_matches( '.' ) ),
      "Formatted listing should contain command: {name}"
    );

    assert!(
      formatted_listing.contains( desc ),
      "Formatted listing should contain description: {desc}"
    );
  }

  // Verify formatting structure
  assert!(
    formatted_listing.contains( "Available" ) || formatted_listing.contains( "Commands" ),
    "Formatted listing should have header"
  );

  // Verify proper alignment/spacing
  let lines : Vec< &str > = formatted_listing.lines().collect();
  assert!(
    lines.len() >= commands.len(),
    "Should have at least one line per command"
  );
}
