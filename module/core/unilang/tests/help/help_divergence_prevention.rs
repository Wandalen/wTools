//! Help Divergence Prevention Tests
//!
//! ## Test Matrix
//!
//! | Test ID | What's Tested | Input | Expected | Actual Bug |
//! |---------|---------------|-------|----------|------------|
//! | HD-1 | Command visible in listing | Registered command | In listing | MAY FAIL |
//! | HD-2 | Help command auto-generated | Registered command | .cmd.help exists | MAY FAIL |
//! | HD-3 | Multiple commands complete | 3 commands | All 3 listed | MAY FAIL |
//! | HD-4 | Registry-help sync | Register then list | Complete list | MAY FAIL |
//!
//! ## Scope
//!
//! Tests that commands registered in `CommandRegistry` ALWAYS appear in help listings.
//! These tests demonstrate the wflow production bug where .languages command was
//! registered and executable but INVISIBLE in help output.
//!
//! ## Coverage
//!
//! - Command registration must add to help listing
//! - Help commands (.command.help) must be auto-generated
//! - No commands can be registered without appearing in help
//! - Registry and help system must stay synchronized
//!
//! ## Related
//!
//! - `task/prevent_command_help_divergence.md` - Full problem description
//! - `tests/help/generation.rs` - Help generation tests
//! - `tests/registry/duplicate_detection.rs` - Registry validation tests
//!
//! ## Known Issues
//!
//! These tests MAY FAIL initially - they demonstrate bugs that need fixing:
//! - Commands can be registered without appearing in help
//! - No automatic help command generation
//! - Manual help maintenance allows divergence

#![ allow( deprecated ) ]

use unilang::data::{ CommandDefinition, OutputData };
use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper: Create minimal test command with `auto_help` enabled
fn create_test_command( name : &str, description : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( description )
    .hint( format!( "Hint for {name}" ) )
    .auto_help_enabled( true ) // Explicitly enable auto-help for testing
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

/// HD-1: Command registered must appear in help listing
///
/// Reproduces wflow bug: .languages command registered but invisible in help
///
/// **Expected behavior:** Every registered command MUST appear in help listing
/// **Actual bug:** Commands can be registered without appearing in help
#[ test ]
fn test_registered_command_appears_in_help_listing()
{
  let mut registry = CommandRegistry::new();

  // Register command (simulates wflow registering .languages)
  let cmd = create_test_command( ".languages", "Detect programming languages" );
  let cmd_name = cmd.name.clone();

  registry.command_add_runtime( &cmd, create_mock_routine() )
    .expect( "Command registration should succeed" );

  // Generate help listing
  let help_generator = HelpGenerator::new( &registry );
  let help_listing = help_generator.list_commands();

  // CRITICAL CHECK: Registered command MUST be visible in help
  assert!(
    help_listing.contains( &cmd_name ) || help_listing.contains( "languages" ),
    "HELP DIVERGENCE BUG: Command '{cmd_name}' was registered but is NOT in help listing!\n\
     This is the exact bug from wflow production.\n\
     Help listing:\n{help_listing}"
  );

  // Verify command is actually in registry
  assert!(
    registry.command( &cmd_name ).is_some(),
    "Command should exist in registry"
  );
}

/// HD-2: Help command (.command.help) must be auto-generated
///
/// **Expected behavior:** Registering .foo should auto-create .foo.help
/// **Actual bug:** No automatic help command generation
#[ test ]
fn test_help_command_auto_generated_on_registration()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_test_command( ".deploy", "Deploy application" );
  let cmd_name = cmd.name.clone();
  let help_cmd_name = format!( "{cmd_name}.help" );

  registry.command_add_runtime( &cmd, create_mock_routine() )
    .expect( "Command registration should succeed" );

  // CRITICAL CHECK: Help command should exist automatically
  assert!(
    registry.command( &help_cmd_name ).is_some(),
    "HELP AUTO-GENERATION BUG: Help command '{help_cmd_name}' was NOT auto-generated!\n\
     Commands should automatically get .help variants."
  );

  // Verify help command is executable
  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( &cmd_name );

  assert!(
    help_content.is_some(),
    "Help content should be available for registered command"
  );
}

