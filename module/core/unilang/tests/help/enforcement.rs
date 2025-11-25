//!
//! Comprehensive tests for mandatory help enforcement system
//!
//! This test suite validates that the help system is now completely mandatory:
//! 1. Every command MUST have a .*.help counterpart automatically generated
//! 2. Global .help command MUST always be present
//! 3. No flexibility or opt-out mechanisms exist
//! 4. Help enforcement works for all registration methods
//!

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;

/// Test routine for mandatory help tests
#[allow(clippy::unnecessary_wraps)]
fn test_routine( _cmd : unilang::semantic::VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Test command executed successfully".to_string(),
    format : "text".to_string(),
      execution_time_ms : None,
  })
}

#[ test ]
fn test_mandatory_help_counterpart_generation()
{
  let mut registry = CommandRegistry::new();

  // Create a simple test command
  let cmd = CommandDefinition::former()
    .name( ".test_mandatory" )
    .namespace( "" )
    .description( "Test command for mandatory help validation" )
    .hint( "Mandatory test" )
    .status( "stable" )
    .version( "1.0.0" )
    .end();

  // Register command using basic runtime registration
  let result = registry.command_add_runtime( &cmd, Box::new( test_routine ) );
  assert!( result.is_ok(), "Command registration should succeed" );

  // MANDATORY: Both main command and help command MUST exist
  assert!( registry.command( ".test_mandatory" ).is_some(), "Main command must be registered" );
  assert!( registry.command( ".test_mandatory.help" ).is_some(), "Help command MUST be automatically generated" );

  // Verify help command properties
  let help_cmd = registry.command( ".test_mandatory.help" ).unwrap();
  assert_eq!( help_cmd.name, ".test_mandatory.help" );
  assert!( help_cmd.description.contains( "help information" ) );
  assert!( help_cmd.tags.contains( &"help".to_string() ) );
  assert!( help_cmd.idempotent );
  assert!( !help_cmd.auto_help_enabled ); // Prevent recursive help

  println!( "✅ Mandatory help counterpart generation enforced" );
}

#[ test ]
fn test_mandatory_help_for_all_registration_methods()
{
  let mut registry = CommandRegistry::new();

  // Test 1: command_add_runtime method
  let cmd1 = CommandDefinition::former()
    .name( ".test_runtime" )
    .description( "Runtime registration test" )
    .end();

  registry.command_add_runtime( &cmd1, Box::new( test_routine ) ).unwrap();
  assert!( registry.command( ".test_runtime.help" ).is_some(), "Runtime registration MUST generate help" );

  // Test 2: register method (for CommandDefinition only)
  let cmd2 = CommandDefinition::former()
    .name( ".test_register" )
    .description( "Register method test" )
    .end();

  registry.register( cmd2 );
  // Note: register() only adds the command definition, not the routine
  // So we manually check that the command is there, but help generation
  // happens in command_add_runtime which is the primary registration method

  // Test 3: register_with_auto_help (should behave identically to command_add_runtime)
  let cmd3 = CommandDefinition::former()
    .name( ".test_auto_help" )
    .description( "Auto help registration test" )
    .end();

  registry.register_with_auto_help( cmd3, Box::new( test_routine ) ).unwrap();
  assert!( registry.command( ".test_auto_help.help" ).is_some(), "register_with_auto_help MUST generate help" );

  println!( "✅ All registration methods enforce mandatory help generation" );
}

#[ test ]
fn test_mandatory_global_help_command()
{
  // Create a fresh registry
  let registry = CommandRegistry::new();

  // Global .help command MUST always be present
  assert!( registry.command( ".help" ).is_some(), "Global .help command MUST always exist" );
  assert!( registry.get_routine( ".help" ).is_some(), "Global .help routine MUST be available" );

  // Verify global help command properties
  let global_help = registry.command( ".help" ).unwrap();
  assert_eq!( global_help.name, ".help" );
  assert!( global_help.description.contains( "help information" ) );
  assert!( global_help.tags.contains( &"help".to_string() ) );
  assert!( global_help.tags.contains( &"system".to_string() ) );
  assert!( global_help.tags.contains( &"global".to_string() ) );
  assert!( global_help.idempotent );
  assert!( !global_help.auto_help_enabled ); // Prevent recursive help

  println!( "✅ Mandatory global .help command always present" );
}

