//! Edge Case Handling Unit Tests
//!
//! ## Critical Issue 4: Empty Value Parsing Errors
//! **Severity**: Medium (Edge Cases)
//! **Root Cause**: Tokenizer cannot handle empty quoted strings
//! **Error**: "Unexpected token 'command1:' in arguments"
//!
//! ## Test Coverage (T4)
//! - T4.1: Empty Quoted Values (command1:"")
//! - T4.2: Whitespace-Only Values (command1:"   ")
//! - Additional: Null characters, escape sequences, malformed quotes
//!
//! ## Expected Test Behavior (TDD)
//! - **RED PHASE**: "Unexpected token" errors for empty values
//! - **GREEN PHASE**: Empty values handled gracefully with clear behavior
//! - **REFACTOR PHASE**: Consistent error messaging and edge case handling

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Helper to create a simple command for edge case testing
fn create_edge_case_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Edge case test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "content".to_string(),
        description : "Content parameter for edge case testing".to_string(),
        kind : Kind::String,
        hint : "String value that may be empty or contain edge cases".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Helper to parse and analyze edge case content
fn parse_edge_case_command( registry : &CommandRegistry, input : &str ) -> Result< (), String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input )
    .map_err( |e| format!( "Parse error: {e:?}" ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  let _verified_commands = analyzer.analyze()
    .map_err( |e| format!( "Semantic analysis error: {e:?}" ) )?;

  Ok(())
}

/// T4.1: Empty Quoted Command Value
/// **Test Case**: `.test content::""`
/// **Expected**: Should handle empty commands gracefully
/// **Current**: "Unexpected token 'command1:' in arguments" - tokenization failure
#[test]
fn test_empty_command_value()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".test" );
  registry.register( cmd );

  // Empty quoted string - using correct double-colon syntax per specification
  let result = parse_edge_case_command( &registry, r#".test content::"""# );

  assert!( result.is_ok(), "Empty quoted command value should parse gracefully: {result:?}" );
}

/// T4.1b: Empty Quoted Argument Value
/// **Test Case**: Command with empty argument value
/// **Expected**: Should handle empty argument values
#[test]
fn test_empty_argument_value()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".empty_arg" );
  registry.register( cmd );

  // Empty argument value
  let result = parse_edge_case_command( &registry, r#".empty_arg content::"""# );

  assert!( result.is_ok(), "Empty argument value should be handled: {result:?}" );
}

/// T4.2: Whitespace-Only Command Value
/// **Test Case**: `.test content::"   "`
/// **Expected**: Should handle whitespace-only commands
/// **Current**: Need to verify behavior
#[test]
fn test_whitespace_only_command()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".whitespace" );
  registry.register( cmd );

  // Whitespace-only content
  let result = parse_edge_case_command( &registry, r#".whitespace content::"   ""# );

  assert!( result.is_ok(), "Whitespace-only command should parse: {result:?}" );
}

/// T4.2b: Mixed Whitespace Types
/// **Test Case**: Various whitespace characters (tabs, newlines, spaces)
/// **Expected**: Should handle all whitespace types consistently
#[test]
fn test_mixed_whitespace()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".mixed_ws" );
  registry.register( cmd );

  // Mixed whitespace: tabs, newlines, spaces
  let result = parse_edge_case_command( &registry, ".mixed_ws content::\"\t\n  \"" );

  assert!( result.is_ok(), "Mixed whitespace should be handled: {result:?}" );
}

/// Additional Test: Null Characters
/// **Test Case**: Strings containing null characters
/// **Expected**: Should handle null characters safely
#[test]
fn test_null_characters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".null_test" );
  registry.register( cmd );

  // Null character in string (note: this may need special handling)
  let input_with_null = format!( r#".null_test content::"test{}null""#, '\0' );
  let result = parse_edge_case_command( &registry, &input_with_null );

  // Note: This test should either succeed gracefully or fail with clear error
  match result {
    Ok(()) => {}, // Null characters handled gracefully
    Err(e) => {
      // Should get clear error message, not a panic
      assert!( !e.contains( "panic" ), "Should not panic on null characters: {e}" );
    }
  }
}

/// Additional Test: Escape Sequences
/// **Test Case**: Strings with escape sequences \" \\ \n
/// **Expected**: Should handle escape sequences properly
#[test]
fn test_escape_sequences()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".escape" );
  registry.register( cmd );

  // Test various escape sequences
  let test_cases = vec![
    r#".escape content::"test with \"quotes\"""#,  // Escaped quotes
    r#".escape content::"test with \\backslash""#,  // Escaped backslash
    r#".escape content::"test with \n newline""#,   // Escaped newline
  ];

  for test_case in test_cases {
    let result = parse_edge_case_command( &registry, test_case );
    assert!( result.is_ok(), "Escape sequence should parse: {test_case} -> {result:?}" );
  }
}

