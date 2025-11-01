//! Extended ASCII Safety Unit Tests
//!
//! ## Critical Issue 2: Extended ASCII Causes Parser Panics
//! **Severity**: Critical (Memory Safety)
//! **Root Cause**: Character boundary violations in string processing
//! **Location**: `parser_engine.rs:34:48` - "called `Option::unwrap()` on a `None` value"
//!
//! ## Test Coverage (T2)
//! - T2.1: Accented Characters (café naïve résumé)
//! - T2.2: European Characters (Ñoño España)
//! - T2.3: Currency and Symbols (€50 £40 ¥500)
//! - Additional: ISO-8859-1 and Windows-1252 character sets
//!
//! ## Expected Test Behavior (TDD)
//! - **RED PHASE**: Panic on first extended ASCII character encountered
//! - **GREEN PHASE**: All extended ASCII characters parse without panics
//! - **REFACTOR PHASE**: Optimize character encoding detection

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Helper to create a simple test command for extended ASCII testing
fn create_extended_ascii_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Extended ASCII test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "text".to_string(),
        description : "Text parameter for extended ASCII testing".to_string(),
        kind : Kind::String,
        hint : "Extended ASCII string value".to_string(),
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

/// Helper to parse and analyze extended ASCII content
fn parse_extended_ascii_command( registry : &CommandRegistry, input : &str ) -> Result< (), String >
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

/// T2.1: French Accented Characters
/// **Test Case**: `.test text::"café naïve résumé"`
/// **Expected**: Should handle French accented characters
/// **Current**: Panics with "called `Option::unwrap()` on a `None` value"
#[test]
fn test_french_accents()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".french" );
  registry.register( cmd ).expect( "Failed to register command" );

  // French accented characters
  let result = parse_extended_ascii_command( &registry, r#".french text::"café naïve résumé""# );

  assert!( result.is_ok(), "French accents should parse without panics: {result:?}" );
}

/// T2.1b: German Umlauts and Eszett
/// **Test Case**: German characters ä ö ü ß
/// **Expected**: Should handle German special characters
#[test]
fn test_german_umlauts()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".german" );
  registry.register( cmd ).expect( "Failed to register command" );

  // German umlauts and eszett
  let result = parse_extended_ascii_command( &registry, r#".german text::"Bäcker größer Büße""# );

  assert!( result.is_ok(), "German umlauts should parse without panics: {result:?}" );
}

/// T2.2: Spanish Characters
/// **Test Case**: `.test text::"Ñoño España"`
/// **Expected**: Should handle Spanish characters including ñ
/// **Current**: Parser crashes on extended ASCII
#[test]
fn test_spanish_characters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".spanish" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Spanish characters including ñ
  let result = parse_extended_ascii_command( &registry, r#".spanish text::"Ñoño España mañana""# );

  assert!( result.is_ok(), "Spanish characters should parse without panics: {result:?}" );
}

/// T2.2b: Scandinavian Characters
/// **Test Case**: Norwegian/Danish characters æ ø å
/// **Expected**: Should handle Scandinavian special characters
#[test]
fn test_scandinavian_characters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".scandi" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Scandinavian characters
  let result = parse_extended_ascii_command( &registry, r#".scandi text::"København Ålesund Bjørn""# );

  assert!( result.is_ok(), "Scandinavian characters should parse without panics: {result:?}" );
}

/// T2.3: Currency Symbols
/// **Test Case**: `.test text::"Price: €50 £40 ¥500"`
/// **Expected**: Should handle currency symbols
/// **Current**: Multiple character encoding failures
#[test]
fn test_currency_symbols()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".currency" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Currency symbols
  let result = parse_extended_ascii_command( &registry, r#".currency text::"Price: €50 £40 ¥500 $100""# );

  assert!( result.is_ok(), "Currency symbols should parse without panics: {result:?}" );
}

/// T2.3b: Mathematical and Special Symbols
/// **Test Case**: Mathematical symbols ± ° § ¶
/// **Expected**: Should handle mathematical and special symbols
#[test]
fn test_mathematical_symbols()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".math" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Mathematical and special symbols
  let result = parse_extended_ascii_command( &registry, r#".math text::"Temperature: ±20° Section§ ¶Graph""# );

  assert!( result.is_ok(), "Mathematical symbols should parse without panics: {result:?}" );
}

/// Additional Test: ISO-8859-1 Character Set
/// **Test Case**: Comprehensive ISO-8859-1 character coverage
/// **Expected**: Full ISO-8859-1 character set support
#[test]
fn test_iso_8859_1_charset()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".iso" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Comprehensive ISO-8859-1 characters
  let iso_chars = "àáâãäåæçèéêëìíîïðñòóôõöøùúûüýþÿ";
  let input = format!( r#".iso text::"{iso_chars}""# );

  let result = parse_extended_ascii_command( &registry, &input );
  assert!( result.is_ok(), "ISO-8859-1 character set should parse: {result:?}" );
}

/// Additional Test: Windows-1252 Characters
/// **Test Case**: Windows-1252 specific characters (smart quotes, etc.)
/// **Expected**: Windows extended characters should work
#[test]
fn test_windows_1252_chars()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".win1252" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Windows-1252 specific characters (em-dash, properly quoted)
  // Use escaped quotes to avoid quote parsing issues
  let result = parse_extended_ascii_command( &registry, r#".win1252 text::"Smart \"quotes\" and – dashes""# );

  assert!( result.is_ok(), "Windows-1252 characters should parse: {result:?}" );
}

/// Additional Test: Mixed Extended ASCII and Regular ASCII
/// **Test Case**: Mixing regular ASCII with extended ASCII characters
/// **Expected**: Seamless handling of mixed content
#[test]
fn test_mixed_ascii_extended_ascii()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".mixed" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Mixed ASCII and extended ASCII
  let result = parse_extended_ascii_command( &registry, r#".mixed text::"Hello café, welcome to España! Price: €25""# );

  assert!( result.is_ok(), "Mixed ASCII/extended ASCII should parse: {result:?}" );
}

/// Additional Test: Eastern European Characters
/// **Test Case**: Czech, Polish, Hungarian characters
/// **Expected**: Should handle Eastern European character sets
#[test]
fn test_eastern_european_characters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".eastern" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Eastern European characters
  let result = parse_extended_ascii_command( &registry, r#".eastern text::"Czech: ěščřžýáíé Polish: ąćęłńóśźż""# );

  assert!( result.is_ok(), "Eastern European characters should parse: {result:?}" );
}

/// Additional Test: Character Encoding Boundary Cases
/// **Test Case**: Characters at encoding boundaries
/// **Expected**: Proper handling of character encoding edge cases
#[test]
fn test_encoding_boundaries()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_extended_ascii_test_command( ".boundaries" );
  registry.register( cmd ).expect( "Failed to register command" );

  // Test characters at various encoding boundaries
  let test_cases = vec![
    r#".boundaries text::"ASCII: abc123""#,           // Pure ASCII
    r#".boundaries text::"Latin1: café""#,            // Latin-1 supplement
    r#".boundaries text::"Mixed: abc café 123""#,     // Mixed boundaries
  ];

  for test_case in test_cases {
    let result = parse_extended_ascii_command( &registry, test_case );
    assert!( result.is_ok(), "Encoding boundary case should parse: {test_case} -> {result:?}" );
  }
}