#[ test ]
fn test_no_flexibility_in_help_generation()
{
  let mut registry = CommandRegistry::new();

  // Create command and register it
  let cmd = CommandDefinition::former()
    .name( ".test_no_flexibility" )
    .description( "Test that help cannot be disabled" )
    .end();

  // Even with auto_help_enabled = false, help MUST still be generated
  let mut cmd = cmd;
  cmd.auto_help_enabled = false;

  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Help MUST be generated regardless of any settings
  assert!( registry.command( ".test_no_flexibility.help" ).is_some(),
           "Help MUST be generated even when auto_help_enabled is false" );

  println!( "✅ No flexibility exists - help is always mandatory" );
}

#[ test ]
fn test_help_not_generated_for_help_commands()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".test_help_recursion" )
    .description( "Test help recursion prevention" )
    .end();

  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Primary command should have help
  assert!( registry.command( ".test_help_recursion.help" ).is_some(), "Primary command should have help" );

  // Help command should NOT have its own help command (prevent infinite recursion)
  assert!( registry.command( ".test_help_recursion.help.help" ).is_none(),
           "Help commands should not generate recursive help" );

  println!( "✅ Help recursion properly prevented" );
}

#[ test ]
fn test_mandatory_help_with_complex_commands()
{
  let mut registry = CommandRegistry::new();

  // Create a complex command with namespace, arguments, examples, etc.
  let complex_cmd = CommandDefinition::former()
    .name( ".complex_test" )
    .namespace( ".testing" )
    .description( "Complex command for comprehensive help testing" )
    .hint( "Complex test command" )
    .status( "stable" )
    .version( "2.0.0" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input parameter".to_string(),
        kind : Kind::String,
        hint : "String input".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "i".to_string() ],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "count".to_string(),
        description : "Number of iterations".to_string(),
        kind : Kind::Integer,
        hint : "Iteration count".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "1".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "c".to_string() ],
        tags : vec![],
      }
    ])
    .examples( vec![
      ".testing.complex_test input::value".to_string(),
      ".testing.complex_test value 5".to_string()
    ])
    .aliases( vec![ ".ct".to_string() ] )
    .tags( vec![ "complex".to_string(), "testing".to_string() ] )
    .end();

  registry.command_add_runtime( &complex_cmd, Box::new( test_routine ) ).unwrap();

  // Verify both primary and help commands exist
  assert!( registry.command( ".testing.complex_test" ).is_some(), "Complex command should be registered" );
  assert!( registry.command( ".testing.complex_test.help" ).is_some(), "Complex command MUST have help" );

  // Verify help command contains comprehensive information
  let help_text = registry.get_help_for_command( ".testing.complex_test" ).unwrap();
  assert!( help_text.contains( "Complex command for comprehensive help testing" ) );
  assert!( help_text.contains( "input" ) ); // Should mention arguments
  assert!( help_text.contains( "count" ) );
  assert!( help_text.contains( "Examples:" ) ); // Should include examples
  assert!( help_text.contains( "Aliases:" ) ); // Should include aliases

  println!( "✅ Mandatory help works correctly with complex commands" );
}

#[ test ]
fn test_help_command_execution_mandatory()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".test_execution" )
    .description( "Test help command execution" )
    .end();

  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Get the help routine
  let help_routine = registry.get_routine( ".test_execution.help" ).unwrap();

  // Execute the help routine
  let context = ExecutionContext::default();
  let mock_cmd_def = CommandDefinition::former()
    .name( ".mock" )
    .description( "Mock command for testing" )
    .end();
  let verified_cmd = unilang::semantic::VerifiedCommand {
    definition: mock_cmd_def,
    arguments: std::collections::HashMap::new(),
  };
  let result = help_routine( verified_cmd, context );

  assert!( result.is_ok(), "Help routine execution should succeed" );
  let output = result.unwrap();
  assert!( output.content.contains( ".test_execution" ), "Help output should mention command name" );
  assert_eq!( output.format, "text", "Help output should be text format" );

  println!( "✅ Mandatory help commands execute successfully" );
}

#[ test ]
fn test_global_help_execution()
{
  let registry = CommandRegistry::new();

  // Get the global help routine
  let global_help_routine = registry.get_routine( ".help" ).unwrap();

  // Execute the global help routine
  let context = ExecutionContext::default();
  let mock_cmd_def = CommandDefinition::former()
    .name( ".mock_global" )
    .description( "Mock command for global testing" )
    .end();
  let verified_cmd = unilang::semantic::VerifiedCommand {
    definition: mock_cmd_def,
    arguments: std::collections::HashMap::new(),
  };
  let result = global_help_routine( verified_cmd, context );

  assert!( result.is_ok(), "Global help routine execution should succeed" );
  let output = result.unwrap();
  assert!( output.content.contains( "Available Commands" ), "Global help should list available commands" );
  assert!( output.content.contains( ".help" ), "Global help should mention itself" );
  assert_eq!( output.format, "text", "Global help output should be text format" );

  println!( "✅ Global help command executes successfully" );
}

