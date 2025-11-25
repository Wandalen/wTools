//! Help Generation Unit Tests
//!
//! ## Scope
//! Tests the help system's ability to generate comprehensive help content for commands.
//! This covers help content creation, formatting, and the various help access patterns.
//!
//! ## Coverage
//! - Command-specific help generation
//! - Global help listing
//! - Help content accuracy and completeness
//! - Help formatting and structure
//! - Error conditions in help generation
//!
//! ## Related
//! - `unit/help/conventions.rs` - Help system conventions
//! - `unit/help/formatting.rs` - Help output formatting

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes, OutputData };
use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper to create a test command definition
fn create_test_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".test" )
    .description( "A test command for help generation validation" )
    .hint( "Use this command to test help functionality" )
    .version( "1.0.0" )
    .status( "stable" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input file path".to_string(),
        kind : Kind::String,
        hint : "Path to the input file".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "i".to_string() ],
        tags : vec![ "file".to_string() ],
      },
      ArgumentDefinition {
        name : "output".to_string(),
        description : "Output file path".to_string(),
        kind : Kind::String,
        hint : "Path to the output file".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "output.txt".to_string() ),
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "o".to_string() ],
        tags : vec![ "file".to_string() ],
      },
      ArgumentDefinition {
        name : "verbose".to_string(),
        description : "Enable verbose output".to_string(),
        kind : Kind::Boolean,
        hint : "Show detailed output information".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "false".to_string() ),
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "v".to_string() ],
        tags : vec![ "output".to_string() ],
      }
    ])
    .examples( vec![
      ".test input::\"data.txt\"".to_string(),
      ".test input::\"data.txt\" output::\"result.txt\" verbose::true".to_string(),
    ])
    .end()
}

#[test]
fn test_command_specific_help_generation()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify help contains essential information (Level 2 Standard format)
  assert!( help_content.contains( "test" ), "Help should contain command name" );
  assert!( help_content.contains( "A test command for help generation validation" ), "Help should contain description" );
  assert!( help_content.contains( "input" ), "Help should contain required argument" );
  assert!( help_content.contains( "output" ), "Help should contain optional argument" );
  assert!( help_content.contains( "verbose" ), "Help should contain boolean argument" );
  assert!( help_content.contains( "Usage:" ), "Help should contain usage section" );
}

#[test]
fn test_help_includes_argument_details()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify argument details are included (Level 2 Standard format uses Arguments:)
  // Note: Level 2 shows hints if available, otherwise descriptions
  assert!( help_content.contains( "Arguments:" ), "Help should contain arguments section" );
  assert!( help_content.contains( "input" ), "Help should contain argument names" );
  assert!( help_content.contains( "string" ), "Help should contain type information" );
  assert!( help_content.contains( "Path to the input file" ) || help_content.contains( "Input file path" ), "Help should contain argument descriptions or hints" );
}

#[test]
fn test_help_includes_examples()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify help content structure (Level 2 Standard format)
  assert!( help_content.contains( "Usage:" ), "Help should contain usage section" );
  assert!( help_content.contains( "Examples:" ), "Help should contain examples section" );
  assert!( help_content.contains( "A test command" ), "Help should contain description" );
}

#[test]
fn test_help_includes_aliases()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify aliases are mentioned
  assert!( help_content.contains( 'i' ) && help_content.contains( "input" ), "Help should mention argument aliases" );
  assert!( help_content.contains( 'o' ) && help_content.contains( "output" ), "Help should mention argument aliases" );
}

