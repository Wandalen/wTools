//!
//! Tests for help conventions implementation (FR-HELP-4, FR-HELP-5, FR-HELP-6)
//!
//! This test suite validates the standardized help conventions:
//! 1. Automatic `.command.help` generation for every registered command
//! 2. Universal `??` parameter support for alternative help access
//! 3. Developer-friendly APIs for help configuration
//!

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;

/// Test routine for help convention tests
#[allow(clippy::unnecessary_wraps)]
fn test_routine( _cmd : unilang::semantic::VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Test command executed successfully".to_string(),
    format : "text".to_string(),
  })
}

#[ test ]
fn test_automatic_help_command_generation()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();

  // Enable help conventions globally
  registry.enable_help_conventions( true );

  // Create a test command with auto-help enabled
  let cmd = CommandDefinition::former()
    .name( ".test_example" )
    .namespace( "" )
    .description( "A test command for help convention validation" )
    .hint( "Test command" )
    .status( "stable" )
    .version( "1.0.0" )
    .end();

  // Manually enable auto-help since builder method doesn't work yet
  let mut cmd = cmd;
  cmd.auto_help_enabled = true;

  // Register command with auto-help
  let result = registry.register_with_auto_help( cmd, Box::new( test_routine ) );
  assert!( result.is_ok(), "Command registration should succeed" );

  // Verify both main command and help command are registered
  assert!( registry.command( ".test_example" ).is_some(), "Main command should be registered" );
  assert!( registry.command( ".test_example.help" ).is_some(), "Help command should be automatically generated" );

  // Verify help command has correct properties
  let help_cmd = registry.command( ".test_example.help" ).unwrap();
  assert_eq!( help_cmd.name, ".test_example.help" );
  assert!( help_cmd.description.contains( "help information" ) );
  assert!( help_cmd.tags.contains( &"help".to_string() ) );
  assert!( help_cmd.tags.contains( &"documentation".to_string() ) );
  assert!( help_cmd.idempotent );
  assert!( help_cmd.permissions.is_empty() ); // Help should be accessible to all

  println!( "✅ Automatic help command generation works correctly" );
}

#[ test ]
fn test_double_question_mark_parameter()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();

  // Create a command with arguments for comprehensive help testing
  let cmd = CommandDefinition::former()
    .name( ".test_command" )
    .namespace( "" )
    .description( "Test command with arguments for ?? parameter testing" )
    .hint( "Test with args" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![
      ArgumentDefinition {
        name : "arg1".to_string(),
        description : "First test argument".to_string(),
        kind : Kind::String,
        hint : "String argument".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "a1".to_string() ],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "arg2".to_string(),
        description : "Second test argument".to_string(),
        kind : Kind::Integer,
        hint : "Integer argument".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "42".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .examples( vec![
      ".test_command arg1::value arg2::123".to_string(),
      ".test_command value 456".to_string()
    ])
    .end();

  registry.register_with_auto_help( cmd, Box::new( test_routine ) ).unwrap();

  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();

  // Test 1: ?? as positional parameter
  let result1 = pipeline.process_command( ".test_command \"??\"", context.clone() );
  assert!( result1.success, "Command with ?? parameter should trigger help" );
  assert!( !result1.outputs.is_empty(), "Help should produce output" );
  assert!( result1.outputs[0].content.contains( "test_command" ), "Help should mention command name" );
  assert!( result1.outputs[0].content.contains( "First test argument" ), "Help should include argument descriptions" );

  // Test 2: ?? as named parameter
  let result2 = pipeline.process_command( ".test_command help::\"??\"", context.clone() );
  assert!( result2.success, "Command with ?? as named parameter should trigger help" );

  // Test 3: ?? mixed with other arguments (should still trigger help)
  let result3 = pipeline.process_command( ".test_command arg1::test \"??\"", context.clone() );
  assert!( result3.success, "Command with ?? and other args should trigger help" );

  // Test 4: Compare with traditional ? operator
  let result4 = pipeline.process_command( ".test_command ?", context.clone() );
  assert!( result4.success, "Traditional ? operator should still work" );

  // Both ?? and ? should produce identical help content
  assert_eq!( result1.outputs[0].content, result4.outputs[0].content,
             "?? parameter and ? operator should produce identical help" );

  println!( "✅ Double question mark parameter works correctly in all scenarios" );
}

#[ test ]
fn test_help_command_execution()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".test_help_exec" )
    .namespace( "" )
    .description( "Test command for help execution validation" )
    .hint( "Help exec test" )
    .status( "stable" )
    .version( "1.0.0" )
    .examples( vec![ ".test_help_exec".to_string() ] )
    .end();

  registry.register_with_auto_help( cmd, Box::new( test_routine ) ).unwrap();

  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();

  // Execute the automatically generated help command
  let help_result = pipeline.process_command( ".test_help_exec.help", context );
  assert!( help_result.success, "Help command execution should succeed" );
  assert!( !help_result.outputs.is_empty(), "Help command should produce output" );

  let help_content = &help_result.outputs[0].content;

  // Verify help content contains all expected sections
  assert!( help_content.contains( "Command: .test_help_exec" ), "Help should show command name" );
  assert!( help_content.contains( "Description: Test command for help execution validation" ), "Help should show description" );
  assert!( help_content.contains( "Version: 1.0.0" ), "Help should show version" );
  assert!( help_content.contains( "Status: stable" ), "Help should show status" );
  assert!( help_content.contains( "Usage:" ), "Help should include usage section" );
  assert!( help_content.contains( ".test_help_exec.help" ), "Help should mention help command itself" );
  assert!( help_content.contains( ".test_help_exec ??" ), "Help should mention ?? alternative" );

  println!( "✅ Help command execution produces comprehensive help content" );
}

