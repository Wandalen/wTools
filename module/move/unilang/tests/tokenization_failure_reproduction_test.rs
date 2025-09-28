//!
//! Test to reproduce the tokenization failure described in Task 024
//!
//! This test recreates the exact scenario where multiple parameters with the same name
//! are provided but only the first value is processed due to incorrect argument definition.
//!

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Test routine for reproduction test
#[allow(clippy::unnecessary_wraps)]
fn test_run_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Commands executed successfully".to_string(),
    format : "text".to_string(),
  })
}

#[ test ]
fn test_task_024_automatic_multiple_parameter_collection()
{
  let mut registry = CommandRegistry::new();

  // Create a command definition that does NOT have multiple=true
  // With Task 024 fix, multiple values should still be collected automatically
  let cmd = CommandDefinition::former()
    .name( ".run" )
    .description( "Run multiple commands" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Command to execute".to_string(),
        kind : Kind::String,
        hint : "Shell command".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // ❌ THIS IS THE PROBLEM - should be true for collecting multiple values
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
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( test_run_routine ) ).unwrap();

  // Parse the exact command from Task 024
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".run command::"cargo build" command::"echo hello1" command::"cargo clippy" parallel::2"#;

  println!( "Input: {}", input );

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );

  // Debug: Show what the parser actually found
  println!( "Parser results:" );
  println!( "  Command path: {:?}", instruction.command_path_slices );
  println!( "  Named arguments count: {}", instruction.named_arguments.len() );
  for (name, args) in &instruction.named_arguments {
    println!( "  Argument '{}' has {} values:", name, args.len() );
    for (i, arg) in args.iter().enumerate() {
      println!( "    [{}]: {:?}", i, arg.value );
    }
  }

  // Now run semantic analysis
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Semantic analysis should succeed" );

  assert_eq!( verified_commands.len(), 1, "Should have one verified command" );
  let verified_cmd = &verified_commands[0];

  // Debug: Show what the semantic analyzer processed
  println!( "Semantic analyzer results:" );
  for (name, value) in &verified_cmd.arguments {
    println!( "  Argument '{}': {:?}", name, value );
  }

  // VERIFY THE FIX: With Task 024 fix, multiple values should be automatically collected
  // This test now VERIFIES the fix described in Task 024

  let command_value = verified_cmd.arguments.get( "command" ).expect( "command argument should exist" );
  match command_value {
    unilang::types::Value::List( list ) => {
      assert_eq!( list.len(), 3, "All three commands should be collected automatically" );

      // Check each command value
      let commands : Vec< String > = list.iter().map( |v| match v {
        unilang::types::Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in command list" ),
      }).collect();

      assert_eq!( commands[0], "cargo build" );
      assert_eq!( commands[1], "echo hello1" );
      assert_eq!( commands[2], "cargo clippy" );

      println!( "✅ TASK 024 FIX VERIFIED: All commands automatically collected: {:?}", commands );
    },
    unilang::types::Value::String( s ) => {
      panic!( "❌ UNEXPECTED: Got single string '{}', but Task 024 fix should auto-collect multiple values into list", s );
    },
    _ => panic!( "❌ UNEXPECTED: Command value is not String or List" ),
  }

  let parallel_value = verified_cmd.arguments.get( "parallel" ).expect( "parallel argument should exist" );
  match parallel_value {
    unilang::types::Value::Integer( n ) => {
      assert_eq!( *n, 2, "Parallel argument should be parsed correctly" );
      println!( "✅ Parallel value processed correctly: {}", n );
    },
    _ => panic!( "❌ UNEXPECTED: Parallel value is not Integer" ),
  }

  println!( "✅ Task 024 fix successfully implemented: multiple values automatically collected regardless of multiple attribute" );
}

#[ test ]
fn test_task_024_fix_with_multiple_true()
{
  let mut registry = CommandRegistry::new();

  // Create a command definition that DOES have multiple=true
  // This shows the correct behavior we want to achieve
  let cmd = CommandDefinition::former()
    .name( ".run_fixed" )
    .description( "Run multiple commands (fixed)" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Commands to execute".to_string(),
        kind : Kind::String,
        hint : "Shell commands".to_string(),
        attributes : ArgumentAttributes {
          multiple : true, // ✅ THIS FIXES THE PROBLEM
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
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( test_run_routine ) ).unwrap();

  // Parse the same command
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = r#".run_fixed command::"cargo build" command::"echo hello1" command::"cargo clippy" parallel::2"#;

  println!( "Input (fixed): {}", input );

  let instruction = parser.parse_single_instruction( input ).expect( "Parse should succeed" );

  // Run semantic analysis
  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, &registry );
  let verified_commands = analyzer.analyze().expect( "Semantic analysis should succeed" );

  assert_eq!( verified_commands.len(), 1, "Should have one verified command" );
  let verified_cmd = &verified_commands[0];

  // VERIFY THE FIX: With multiple=true, all values should be collected
  let command_value = verified_cmd.arguments.get( "command" ).expect( "command argument should exist" );
  match command_value {
    unilang::types::Value::List( list ) => {
      assert_eq!( list.len(), 3, "All three commands should be collected" );

      // Check each command value
      let commands : Vec< String > = list.iter().map( |v| match v {
        unilang::types::Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in command list" ),
      }).collect();

      assert_eq!( commands[0], "cargo build" );
      assert_eq!( commands[1], "echo hello1" );
      assert_eq!( commands[2], "cargo clippy" );

      println!( "✅ FIX VERIFIED: All commands collected: {:?}", commands );
    },
    unilang::types::Value::String( s ) => {
      panic!( "❌ UNEXPECTED: Got single string '{}', but multiple=true should collect into list", s );
    },
    _ => panic!( "❌ UNEXPECTED: Command value is not String or List" ),
  }

  println!( "✅ Task 024 fix verified: multiple=true correctly collects all values" );
}