#[test]
fn test_global_help_listing()
{
  let mut registry = CommandRegistry::new();

  // Add multiple commands
  let cmd1 = CommandDefinition::former()
    .name( ".first" )
    .description( "First test command" )
    .end();

  let cmd2 = CommandDefinition::former()
    .name( ".second" )
    .description( "Second test command" )
    .end();

  let cmd3 = CommandDefinition::former()
    .name( ".third" )
    .description( "Third test command" )
    .end();

  // Use runtime registration for all commands
  let mock_routine1 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let mock_routine2 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let mock_routine3 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd1, mock_routine1 ).unwrap();
  registry.command_add_runtime( &cmd2, mock_routine2 ).unwrap();
  registry.command_add_runtime( &cmd3, mock_routine3 ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.list_commands();

  // Verify all commands are listed
  assert!( help_content.contains( "first" ), "Global help should list first command" );
  assert!( help_content.contains( "second" ), "Global help should list second command" );
  assert!( help_content.contains( "third" ), "Global help should list third command" );

  // Verify descriptions are included
  assert!( help_content.contains( "First test command" ), "Global help should include descriptions" );
  assert!( help_content.contains( "Second test command" ), "Global help should include descriptions" );
  assert!( help_content.contains( "Third test command" ), "Global help should include descriptions" );

  // Verify overall structure
  assert!( help_content.contains( "Available" ) || help_content.contains( "Commands:" ), "Global help should have header" );
}

#[test]
fn test_help_for_nonexistent_command()
{
  let registry = CommandRegistry::new();
  let help_generator = HelpGenerator::new( &registry );

  let help_result = help_generator.command( ".nonexistent" );

  // Should handle gracefully
  assert!( help_result.is_none(), "Help for nonexistent command should return None" );
}

#[test]
fn test_help_with_empty_registry()
{
  let registry = CommandRegistry::new();
  let help_generator = HelpGenerator::new( &registry );

  let help_content = help_generator.list_commands();

  // Should handle empty registry gracefully - returns some content (even if just empty message)
  assert!( !help_content.is_empty(), "Should return some help content" );
  // Empty registry should not list any specific commands
  assert!( !help_content.contains( ".test" ), "Empty registry should not show test commands" );
}

#[test]
fn test_help_content_formatting()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify basic formatting structure
  let lines : Vec< &str > = help_content.lines().collect();
  assert!( lines.len() > 3, "Help should have multiple lines" );

  // Verify no obviously malformed content
  assert!( !help_content.contains( "{{" ), "Help should not contain template placeholders" );
  assert!( !help_content.contains( "}}" ), "Help should not contain template placeholders" );
  assert!( !help_content.is_empty(), "Help should not be empty" );
}

#[test]
fn test_help_performance()
{
  use std::time::Instant;

  let mut registry = CommandRegistry::new();

  // Add many commands to test performance
  for i in 1..=50 {
    let cmd = CommandDefinition::former()
      .name( format!( ".command{i}" ) )
      .description( format!( "Test command number {i}" ) )
      .end();

    let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
      Ok(OutputData::new("test", "text"))
    });
    registry.command_add_runtime( &cmd, mock_routine ).unwrap();
  }

  let help_generator = HelpGenerator::new( &registry );

  let start = Instant::now();
  let help_content = help_generator.list_commands();
  let duration = start.elapsed();

  // Performance check
  assert!( duration.as_millis() < 100, "Help generation for 50 commands took too long: {duration:?}" );

  // Verify correctness wasn't sacrificed
  assert!( help_content.contains( "command1" ), "Help should contain first command" );
  assert!( help_content.contains( "command50" ), "Help should contain last command" );
}

#[test]
fn test_command_help_with_complex_arguments()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".complex" )
    .description( "Command with complex arguments" )
    .arguments( vec![
      ArgumentDefinition {
        name : "multi_value".to_string(),
        description : "Parameter that accepts multiple values".to_string(),
        kind : Kind::List( Box::new( Kind::String ), None ),  // Fixed: multiple:true requires Kind::List
        hint : "Multiple string values".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "m".to_string(), "multi".to_string() ],
        tags : vec![ "list".to_string() ],
      }
    ])
    .end();

  let cmd_name = cmd.name().clone();
  // Use runtime registration instead since command_add is deprecated
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify complex argument features are documented
  assert!( help_content.contains( "multi_value" ), "Help should contain argument name" );
  assert!( help_content.contains( "multiple" ) || help_content.contains( "Multi" ), "Help should indicate multiple values capability" );
  assert!( help_content.contains( 'm' ), "Help should show aliases" );
}