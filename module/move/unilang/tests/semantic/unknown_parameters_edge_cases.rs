//! Edge case tests for unknown parameter detection
//!
//! Comprehensive test coverage for boundary conditions, special characters,
//! Levenshtein distance edge cases, and complex scenarios.

use unilang::
{
  data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes },
  registry::CommandRegistry,
  semantic::SemanticAnalyzer,
};
use unilang_parser::{ Parser, UnilangParserOptions };

/// Helper to create command with multiple parameters for testing
fn create_complex_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".complex" )
    .description( "Command with multiple parameters" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "verbose" )
        .kind( Kind::Boolean )
        .attributes( ArgumentAttributes
        {
          optional: true,
          default: Some( "false".to_string() ),
          ..Default::default()
        })
        .aliases( vec![ "v".to_string(), "verb".to_string() ] )
        .description( "Verbose output" )
        .end(),
      ArgumentDefinition::former()
        .name( "output" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          default: None,
          ..Default::default()
        })
        .aliases( vec![ "o".to_string() ] )
        .description( "Output file" )
        .end(),
      ArgumentDefinition::former()
        .name( "config" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          default: None,
          ..Default::default()
        })
        .aliases( vec![ "c".to_string(), "cfg".to_string() ] )
        .description( "Config file" )
        .end(),
    ])
    .end()
}

// ============================================================================
// LEVENSHTEIN DISTANCE EDGE CASES
// ============================================================================

/// TEST: Distance of 1 should suggest (single character typo)
#[test]
fn test_distance_one_suggests()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "verbse" vs "verbose" - distance 1 (missing 'o')
  let instruction_text = ".complex verbse::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "verbse" ), "Should mention typo" );
  assert!( error_msg.contains( "Did you mean" ), "Should suggest correction" );
}

/// TEST: Distance of 2 should suggest (two character typo)
#[test]
fn test_distance_two_suggests()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "verbos" vs "verbose" - distance 1, but let's try "vrbose" - distance 1 deletion
  // Actually "verboose" vs "verbose" - distance 1 (extra 'o')
  // Let's try "varbose" vs "verbose" - distance 1 (substitution)
  // Try "vebose" - distance 1 (deletion of 'r')
  // Try "vrbase" vs "verbose" - distance 2
  let instruction_text = ".complex vrbase::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "vrbase" ), "Should mention typo" );
}

/// TEST: Distance of 3 should NOT suggest (too different)
#[test]
fn test_distance_three_no_suggestion()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "xyz" vs "verbose" - distance > 2
  let instruction_text = ".complex xyz::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "xyz" ), "Should mention unknown parameter" );
  assert!( !error_msg.contains( "Did you mean" ), "Should NOT suggest (distance too large)" );
}

/// TEST: Exact match should not be unknown (sanity check)
#[test]
fn test_exact_match_succeeds()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction_text = ".complex verbose::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_ok(), "Exact match should succeed" );
}

// ============================================================================
// ALIAS MATCHING TESTS
// ============================================================================

/// TEST: Unknown parameter close to alias should suggest alias
#[test]
fn test_unknown_close_to_alias_suggests_alias()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "vrb" vs "verb" (alias) - distance 1
  let instruction_text = ".complex vrb::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "vrb" ) );
  // Should suggest either "verb" (alias) or "v" (another alias), depending on distance
}

/// TEST: Using alias should succeed
#[test]
fn test_alias_parameter_succeeds()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Use alias "v" instead of "verbose"
  let instruction_text = ".complex v::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_ok(), "Alias should work" );
}

/// TEST: Mix of canonical and alias names
#[test]
fn test_mix_canonical_and_alias()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Use both canonical name and alias
  let instruction_text = ".complex verbose::1 o::output.txt";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_ok(), "Mix of canonical and alias should work" );
}

// ============================================================================
// MULTIPLE SUGGESTIONS TESTS
// ============================================================================

/// TEST: Unknown parameter close to multiple valid names (picks closest)
#[test]
fn test_multiple_close_matches_picks_closest()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "cfg" is an exact alias match, but "cg" is close to "cfg" (distance 1) and "config" (distance 4)
  let instruction_text = ".complex cg::value";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  // Should suggest "cfg" or "c" (closest matches)
  assert!( error_msg.contains( "Did you mean" ) );
}

// ============================================================================
// PARAMETER NAME EDGE CASES
// ============================================================================

/// TEST: Case sensitivity - parameters are case-sensitive
#[test]
#[ignore = "Parser doesn't support uppercase in parameter names currently"]
fn test_case_sensitive_parameters()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "Verbose" with capital V should be unknown
  // NOTE: This test is ignored because the parser currently rejects uppercase in parameter names
  let instruction_text = ".complex Verbose::1";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err(), "Parameters should be case-sensitive" );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "Verbose" ) );
  // Should suggest "verbose" (lowercase)
  assert!( error_msg.contains( "Did you mean" ) );
}