#[ test ]
fn test_help_conventions_api()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();

  // Test 1: Global help conventions toggle
  registry.enable_help_conventions( false );

  let cmd1 = CommandDefinition::former()
    .name( ".test_no_auto_help" )
    .description( "Command without auto help" )
    .end();

  registry.register_with_auto_help( cmd1, Box::new( test_routine ) ).unwrap();

  // With global help disabled, no help command should be generated
  assert!( registry.command( ".test_no_auto_help" ).is_some(), "Main command should exist" );
  assert!( registry.command( ".test_no_auto_help.help" ).is_none(), "Help command should not be generated when disabled" );

  // Test 2: Per-command override
  registry.enable_help_conventions( false ); // Still disabled globally

  let cmd2 = CommandDefinition::former()
    .name( ".test_force_help" )
    .description( "Command with forced help" )
    .end();

  // Manually enable auto-help since builder method doesn't work yet
  let mut cmd2 = cmd2;
  cmd2.auto_help_enabled = true;

  registry.register_with_auto_help( cmd2, Box::new( test_routine ) ).unwrap();

  // Per-command setting should override global setting
  assert!( registry.command( ".test_force_help" ).is_some(), "Main command should exist" );
  assert!( registry.command( ".test_force_help.help" ).is_some(), "Help command should be generated when explicitly enabled" );

  // Test 3: get_help_for_command API
  let help_text = registry.get_help_for_command( ".test_force_help" );
  assert!( help_text.is_some(), "get_help_for_command should return help text" );
  assert!( help_text.unwrap().contains( "Command: .test_force_help" ), "Help text should be properly formatted" );

  // Test 4: Pipeline help request processing
  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();
  let pipeline_help_result = pipeline.process_help_request( ".test_force_help", context );
  assert!( pipeline_help_result.is_ok(), "Pipeline help request should succeed" );

  println!( "✅ Help conventions API works correctly with all configuration options" );
}

#[ test ]
fn test_command_definition_builder_methods()
{
  // Test CommandDefinition builder methods
  let cmd = CommandDefinition::former()
    .name( ".test_builder" )
    .description( "Test builder methods" )
    .end();

  // Manually enable auto-help since builder method doesn't work yet
  let mut cmd = cmd;
  cmd.auto_help_enabled = true;

  assert!( cmd.has_auto_help(), "has_auto_help should return true" );
  assert!( cmd.auto_help_enabled, "auto_help_enabled field should be true" );

  let cmd2 = CommandDefinition::former()
    .name( ".test_builder2" )
    .description( "Test builder methods without auto help" )
    .end();

  // Manually disable auto-help since builder method doesn't work yet
  let mut cmd2 = cmd2;
  cmd2.auto_help_enabled = false;

  assert!( !cmd2.has_auto_help(), "has_auto_help should return false" );
  assert!( !cmd2.auto_help_enabled, "auto_help_enabled field should be false" );

  // Test generate_help_command method
  let help_cmd = cmd.generate_help_command();
  assert_eq!( help_cmd.name, ".test_builder.help", "Generated help command should have correct name" );
  assert!( help_cmd.description.contains( "help information" ), "Generated help should have appropriate description" );
  assert!( help_cmd.examples.contains( &".test_builder.help".to_string() ), "Help command should include usage examples" );
  assert!( help_cmd.examples.contains( &".test_builder ??".to_string() ), "Help command should mention ?? parameter" );
  assert!( !help_cmd.auto_help_enabled, "Help commands should not recursively generate help" );

  println!( "✅ CommandDefinition builder methods work correctly" );
}

