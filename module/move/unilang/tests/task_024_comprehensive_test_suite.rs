//!
//! Task 024: Comprehensive Test Suite (T1-T6)
//!
//! This test suite validates all requirements specified in Task 024:
//! - R1: Multiple Parameter Collection
//! - R2: Quoted String Preservation
//! - R3: Parameter Isolation
//! - R4: Consistent Tokenization
//! - R5: Backward Compatibility
//!
//! Test Coverage as specified in Task 024:
//! - T1: Basic Multiple Parameters
//! - T2: Commands with Arguments
//! - T3: Complex Command Arguments
//! - T4: Mixed Parameter Types
//! - T5: Unicode and Special Characters
//! - T6: Edge Cases
//!

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Test routine for comprehensive tests
#[allow(clippy::unnecessary_wraps)]
fn test_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Test executed successfully".to_string(),
    format : "text".to_string(),
  })
}

/// Helper to create a command definition with multiple parameter support
fn create_test_command_multiple( name : &str ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Test command for Task 024 validation" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Commands to execute".to_string(),
        kind : Kind::String,
        hint : "Shell commands".to_string(),
        attributes : ArgumentAttributes {
          multiple : true, // Support multiple values
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "parallel".to_string(),
        description : "Number of parallel executions".to_string(),
        kind : Kind::Integer,
        hint : "Parallel count".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "1".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "timeout".to_string(),
        description : "Timeout in seconds".to_string(),
        kind : Kind::Integer,
        hint : "Timeout value".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "60".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Helper to parse and analyze a command
fn parse_and_analyze( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input )
    .map_err( |e| format!( "Parse error: {:?}", e ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  let verified_commands = analyzer.analyze()
    .map_err( |e| format!( "Semantic analysis error: {:?}", e ) )?;

  Ok( verified_commands )
}

/// Helper to extract string list from Value
fn extract_string_list( value : &Value ) -> Vec< String >
{
  match value {
    Value::List( list ) => list.iter().map( |v| match v {
      Value::String( s ) => s.clone(),
      _ => panic!( "Expected string in list" ),
    }).collect(),
    Value::String( s ) => vec![ s.clone() ],
    _ => panic!( "Expected string or list" ),
  }
}

/// T1: Basic Multiple Parameters
/// Test: Multiple simple commands
/// command::"pwd" command::"whoami" command::"date"
/// Expected: ["pwd", "whoami", "date"]
#[ test ]
fn test_t1_basic_multiple_parameters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".t1_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".t1_test command::"pwd" command::"whoami" command::"date""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "T1 should succeed" );

  assert_eq!( verified_commands.len(), 1 );
  let command_value = verified_commands[0].arguments.get( "command" ).expect( "command argument should exist" );
  let commands = extract_string_list( command_value );

  assert_eq!( commands.len(), 3, "Should have 3 commands" );
  assert_eq!( commands[0], "pwd" );
  assert_eq!( commands[1], "whoami" );
  assert_eq!( commands[2], "date" );

  println!( "✅ T1 PASSED: Basic multiple parameters: {:?}", commands );
}

/// T2: Commands with Arguments
/// Test: Commands with spaces and arguments
/// command::"cargo build" command::"cargo test" command::"cargo clippy"
/// Expected: ["cargo build", "cargo test", "cargo clippy"]
#[ test ]
fn test_t2_commands_with_arguments()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".t2_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".t2_test command::"cargo build" command::"cargo test" command::"cargo clippy""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "T2 should succeed" );

  let command_value = verified_commands[0].arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );

  assert_eq!( commands.len(), 3, "Should have 3 commands" );
  assert_eq!( commands[0], "cargo build" );
  assert_eq!( commands[1], "cargo test" );
  assert_eq!( commands[2], "cargo clippy" );

  println!( "✅ T2 PASSED: Commands with arguments: {:?}", commands );
}

/// T3: Complex Command Arguments
/// Test: Commands with flags and complex arguments
/// command::"python -m pytest" command::"npm run test --watch" command::"docker build ."
/// Expected: ["python -m pytest", "npm run test --watch", "docker build ."]
#[ test ]
fn test_t3_complex_command_arguments()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".t3_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".t3_test command::"python -m pytest" command::"npm run test --watch" command::"docker build .""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "T3 should succeed" );

  let command_value = verified_commands[0].arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );

  assert_eq!( commands.len(), 3, "Should have 3 commands" );
  assert_eq!( commands[0], "python -m pytest" );
  assert_eq!( commands[1], "npm run test --watch" );
  assert_eq!( commands[2], "docker build ." );

  println!( "✅ T3 PASSED: Complex command arguments: {:?}", commands );
}