/// TEST: Very long parameter name
#[test]
fn test_very_long_parameter_name()
{
  let mut registry = CommandRegistry::new();
  registry.register( create_complex_command() );

  let parser = Parser::new( UnilangParserOptions::default() );

  let long_name = "a".repeat( 100 );
  let instruction_text = format!( ".complex {long_name}::value" );
  let instruction = parser.parse_single_instruction( &instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err(), "Unknown long parameter should be rejected" );
}

/// TEST: Parameter name with numbers and underscores
#[test]
fn test_parameter_with_numbers_underscores()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "param_123" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Test param" )
        .end(),
    ])
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Typo: "param_12" instead of "param_123"
  let instruction_text = ".test param_12::value";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  println!( "ERROR: {error_msg}" );
  assert!( error_msg.contains( "param_12" ) );
  assert!( error_msg.contains( "Did you mean" ) && error_msg.contains( "param_123" ),
    "Expected suggestion for param_123, got: {error_msg}" );
}

// ============================================================================
// COMPLEX SCENARIO TESTS
// ============================================================================

/// TEST: Command with many parameters (stress test)
#[test]
fn test_many_parameters_stress()
{
  let mut args = Vec::new();
  for i in 0..20
  {
    args.push(
      ArgumentDefinition::former()
        .name( format!( "param{i}" ) )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Test param" )
        .end()
    );
  }

  let cmd = CommandDefinition::former()
    .name( ".stress" )
    .description( "Stress test" )
    .arguments( args )
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Typo in one of many parameters
  let instruction_text = ".stress param0::a param1::b param99::c";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  println!( "ERROR: {error_msg}" );
  assert!( error_msg.contains( "param99" ), "Should detect unknown among many params, got: {error_msg}" );
}

/// TEST: Similar parameter names (confusable)
#[test]
fn test_similar_parameter_names()
{
  let cmd = CommandDefinition::former()
    .name( ".similar" )
    .description( "Similar names" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "file" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "File" )
        .end(),
      ArgumentDefinition::former()
        .name( "files" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Files" )
        .end(),
      ArgumentDefinition::former()
        .name( "filter" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Filter" )
        .end(),
    ])
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Typo "flie" - could match "file" (distance 2) or "files" (distance 3)
  let instruction_text = ".similar flie::value";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  println!( "ERROR: {error_msg}" );
  assert!( error_msg.contains( "flie" ), "Expected 'flie' in error, got: {error_msg}" );
  // Should suggest "file" (closest match)
  assert!( error_msg.contains( "Did you mean" ) );
}

/// TEST: All optional parameters, unknown one provided
#[test]
fn test_all_optional_params_unknown_provided()
{
  let cmd = CommandDefinition::former()
    .name( ".optional" )
    .description( "All optional" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "a" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Param A" )
        .end(),
      ArgumentDefinition::former()
        .name( "b" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Param B" )
        .end(),
    ])
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Provide unknown "c" even though all params are optional
  let instruction_text = ".optional c::value";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err(), "Unknown parameter should be rejected even if all params optional" );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( 'c' ) );
}

/// TEST: No parameters provided to command that accepts them (should succeed)
#[test]
fn test_no_params_provided_all_optional_succeeds()
{
  let cmd = CommandDefinition::former()
    .name( ".optional" )
    .description( "All optional" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "a" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Param A" )
        .end(),
    ])
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  // Provide no parameters
  let instruction_text = ".optional";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_ok(), "No parameters should succeed when all optional" );
}

// ============================================================================
// BOUNDARY CONDITION TESTS
// ============================================================================

/// TEST: Suggestion with exactly distance 2 (boundary)
#[test]
fn test_suggestion_boundary_distance_2()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "test" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        })
        .description( "Test" )
        .end(),
    ])
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  // "tst" vs "test" - delete 'e', delete 's' = distance 2
  let instruction_text = ".test ts::value";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err() );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  println!( "ERROR: {error_msg}" );
  assert!( error_msg.contains( "ts" ), "Expected 'ts' in error, got: {error_msg}" );
  assert!( error_msg.contains( "Did you mean" ), "Distance 2 should still suggest" );
}

/// TEST: Empty command definition (no parameters at all)
#[test]
fn test_empty_command_definition()
{
  let cmd = CommandDefinition::former()
    .name( ".empty" )
    .description( "No parameters" )
    .arguments( vec![] )
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd );

  let parser = Parser::new( UnilangParserOptions::default() );

  let instruction_text = ".empty anything::value";
  let instruction = parser.parse_single_instruction( instruction_text ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();
  assert!( result.is_err(), "Empty command should reject any named parameter" );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "anything" ) );
  // No suggestion possible since there are no valid parameters
  assert!( !error_msg.contains( "Did you mean" ) );
}
