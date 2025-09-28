//! Example of Well-Structured Unit Test
//!
//! This file demonstrates best practices for unit testing in the systematic
//! organization structure. It shows proper patterns for testing individual
//! components in isolation.

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::data::{ OutputData, ErrorData };
use unilang_parser::{ Parser, UnilangParserOptions };

/// Example: Component isolation with clear test structure
///
/// This test demonstrates:
/// - Single responsibility (testing only argument validation)
/// - Clear arrange/act/assert structure
/// - Descriptive naming
/// - Proper error testing
#[test]
fn test_semantic_analyzer_validates_required_arguments()
{
  // Arrange - Set up test data and dependencies
  let mut registry = CommandRegistry::new();

  // Create command with required argument
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command for validation" )
    .arguments( vec![
      ArgumentDefinition {
        name : "required_arg".to_string(),
        description : "A required argument".to_string(),
        kind : Kind::String,
        hint : "Must be provided".to_string(),
        attributes : ArgumentAttributes {
          optional : false,  // This makes it required
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  // Register command with mock routine
  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData {
      content : "success".to_string(),
      format : "text".to_string(),
    })
  });
  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  // Create instruction missing the required argument
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( ".test" ).unwrap(); // No arguments provided

  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  // Act - Exercise the component under test
  let result = analyzer.analyze();

  // Assert - Verify expected behavior
  assert!( result.is_err(), "Should reject command missing required argument" );

  let error = result.unwrap_err();
  assert!( error.len() > 0, "Should provide error details" );

  // Verify error contains relevant information (flexible assertion)
  let error_message = format!( "{:?}", error );
  assert!( error_message.to_lowercase().contains( "required" ) ||
           error_message.to_lowercase().contains( "missing" ),
           "Error should indicate missing required argument: {}", error_message );
}

/// Example: Boundary testing with edge cases
///
/// This test demonstrates:
/// - Testing edge cases and boundaries
/// - Multiple related scenarios in focused test
/// - Data-driven testing approach
#[test]
fn test_argument_parsing_handles_edge_case_values()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Test boundary conditions and edge cases
  let test_cases = vec![
    // Empty string
    (r#".test arg::"""#, ""),

    // String with only whitespace
    (r#".test arg::"   ""#, "   "),

    // String with special characters
    (r#".test arg::"!@#$%^&*()""#, "!@#$%^&*()"),

    // Very long string (boundary test)
    (r#".test arg::"#.to_string() + &"x".repeat(1000) + r#"""#, "x".repeat(1000).as_str()),

    // String with escaped quotes
    (r#".test arg::"contains \"quotes\" inside""#, r#"contains "quotes" inside"#),
  ];

  for (input, expected_value) in test_cases
  {
    // Act
    let result = parser.parse_single_instruction( &input );

    // Assert
    assert!( result.is_ok(), "Should parse edge case input: {}", input );

    let instruction = result.unwrap();
    assert!( instruction.named_arguments.contains_key( "arg" ),
            "Should contain expected argument" );

    let arg_values = &instruction.named_arguments["arg"];
    assert_eq!( arg_values.len(), 1, "Should have exactly one value" );
    assert_eq!( arg_values[0].value, expected_value,
               "Value should match expected for input: {}", input );
  }
}

/// Example: Mock and dependency injection patterns
///
/// This test demonstrates:
/// - Proper mocking of dependencies
/// - Testing component behavior without external dependencies
/// - Verification of interactions
#[test]
fn test_command_registry_runtime_integration()
{
  // Arrange - Create test registry
  let mut registry = CommandRegistry::new();

  // Create a simple test command
  let cmd = CommandDefinition::former()
    .name( ".mock_test" )
    .description( "Mock test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input parameter".to_string(),
        kind : Kind::String,
        hint : "Test input".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  // Mock routine that tracks if it was called
  use std::sync::{ Arc, Mutex };
  let call_count = Arc::new( Mutex::new( 0 ) );
  let call_count_clone = call_count.clone();

  let mock_routine = Box::new( move |cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    // Track that routine was called
    *call_count_clone.lock().unwrap() += 1;

    // Verify command data was passed correctly
    assert_eq!( cmd.definition.name, ".mock_test" );
    assert!( cmd.arguments.contains_key( "input" ) );

    Ok( OutputData {
      content : format!( "Processed: {}", cmd.arguments["input"] ),
      format : "text".to_string(),
    })
  });

  // Act - Register and use command
  registry.command_add_runtime( &cmd, mock_routine ).unwrap();

  // Verify command was registered
  let retrieved_cmd = registry.command( ".mock_test" );
  assert!( retrieved_cmd.is_some(), "Command should be registered" );

  // Verify no calls yet
  assert_eq!( *call_count.lock().unwrap(), 0, "Routine should not be called yet" );

  // The actual routine execution would happen in integration tests
  // This unit test focuses on the registration mechanism
}

/// Example: Property-based testing pattern
///
/// This test demonstrates:
/// - Testing properties that should hold for any input
/// - Robustness testing with generated inputs
/// - Graceful handling of edge cases
#[cfg(feature = "proptest")]
#[test]
fn test_parser_robustness_with_arbitrary_input()
{
  use proptest::prelude::*;

  proptest!( |input in "\\.[a-zA-Z_][a-zA-Z0-9_]* [a-zA-Z_][a-zA-Z0-9_]*::\"[^\"]*\"" | {
    let parser = Parser::new( UnilangParserOptions::default() );

    // Parser should either succeed or fail gracefully
    match parser.parse_single_instruction( &input )
    {
      Ok( instruction ) => {
        // If parsing succeeds, result should be well-formed
        assert!( !instruction.command_name.is_empty(), "Command name should not be empty" );
        assert!( instruction.command_name.starts_with( '.' ), "Command should start with dot" );
      }
      Err( _error ) => {
        // Graceful failure is acceptable for invalid input
        // Parser should not panic or crash
      }
    }
  });
}

/// Example: Error condition testing
///
/// This test demonstrates:
/// - Comprehensive error testing
/// - Verification of error messages
/// - Testing recovery after errors
#[test]
fn test_semantic_analyzer_error_conditions()
{
  let mut registry = CommandRegistry::new();

  // Set up valid command for reference
  let valid_cmd = CommandDefinition::former()
    .name( ".valid" )
    .description( "Valid test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "arg".to_string(),
        description : "Test argument".to_string(),
        kind : Kind::String,
        hint : "Test value".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let mock_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, ErrorData> {
    Ok( OutputData { content : "success".to_string(), format : "text".to_string() })
  });
  registry.command_add_runtime( &valid_cmd, mock_routine ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );

  // Test various error conditions
  let error_cases = vec![
    // Unknown command
    ( ".unknown_command", "unknown command" ),

    // Missing required argument
    ( ".valid", "required" ),

    // Invalid argument name
    ( ".valid invalid_arg::value", "unknown argument" ),
  ];

  for ( input, expected_error_keyword ) in error_cases
  {
    // Parse instruction (should succeed)
    let instruction = parser.parse_single_instruction( input ).unwrap();
    let instructions = [instruction];

    // Semantic analysis should fail
    let analyzer = SemanticAnalyzer::new( &instructions, &registry );
    let result = analyzer.analyze();

    // Verify error
    assert!( result.is_err(), "Should fail for input: {}", input );

    let error = result.unwrap_err();
    let error_text = format!( "{:?}", error ).to_lowercase();
    assert!( error_text.contains( &expected_error_keyword.to_lowercase() ),
            "Error should contain '{}' for input '{}': {:?}",
            expected_error_keyword, input, error );
  }
}

/// Example: Test helper functions
///
/// This demonstrates:
/// - Reusable test utilities
/// - Consistent test data creation
/// - Reducing test code duplication

/// Helper function to create a standard test command
fn create_simple_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Simple test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "value".to_string(),
        description : "Test value".to_string(),
        kind : Kind::String,
        hint : "Any string value".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "default".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Helper function to create a mock routine for testing
fn create_mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync >
{
  Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result< OutputData, ErrorData > {
    Ok( OutputData {
      content : "mock response".to_string(),
      format : "text".to_string(),
    })
  })
}

/// Example: Using helper functions for cleaner tests
#[test]
fn test_with_helper_functions()
{
  // Arrange - Using helpers for cleaner setup
  let mut registry = CommandRegistry::new();
  let cmd = create_simple_test_command( ".helper_test" );
  registry.command_add_runtime( &cmd, create_mock_routine() ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( ".helper_test" ).unwrap();

  // Act
  let instructions = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Assert
  assert!( result.is_ok(), "Should succeed with default argument value" );
  let verified_commands = result.unwrap();
  assert_eq!( verified_commands.len(), 1 );
  assert_eq!( verified_commands[0].definition.name, ".helper_test" );
}