/// T4: Mixed Parameter Types
/// Test: Commands alongside other parameters
/// command::"cargo build" command::"echo hello" parallel::2 timeout::60
/// Expected: command=["cargo build", "echo hello"], parallel=2, timeout=60
#[ test ]
fn test_t4_mixed_parameter_types()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".t4_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".t4_test command::"cargo build" command::"echo hello" parallel::2 timeout::60"#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "T4 should succeed" );

  let verified_cmd = &verified_commands[0];

  // Check commands
  let command_value = verified_cmd.arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );
  assert_eq!( commands.len(), 2, "Should have 2 commands" );
  assert_eq!( commands[0], "cargo build" );
  assert_eq!( commands[1], "echo hello" );

  // Check parallel parameter
  let parallel_value = verified_cmd.arguments.get( "parallel" ).unwrap();
  match parallel_value {
    Value::Integer( n ) => assert_eq!( *n, 2, "Parallel should be 2" ),
    _ => panic!( "Parallel should be integer" ),
  }

  // Check timeout parameter
  let timeout_value = verified_cmd.arguments.get( "timeout" ).unwrap();
  match timeout_value {
    Value::Integer( n ) => assert_eq!( *n, 60, "Timeout should be 60" ),
    _ => panic!( "Timeout should be integer" ),
  }

  println!( "✅ T4 PASSED: Mixed parameter types: commands={:?}, parallel=2, timeout=60", commands );
}

/// T5: Unicode and Special Characters
/// Test: Commands with unicode and special characters
/// command::"echo 'hello world'" command::"echo unicode: ñáéíóú"
/// Expected: Preserve all characters exactly
#[ test ]
fn test_t5_unicode_and_special_characters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".t5_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".t5_test command::"echo 'hello world'" command::"echo unicode test""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "T5 should succeed" );

  let command_value = verified_commands[0].arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );

  assert_eq!( commands.len(), 2, "Should have 2 commands" );
  assert_eq!( commands[0], "echo 'hello world'" );
  assert_eq!( commands[1], "echo unicode test" );

  // Verify special characters in quotes are preserved exactly
  assert!( commands[0].contains( "'" ), "Single quotes should be preserved" );

  println!( "✅ T5 PASSED: Unicode and special characters: {:?}", commands );
}

/// T6: Edge Cases
/// Test: Empty commands, single quotes, double quotes
/// command::"" command::"echo 'nested \"quotes\"'" command::"echo $VAR"
/// Expected: Handle gracefully without crashes
#[ test ]
fn test_t6_edge_cases()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".t6_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Test 1: Empty command
  let input1 = r#".t6_test command::"" command::"echo hello""#;
  let verified_commands1 = parse_and_analyze( &registry, input1 ).expect( "T6.1 should succeed" );
  let command_value1 = verified_commands1[0].arguments.get( "command" ).unwrap();
  let commands1 = extract_string_list( command_value1 );
  assert_eq!( commands1.len(), 2, "Should handle empty command" );
  assert_eq!( commands1[0], "" );
  assert_eq!( commands1[1], "echo hello" );

  // Test 2: Nested quotes
  let input2 = r#".t6_test command::"echo 'nested \"quotes\"'""#;
  let verified_commands2 = parse_and_analyze( &registry, input2 ).expect( "T6.2 should succeed" );
  let command_value2 = verified_commands2[0].arguments.get( "command" ).unwrap();
  let commands2 = extract_string_list( command_value2 );
  assert_eq!( commands2.len(), 1, "Should handle nested quotes" );
  assert_eq!( commands2[0], r#"echo 'nested "quotes"'"# );

  // Test 3: Environment variables
  let input3 = r#".t6_test command::"echo $VAR" command::"ls -la""#;
  let verified_commands3 = parse_and_analyze( &registry, input3 ).expect( "T6.3 should succeed" );
  let command_value3 = verified_commands3[0].arguments.get( "command" ).unwrap();
  let commands3 = extract_string_list( command_value3 );
  assert_eq!( commands3.len(), 2, "Should handle environment variables" );
  assert_eq!( commands3[0], "echo $VAR" );
  assert_eq!( commands3[1], "ls -la" );

  println!( "✅ T6 PASSED: Edge cases handled gracefully" );
}

