//! Unicode Safety Unit Tests
//!
//! ## Critical Issue 1: Unicode Content Causes Parser Panics
//! **Severity**: Critical (Memory Safety)
//! **Root Cause**: Parser engine fails on multi-byte UTF-8 characters
//! **Location**: `parser_engine.rs:153:48` - "called `Option::unwrap()` on a `None` value"
//!
//! ## Test Coverage (T1)
//! - T1.1: Basic Unicode Characters (🚀 rocket emoji)
//! - T1.2: Unicode in Multiple Commands
//! - T1.3: Mixed ASCII and Unicode Content
//! - Additional: Unicode boundary detection and memory safety
//!
//! ## Expected Test Behavior (TDD)
//! - **RED PHASE**: All tests must fail with `Option::unwrap()` panic
//! - **GREEN PHASE**: Tests pass after implementing safe UTF-8 character iteration
//! - **REFACTOR PHASE**: Optimize for performance while maintaining safety

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Helper to create a simple test command
fn create_unicode_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Unicode test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "content".to_string(),
        description : "Content parameter for unicode testing".to_string(),
        kind : Kind::String,
        hint : "Unicode string value".to_string(),
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

/// Helper to parse and analyze unicode content
fn parse_unicode_command( registry : &CommandRegistry, input : &str ) -> Result< (), String >
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

/// T1.1: Basic Unicode Characters
/// **Test Case**: `.test content::"echo 🚀 rocket"`
/// **Expected**: Should parse emoji without panics
/// **Current**: Panics with "called `Option::unwrap()` on a `None` value"
#[test]
fn test_basic_unicode_emoji()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".test" );
  registry.register( cmd );

  // This should work but currently panics on Unicode emoji
  let result = parse_unicode_command( &registry, r#".test content::"echo 🚀 rocket""# );

  assert!( result.is_ok(), "Basic Unicode emoji should parse without panics: {result:?}" );
}

/// T1.1b: Multiple Unicode Symbols
/// **Test Case**: Various Unicode symbols (✅ ❌ 🎯)
/// **Expected**: Should parse all Unicode symbols without panics
#[test]
fn test_basic_unicode_symbols()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".symbols" );
  registry.register( cmd );

  // Test multiple Unicode symbols
  let result = parse_unicode_command( &registry, r#".symbols content::"Status: ✅ Success ❌ Fail 🎯 Target""# );

  assert!( result.is_ok(), "Multiple Unicode symbols should parse without panics: {result:?}" );
}

/// T1.2: Unicode in Multiple Commands
/// **Test Case**: Multiple commands each containing Unicode
/// **Expected**: Should parse both commands with Unicode content
/// **Current**: Parser crashes on first Unicode character
#[test]
fn test_multiple_commands_with_unicode()
{
  let mut registry = CommandRegistry::new();
  let cmd1 = create_unicode_test_command( ".test1" );
  let cmd2 = create_unicode_test_command( ".test2" );
  registry.register( cmd1 );
  registry.register( cmd2 );

  // This tests multiple commands with Unicode content
  // Currently fails because parser crashes on first Unicode character
  let parser = Parser::new( UnilangParserOptions::default() );

  // Parse first command with Unicode
  let result1 = parser.parse_single_instruction( r#".test1 content::"echo 🎯 test1""# );
  assert!( result1.is_ok(), "First Unicode command should parse: {result1:?}" );

  // Parse second command with Unicode
  let result2 = parser.parse_single_instruction( r#".test2 content::"echo ✅ test2""# );
  assert!( result2.is_ok(), "Second Unicode command should parse: {result2:?}" );
}

/// T1.3: Mixed ASCII and Unicode Content
/// **Test Case**: `.test content::"echo normal text with 🚀 emoji"`
/// **Expected**: Should handle mixed content seamlessly
/// **Current**: Parser fails at Unicode boundary
#[test]
fn test_mixed_ascii_unicode_content()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".mixed" );
  registry.register( cmd );

  // Mixed ASCII and Unicode content in single string
  let result = parse_unicode_command( &registry, r#".mixed content::"echo normal text with 🚀 emoji and more text""# );

  assert!( result.is_ok(), "Mixed ASCII/Unicode content should parse: {result:?}" );
}

/// Additional Test: Unicode Boundary Detection
/// **Test Case**: Unicode characters at string boundaries
/// **Expected**: Proper UTF-8 character boundary handling
#[test]
fn test_unicode_boundary_detection()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".boundary" );
  registry.register( cmd );

  // Test Unicode at start, middle, and end
  let result = parse_unicode_command( &registry, r#".boundary content::"🚀start middle🎯 end✅""# );

  assert!( result.is_ok(), "Unicode boundary detection should work: {result:?}" );
}

/// Additional Test: Complex Unicode Sequences
/// **Test Case**: Multiple multi-byte Unicode characters
/// **Expected**: Handle complex Unicode sequences without memory issues
#[test]
fn test_complex_unicode_sequences()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".complex" );
  registry.register( cmd );

  // Complex Unicode with various byte lengths
  let result = parse_unicode_command( &registry, r#".complex content::"🚀🎯✅❌🔥💯⭐🌟🎉🎊""# );

  assert!( result.is_ok(), "Complex Unicode sequences should parse: {result:?}" );
}

/// Additional Test: UTF-8 Sequence Validation
/// **Test Case**: Test parser resilience with various UTF-8 patterns
/// **Expected**: Safe handling of all valid UTF-8 sequences
#[test]
fn test_utf8_sequence_validation()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".utf8" );
  registry.register( cmd );

  // Various UTF-8 character classes
  let test_cases = vec![
    r#".utf8 content::"ASCII only""#,  // 1-byte UTF-8
    r#".utf8 content::"café naïve""#,  // 2-byte UTF-8
    r#".utf8 content::"中文测试""#,    // 3-byte UTF-8
    r#".utf8 content::"🚀🎯✅""#,     // 4-byte UTF-8
  ];

  for test_case in test_cases {
    let result = parse_unicode_command( &registry, test_case );
    assert!( result.is_ok(), "UTF-8 sequence should parse: {test_case} -> {result:?}" );
  }
}

/// Additional Test: Unicode Memory Safety
/// **Test Case**: Large Unicode strings to test memory handling
/// **Expected**: No memory violations or panics with large Unicode content
#[test]
fn test_unicode_memory_safety()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_unicode_test_command( ".memory" );
  registry.register( cmd );

  // Large Unicode string to test memory safety
  let large_unicode = "🚀".repeat( 100 );
  let input = format!( r#".memory content::"{large_unicode}""# );

  let result = parse_unicode_command( &registry, &input );
  assert!( result.is_ok(), "Large Unicode content should not cause memory issues: {result:?}" );
}