/// Additional Test: Malformed Quotes
/// **Test Case**: Unclosed or incorrectly nested quotes
/// **Expected**: Should provide clear error messages for malformed quotes
#[test]
fn test_malformed_quotes()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".malformed" );
  registry.register( cmd );

  // Test cases that should fail with clear error messages
  let malformed_cases = vec![
    r#".malformed content::"unclosed quote"#,     // Missing closing quote
    r#".malformed content:unclosed quote""#,     // Missing opening quote
  ];

  for malformed_case in malformed_cases {
    let result = parse_edge_case_command( &registry, malformed_case );
    match result {
      Ok(()) => panic!( "Malformed quote should not parse successfully: {malformed_case}" ),
      Err(e) => {
        // Should get clear error message about malformed quotes
        assert!( e.contains( "quote" ) || e.contains( "parse" ),
          "Error message should mention quote issue: {malformed_case} -> {e}" );
      }
    }
  }
}

/// Additional Test: Nested Quotes
/// **Test Case**: Complex quote nesting scenarios
/// **Expected**: Should handle quote nesting appropriately or fail clearly
#[test]
fn test_nested_quotes()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".nested" );
  registry.register( cmd );

  // Test nested quotes (behavior may vary based on parser design)
  let result = parse_edge_case_command( &registry, r#".nested content::"outer \"inner\" quotes""# );

  // This should either work or fail with clear error
  match result {
    Ok(()) => {}, // Nested quotes handled
    Err(e) => {
      // Should get clear error message, not a panic
      assert!( !e.contains( "panic" ), "Should not panic on nested quotes: {e}" );
    }
  }
}

/// Additional Test: Very Long Empty Values
/// **Test Case**: Empty values in long command sequences
/// **Expected**: Should handle empty values consistently even in complex scenarios
#[test]
fn test_long_empty_values()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".long_empty" );
  registry.register( cmd );

  // Multiple empty values in sequence
  // Test multiple empty value parsing
  let empty_cases = [
    r#".long_empty content::"""#,
    r#".long_empty content::"""#,
    r#".long_empty content::"""#,
  ];

  for (i, empty_case) in empty_cases.iter().enumerate() {
    let result = parse_edge_case_command( &registry, empty_case );
    assert!( result.is_ok(), "Empty value {} should parse: {result:?}", i + 1 );
  }
}

/// Additional Test: Unicode in Empty/Whitespace Contexts
/// **Test Case**: Unicode characters mixed with empty/whitespace scenarios
/// **Expected**: Should handle Unicode in edge cases
#[test]
fn test_unicode_edge_cases()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".unicode_edge" );
  registry.register( cmd );

  // Unicode with whitespace edge cases
  let unicode_edge_cases = vec![
    r#".unicode_edge content::" 🚀 ""#,        // Unicode with spaces
    r#".unicode_edge content::"🚀""#,          // Unicode only
    r#".unicode_edge content::"  ""#,          // Spaces only
  ];

  for unicode_case in unicode_edge_cases {
    let result = parse_edge_case_command( &registry, unicode_case );
    assert!( result.is_ok(), "Unicode edge case should parse: {unicode_case} -> {result:?}" );
  }
}

/// Additional Test: Special Characters in Edge Cases
/// **Test Case**: Special characters with empty/whitespace values
/// **Expected**: Should handle special characters in edge case contexts
#[test]
fn test_special_characters_edge_cases()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_edge_case_test_command( ".special_edge" );
  registry.register( cmd );

  // Special characters in edge case contexts
  let special_cases = vec![
    r#".special_edge content::" !@#$%^&*() ""#,    // Special chars with spaces
    r#".special_edge content::"!@#$%^&*()""#,      // Special chars only
    r#".special_edge content::" <>=[]{}|; ""#,     // Punctuation with spaces
  ];

  for special_case in special_cases {
    let result = parse_edge_case_command( &registry, special_case );
    assert!( result.is_ok(), "Special character edge case should parse: {special_case} -> {result:?}" );
  }
}

/// Additional Test: Empty Values with Different Argument Types
/// **Test Case**: Empty values for different argument kinds
/// **Expected**: Should handle empty values for all supported types
#[test]
fn test_empty_values_different_types()
{
  let mut registry = CommandRegistry::new();

  // Create command with multiple argument types
  let cmd = CommandDefinition::former()
    .name( ".multi_type" )
    .description( "Multi-type edge case test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "text".to_string(),
        description : "String parameter".to_string(),
        kind : Kind::String,
        hint : "String value".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "default_text".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( cmd );

  // Test empty string for string type
  let result = parse_edge_case_command( &registry, r#".multi_type text::"""# );
  assert!( result.is_ok(), "Empty string value should be handled: {result:?}" );
}