/// HD-3: All registered commands must appear in help (no partial listings)
///
/// **Expected behavior:** If 3 commands registered, all 3 must be in help
/// **Actual bug:** Manual help maintenance can forget commands
#[ test ]
fn test_multiple_commands_all_appear_in_help()
{
  let mut registry = CommandRegistry::new();

  // Register 3 commands (simulates multi-command CLI)
  let commands = vec![
    create_test_command( ".build", "Build the project" ),
    create_test_command( ".test", "Run tests" ),
    create_test_command( ".deploy", "Deploy application" ),
  ];

  for cmd in &commands
  {
    registry.command_add_runtime( cmd, create_mock_routine() )
      .expect( "Command registration should succeed" );
  }

  // Generate help listing
  let help_generator = HelpGenerator::new( &registry );
  let help_listing = help_generator.list_commands();

  // CRITICAL CHECK: ALL commands must be visible
  for cmd in &commands
  {
    let cmd_name = &cmd.name;
    assert!(
      help_listing.contains( cmd_name ) || help_listing.contains( cmd_name.trim_start_matches( '.' ) ),
      "HELP COMPLETENESS BUG: Command '{}' is missing from help listing!\n\
       {} commands registered, but '{}' is invisible.\n\
       Help listing:\n{}",
      cmd_name,
      commands.len(),
      cmd_name,
      help_listing
    );
  }

  // Verify count matches
  let registered_count = commands.len();
  let help_command_count = help_listing.matches( "build" ).count()
    + help_listing.matches( "test" ).count()
    + help_listing.matches( "deploy" ).count();

  assert!(
    help_command_count >= registered_count,
    "HELP COUNT MISMATCH: Registered {registered_count} commands but help only shows {help_command_count} command mentions"
  );
}

/// HD-4: Registry-help synchronization across multiple operations
///
/// **Expected behavior:** Help listing stays in sync with registry state
/// **Actual bug:** Manual help can diverge from actual registry
#[ test ]
fn test_registry_help_synchronization()
{
  let mut registry = CommandRegistry::new();

  // Phase 1: Register initial commands
  let cmd1 = create_test_command( ".config", "Manage configuration" );
  let cmd2 = create_test_command( ".status", "Show status" );

  registry.command_add_runtime( &cmd1, create_mock_routine() )
    .expect( "First command registration should succeed" );
  registry.command_add_runtime( &cmd2, create_mock_routine() )
    .expect( "Second command registration should succeed" );

  // Phase 2: Check help is synchronized
  let help_generator = HelpGenerator::new( &registry );
  let help_listing = help_generator.list_commands();

  // CRITICAL CHECK: Both commands must be visible
  assert!(
    help_listing.contains( "config" ) && help_listing.contains( "status" ),
    "SYNC BUG: Not all registered commands appear in help!\n\
     Expected: .config, .status\n\
     Help listing:\n{help_listing}"
  );

  // Phase 3: Add third command
  let cmd3 = create_test_command( ".logs", "View logs" );
  registry.command_add_runtime( &cmd3, create_mock_routine() )
    .expect( "Third command registration should succeed" );

  // Phase 4: Verify help updated automatically
  let help_generator_updated = HelpGenerator::new( &registry );
  let help_listing_updated = help_generator_updated.list_commands();

  assert!(
    help_listing_updated.contains( "logs" ),
    "SYNC UPDATE BUG: Newly registered command '.logs' not in updated help!\n\
     Help should automatically include new commands.\n\
     Updated help listing:\n{help_listing_updated}"
  );

  // Verify all three are still present
  assert!(
    help_listing_updated.contains( "config" )
      && help_listing_updated.contains( "status" )
      && help_listing_updated.contains( "logs" ),
    "SYNC COMPLETENESS BUG: Not all commands present after update!"
  );
}