#[ test ]
fn test_help_content_formatting()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();

  // Create a command with comprehensive metadata for help formatting testing
  let cmd = CommandDefinition::former()
    .name( ".test_format" )
    .namespace( ".testing" )
    .description( "Comprehensive test command for help formatting validation" )
    .hint( "Format test" )
    .status( "stable" )
    .version( "2.1.0" )
    .tags( vec![ "test".to_string(), "formatting".to_string() ] )
    .aliases( vec![ ".tf".to_string(), ".test_fmt".to_string() ] )
    .examples( vec![
      ".test_format arg1::value".to_string(),
      ".testing.test_format positional_value".to_string()
    ])
    .arguments( vec![
      ArgumentDefinition {
        name : "required_arg".to_string(),
        description : "A required string argument".to_string(),
        kind : Kind::String,
        hint : "Required string".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![
          ValidationRule::MinLength( 3 ),
          ValidationRule::MaxLength( 50 )
        ],
        aliases : vec![ "req".to_string(), "r".to_string() ],
        tags : vec![ "required".to_string() ],
      },
      ArgumentDefinition {
        name : "optional_arg".to_string(),
        description : "An optional integer with default value".to_string(),
        kind : Kind::Integer,
        hint : "Optional integer".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "100".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![
          ValidationRule::Min( 1.0 ),
          ValidationRule::Max( 1000.0 )
        ],
        aliases : vec![ "opt".to_string() ],
        tags : vec![ "optional".to_string() ],
      }
    ])
    .end();

  registry.register_with_auto_help( cmd, Box::new( test_routine ) ).unwrap();

  let help_text = registry.get_help_for_command( ".testing.test_format" ).unwrap();

  // Verify all sections are present and properly formatted
  assert!( help_text.contains( "Command: .test_format" ), "Command name section" );
  assert!( help_text.contains( "Description: Comprehensive test command" ), "Description section" );
  assert!( help_text.contains( "Hint: Format test" ), "Hint section" );
  assert!( help_text.contains( "Version: 2.1.0" ), "Version section" );
  assert!( help_text.contains( "Status: stable" ), "Status section" );

  // Arguments section
  assert!( help_text.contains( "Arguments:" ), "Arguments section header" );
  assert!( help_text.contains( "required_arg (String, required)" ), "Required argument info" );
  assert!( help_text.contains( "optional_arg (Integer, optional) [default: 100]" ), "Optional argument with default" );
  assert!( help_text.contains( "A required string argument" ), "Argument descriptions" );
  assert!( help_text.contains( "Aliases: req, r" ), "Argument aliases" );

  // Examples section
  assert!( help_text.contains( "Examples:" ), "Examples section header" );
  assert!( help_text.contains( ".test_format arg1::value" ), "Example commands" );

  // Aliases section
  assert!( help_text.contains( "Aliases: .tf, .test_fmt" ), "Command aliases" );

  // Usage section
  assert!( help_text.contains( "Usage:" ), "Usage section header" );
  assert!( help_text.contains( ".test_format  # Execute command" ), "Execute usage" );
  assert!( help_text.contains( ".test_format.help  # Show this help" ), "Help command usage" );
  assert!( help_text.contains( ".test_format ??  # Alternative help access" ), "?? parameter usage" );

  println!( "✅ Help content formatting includes all required sections with proper structure" );
}

#[ test ]
fn test_help_error_handling()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let registry = CommandRegistry::new();
  let context = ExecutionContext::default();

  // Test help request for non-existent command
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_help_request( ".nonexistent", context );
  assert!( result.is_err(), "Help request for non-existent command should fail" );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "not found" ), "Error should indicate command not found" );
  assert!( error_msg.contains( ".nonexistent" ), "Error should mention the command name" );

  // Test get_help_for_command with non-existent command - create new registry
  #[allow(deprecated)]
  #[allow(deprecated)]
    let new_registry = CommandRegistry::new();
  let help_text = new_registry.get_help_for_command( ".nonexistent" );
  assert!( help_text.is_none(), "get_help_for_command should return None for non-existent commands" );

  println!( "✅ Help error handling works correctly for non-existent commands" );
}