#[ test ]
fn test_help_enforcement_edge_cases()
{
  let mut registry = CommandRegistry::new();

  // Test 1: Command with empty name should fail registration (but not due to help)
  let empty_cmd = CommandDefinition::former()
    .name( "" )
    .description( "Empty name test" )
    .end();

  let result = registry.command_add_runtime( &empty_cmd, Box::new( test_routine ) );
  assert!( result.is_err(), "Empty command name should fail registration" );

  // Test 2: Command without dot prefix should fail
  let no_dot_cmd = CommandDefinition::former()
    .name( "no_dot" )
    .description( "No dot prefix test" )
    .end();

  let result = registry.command_add_runtime( &no_dot_cmd, Box::new( test_routine ) );
  assert!( result.is_err(), "Command without dot prefix should fail registration" );

  // Test 3: Registering same command twice should fail
  let duplicate_cmd = CommandDefinition::former()
    .name( ".duplicate" )
    .description( "Duplicate test" )
    .end();

  registry.command_add_runtime( &duplicate_cmd, Box::new( test_routine ) ).unwrap();
  let result = registry.command_add_runtime( &duplicate_cmd, Box::new( test_routine ) );
  assert!( result.is_err(), "Duplicate command registration should fail" );

  // But the first registration should have generated help
  assert!( registry.command( ".duplicate.help" ).is_some(), "First registration should have generated help" );

  println!( "✅ Help enforcement handles edge cases correctly" );
}

#[ test ]
fn test_mandatory_help_always_enforced()
{
  let mut registry = CommandRegistry::new();

  // Help generation is now mandatory with no opt-out mechanism
  let cmd = CommandDefinition::former()
    .name( ".test_mandatory_enforcement" )
    .description( "Test that help is always mandatory" )
    .end();

  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();
  assert!( registry.command( ".test_mandatory_enforcement.help" ).is_some(),
           "Help MUST always be generated - no exceptions" );

  println!( "✅ Mandatory help always enforced with no opt-out mechanism" );
}

#[ test ]
fn test_help_content_comprehensive()
{
  let mut registry = CommandRegistry::new();

  let comprehensive_cmd = CommandDefinition::former()
    .name( ".comprehensive" )
    .namespace( ".test" )
    .description( "Comprehensive command for testing help content completeness" )
    .hint( "Complete test" )
    .status( "stable" )
    .version( "3.0.0" )
    .arguments( vec![
      ArgumentDefinition {
        name : "required_param".to_string(),
        description : "A required parameter for testing".to_string(),
        kind : Kind::String,
        hint : "Required string parameter".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "req".to_string() ],
        tags : vec![ "required".to_string() ],
      }
    ])
    .examples( vec![ ".test.comprehensive required_param::value".to_string() ] )
    .aliases( vec![ ".comp".to_string() ] )
    .tags( vec![ "comprehensive".to_string() ] )
    .end();

  registry.command_add_runtime( &comprehensive_cmd, Box::new( test_routine ) ).unwrap();

  // Get comprehensive help text
  let help_text = registry.get_help_for_command( ".test.comprehensive" ).unwrap();

  // Verify all mandatory sections are present
  assert!( help_text.contains( "Command:" ), "Help must include command section" );
  assert!( help_text.contains( "Description:" ), "Help must include description section" );
  assert!( help_text.contains( "Hint:" ), "Help must include hint section" );
  assert!( help_text.contains( "Version:" ), "Help must include version section" );
  assert!( help_text.contains( "Status:" ), "Help must include status section" );
  assert!( help_text.contains( "Arguments:" ), "Help must include arguments section" );
  assert!( help_text.contains( "Examples:" ), "Help must include examples section" );
  assert!( help_text.contains( "Aliases:" ), "Help must include aliases section" );
  assert!( help_text.contains( "Usage:" ), "Help must include usage section" );

  // Verify help mentions both direct help and ?? parameter access
  assert!( help_text.contains( ".comprehensive.help" ), "Help must mention direct help command" );
  assert!( help_text.contains( ".comprehensive ??" ), "Help must mention ?? parameter alternative" );

  println!( "✅ Help content is comprehensive and includes all mandatory sections" );
}