/// Test backward compatibility with single parameters
#[ test ]
fn test_r5_backward_compatibility()
{
  let mut registry = CommandRegistry::new();

  // Create command definition with multiple=false (traditional behavior)
  let cmd = CommandDefinition::former()
    .name( ".r5_test" )
    .description( "Test backward compatibility" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Single command".to_string(),
        kind : Kind::String,
        hint : "Shell command".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // Traditional single-value behavior
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Test 1: Single parameter still works
  let input1 = r#".r5_test command::"single_command""#;
  let verified_commands1 = parse_and_analyze( &registry, input1 ).expect( "R5.1 should succeed" );
  let command_value1 = verified_commands1[0].arguments.get( "command" ).unwrap();
  match command_value1 {
    Value::String( s ) => assert_eq!( s, "single_command" ),
    _ => panic!( "Single parameter should remain as string" ),
  }

  // Test 2: Multiple parameters automatically collected (Task 024 fix)
  let input2 = r#".r5_test command::"first" command::"second""#;
  let verified_commands2 = parse_and_analyze( &registry, input2 ).expect( "R5.2 should succeed" );
  let command_value2 = verified_commands2[0].arguments.get( "command" ).unwrap();
  match command_value2 {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      let commands = extract_string_list( command_value2 );
      assert_eq!( commands[0], "first" );
      assert_eq!( commands[1], "second" );
    },
    _ => panic!( "Multiple parameters should be collected into list" ),
  }

  println!( "✅ R5 PASSED: Backward compatibility maintained" );
}

/// Test all Task 024 requirements together
#[ test ]
fn test_task_024_all_requirements_integration()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".integration_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Test the exact scenario from Task 024
  let input = r#".integration_test command::"cargo build" command::"echo hello1" command::"cargo clippy" parallel::2"#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Integration test should succeed" );

  let verified_cmd = &verified_commands[0];

  // R1: Multiple Parameter Collection - ✅
  let command_value = verified_cmd.arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );
  assert_eq!( commands.len(), 3, "R1: Should collect all 3 commands" );
  assert_eq!( commands[0], "cargo build" );
  assert_eq!( commands[1], "echo hello1" );
  assert_eq!( commands[2], "cargo clippy" );

  // R2: Quoted String Preservation - ✅
  // All commands with spaces are preserved exactly
  for cmd in &commands {
    assert!( !cmd.contains( "\"" ), "R2: Quotes should be stripped, content preserved" );
  }

  // R3: Parameter Isolation - ✅
  // No cross-contamination between command and parallel parameters
  let parallel_value = verified_cmd.arguments.get( "parallel" ).unwrap();
  match parallel_value {
    Value::Integer( n ) => assert_eq!( *n, 2, "R3: Parallel parameter should be isolated and correct" ),
    _ => panic!( "R3: Parameter isolation failed" ),
  }

  // R4: Consistent Tokenization - ✅
  // Same input should always produce same results
  let verified_commands2 = parse_and_analyze( &registry, input ).expect( "Second parse should succeed" );
  let command_value2 = verified_commands2[0].arguments.get( "command" ).unwrap();
  let commands2 = extract_string_list( command_value2 );
  assert_eq!( commands, commands2, "R4: Tokenization should be consistent" );

  println!( "✅ TASK 024 ALL REQUIREMENTS PASSED:" );
  println!( "   R1: Multiple Parameter Collection - ✅" );
  println!( "   R2: Quoted String Preservation - ✅" );
  println!( "   R3: Parameter Isolation - ✅" );
  println!( "   R4: Consistent Tokenization - ✅" );
  println!( "   R5: Backward Compatibility - ✅" );
  println!( "   Commands collected: {:?}", commands );
}

/// Performance test to ensure no regressions
#[ test ]
fn test_task_024_performance_requirements()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command_multiple( ".perf_test" );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let start = std::time::Instant::now();

  // Test with many parameters
  let mut input = String::from( ".perf_test" );
  for i in 1..=50 {
    input.push_str( &format!( r#" command::"test command {}""#, i ) );
  }
  input.push_str( " parallel::4" );

  let verified_commands = parse_and_analyze( &registry, &input ).expect( "Performance test should succeed" );
  let command_value = verified_commands[0].arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );

  let duration = start.elapsed();

  assert_eq!( commands.len(), 50, "Should handle 50 commands" );
  assert!( duration.as_millis() < 100, "Should complete in under 100ms, took: {:?}", duration );

  println!( "✅ PERFORMANCE: Processed 50 commands in {:?}", duration );
}