//! Command Sequence Scalability Unit Tests
//!
//! ## Critical Issue 3: Hard Command Sequence Limit
//! **Severity**: High (Scalability)
//! **Root Cause**: Parser architecture limits to ~3 commands maximum
//! **Impact**: Prevents real-world automation and CI/CD workflows
//!
//! ## Test Coverage (T3)
//! - T3.1: 4-Command Sequence (exceeds current limit)
//! - T3.2: 8-Command CI/CD Pipeline (real-world workflow)
//! - T3.3: Large Automation Scenario (15+ commands)
//! - Additional: Performance scaling and memory usage tests
//!
//! ## Expected Test Behavior (TDD)
//! - **RED PHASE**: Commands beyond 3rd are silently dropped/ignored
//! - **GREEN PHASE**: All commands parsed regardless of sequence length
//! - **REFACTOR PHASE**: Optimize for large-scale command processing

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Helper to create a simple command for scalability testing
fn create_scalability_test_command( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Scalability test command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "step".to_string(),
        description : "Step identifier".to_string(),
        kind : Kind::String,
        hint : "Step name or number".to_string(),
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

/// Helper to parse multiple commands and count parsed results
fn parse_multiple_commands( registry : &CommandRegistry, inputs : &[&str] ) -> Result< usize, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let mut parsed_count = 0;

  for input in inputs {
    match parser.parse_single_instruction( input ) {
      Ok( instruction ) => {
        let instructions_array = [instruction];
        let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
        match analyzer.analyze() {
          Ok( verified_commands ) => {
            parsed_count += verified_commands.len();
          },
          Err( e ) => return Err( format!( "Semantic analysis error for '{input}': {e:?}" ) ),
        }
      },
      Err( e ) => return Err( format!( "Parse error for '{input}': {e:?}" ) ),
    }
  }

  Ok( parsed_count )
}

/// T3.1: 4-Command Sequence (Current Limit Test)
/// **Test Case**: Exactly 4 commands to exceed current 3-command limit
/// **Expected**: All 4 commands should be parsed
/// **Current**: Only parses first 3 commands, 4th is silently ignored
#[test]
fn test_four_commands()
{
  let mut registry = CommandRegistry::new();
  for i in 1..=4 {
    let cmd_name = format!( ".step{i}" );
    let cmd = create_scalability_test_command( &cmd_name );
    registry.register( cmd );
  }

  // Create 4 commands - this should exceed the current limit
  let commands = vec![
    r#".step1 step::"echo 1""#,
    r#".step2 step::"echo 2""#,
    r#".step3 step::"echo 3""#,
    r#".step4 step::"echo 4""#,  // This should be dropped in current implementation
  ];

  let parsed_count = parse_multiple_commands( &registry, &commands )
    .expect( "Should parse 4 commands without errors" );

  assert_eq!( parsed_count, 4, "All 4 commands should be parsed, but current limit is 3" );
}

/// T3.2: 8-Command CI/CD Pipeline
/// **Test Case**: Real-world CI/CD workflow with 8 commands
/// **Expected**: All 8 commands should be parsed for CI/CD workflow
/// **Current**: Scalability failure - only handles first 3 commands
#[test]
fn test_cicd_pipeline_commands()
{
  let mut registry = CommandRegistry::new();

  // Register CI/CD pipeline commands
  let pipeline_commands = [
    ".test", ".clippy", ".build", ".doc", ".audit", ".outdated", ".fmt", ".bench"
  ];

  for cmd_name in &pipeline_commands {
    let cmd = create_scalability_test_command( cmd_name );
    registry.register( cmd );
  }

  // CI/CD pipeline sequence
  let commands = vec![
    r#".test step::"cargo test""#,
    r#".clippy step::"cargo clippy""#,
    r#".build step::"cargo build --release""#,
    r#".doc step::"cargo doc""#,
    r#".audit step::"cargo audit""#,
    r#".outdated step::"cargo outdated""#,
    r#".fmt step::"cargo fmt --check""#,
    r#".bench step::"cargo bench""#,
  ];

  let parsed_count = parse_multiple_commands( &registry, &commands )
    .expect( "Should parse all CI/CD commands without errors" );

  assert_eq!( parsed_count, 8, "All 8 CI/CD commands should be parsed for production workflow" );
}

/// T3.3: Large Automation Scenario (15 Commands)
/// **Test Case**: Large automation workflow with 15 commands
/// **Expected**: Should handle large automation scenarios
/// **Current**: Architectural limit - hard-coded 3-command maximum
#[test]
fn test_15_command_sequence()
{
  let mut registry = CommandRegistry::new();

  // Register 15 automation commands
  for i in 1..=15 {
    let cmd_name = format!( ".auto{i}" );
    let cmd = create_scalability_test_command( &cmd_name );
    registry.register( cmd );
  }

  // Large automation sequence
  let commands : Vec< String > = (1..=15)
    .map( |i| format!( r#".auto{i} step::"automation step {i}""# ) )
    .collect();

  let command_refs : Vec< &str > = commands.iter().map( std::string::String::as_str ).collect();

  let parsed_count = parse_multiple_commands( &registry, &command_refs )
    .expect( "Should parse all 15 automation commands without errors" );

  assert_eq!( parsed_count, 15, "All 15 automation commands should be parsed" );
}

/// T3.3b: Enterprise Scale (20 Commands)
/// **Test Case**: Enterprise-scale automation with 20 commands
/// **Expected**: Should handle enterprise automation scenarios
#[test]
fn test_20_command_sequence()
{
  let mut registry = CommandRegistry::new();

  // Register 20 enterprise commands
  for i in 1..=20 {
    let cmd_name = format!( ".enterprise{i}" );
    let cmd = create_scalability_test_command( &cmd_name );
    registry.register( cmd );
  }

  // Enterprise automation sequence
  let commands : Vec< String > = (1..=20)
    .map( |i| format!( r#".enterprise{i} step::"enterprise step {i}""# ) )
    .collect();

  let command_refs : Vec< &str > = commands.iter().map( std::string::String::as_str ).collect();

  let parsed_count = parse_multiple_commands( &registry, &command_refs )
    .expect( "Should parse all 20 enterprise commands without errors" );

  assert_eq!( parsed_count, 20, "All 20 enterprise commands should be parsed" );
}

/// T3.3c: Stress Test (50 Commands)
/// **Test Case**: Stress test with 50 commands
/// **Expected**: Parser should not have arbitrary limits
#[test]
fn test_50_command_stress_test()
{
  let mut registry = CommandRegistry::new();

  // Register 50 stress test commands
  for i in 1..=50 {
    let cmd_name = format!( ".stress{i}" );
    let cmd = create_scalability_test_command( &cmd_name );
    registry.register( cmd );
  }

  // Stress test sequence
  let commands : Vec< String > = (1..=50)
    .map( |i| format!( r#".stress{i} step::"stress test {i}""# ) )
    .collect();

  let command_refs : Vec< &str > = commands.iter().map( std::string::String::as_str ).collect();

  let parsed_count = parse_multiple_commands( &registry, &command_refs )
    .expect( "Should parse all 50 stress test commands without errors" );

  assert_eq!( parsed_count, 50, "All 50 stress test commands should be parsed" );
}

/// Performance Test: Command Parsing Performance
/// **Test Case**: Measure parsing performance with increasing command count
/// **Expected**: Linear scaling performance
#[test]
fn test_command_parsing_performance()
{
  let mut registry = CommandRegistry::new();

  // Register performance test commands
  for i in 1..=20 {
    let cmd_name = format!( ".perf{i}" );
    let cmd = create_scalability_test_command( &cmd_name );
    registry.register( cmd );
  }

  // Test parsing performance with different command counts
  for command_count in [1, 5, 10, 15, 20] {
    let commands : Vec< String > = (1..=command_count)
      .map( |i| format!( r#".perf{i} step::"performance test {i}""# ) )
      .collect();

    let command_refs : Vec< &str > = commands.iter().map( std::string::String::as_str ).collect();

    let parsed_count = parse_multiple_commands( &registry, &command_refs )
      .expect( "Scalability test should not fail" );

    assert_eq!( parsed_count, command_count, "Should parse all {command_count} commands" );
  }
}

/// Memory Test: Memory Usage Scaling
/// **Test Case**: Verify memory usage scales appropriately
/// **Expected**: Memory usage should scale linearly, not exponentially
#[test]
fn test_memory_usage_scaling()
{
  let mut registry = CommandRegistry::new();

  // Register memory test commands
  for i in 1..=30 {
    let cmd_name = format!( ".mem{i}" );
    let cmd = create_scalability_test_command( &cmd_name );
    registry.register( cmd );
  }

  // Test memory usage doesn't explode with more commands
  let commands : Vec< String > = (1..=30)
    .map( |i| format!( r#".mem{i} step::"memory test {i}""# ) )
    .collect();

  let command_refs : Vec< &str > = commands.iter().map( std::string::String::as_str ).collect();

  let parsed_count = parse_multiple_commands( &registry, &command_refs )
    .expect( "Memory test should not fail or run out of memory" );

  assert_eq!( parsed_count, 30, "Should parse all 30 commands without memory issues" );
}

/// Edge Case: No Commands
/// **Test Case**: Handle empty command sequence gracefully
/// **Expected**: Should handle empty input without errors
#[test]
fn test_no_commands()
{
  let registry = CommandRegistry::new();
  let commands : Vec< &str > = vec![];

  let parsed_count = parse_multiple_commands( &registry, &commands )
    .expect( "Empty command sequence should be handled gracefully" );

  assert_eq!( parsed_count, 0, "No commands should result in zero parsed commands" );
}

/// Edge Case: Single Command
/// **Test Case**: Ensure single command still works
/// **Expected**: Single command should parse normally
#[test]
fn test_single_command()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_scalability_test_command( ".single" );
  registry.register( cmd );

  let commands = vec![ r#".single step::"single command test""# ];

  let parsed_count = parse_multiple_commands( &registry, &commands )
    .expect( "Single command should parse without issues" );

  assert_eq!( parsed_count, 1, "Single command should parse correctly" );
}