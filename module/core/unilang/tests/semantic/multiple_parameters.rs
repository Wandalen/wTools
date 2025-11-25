//! Multiple Parameter Collection Tests
//!
//! ## Scope
//! Tests the semantic analyzer's ability to collect multiple parameters with the same name
//! into lists, regardless of the `multiple` attribute setting. This implements the core
//! functionality from the critical Task 024 fix.
//!
//! ## Coverage
//! - Basic multiple parameter collection
//! - Mixed parameter types with multiple values
//! - Quoted string preservation in multiple parameters
//! - Alias handling with multiple parameters
//! - Performance characteristics of multiple parameter processing
//! - Backward compatibility with single parameters
//! - Error conditions and edge cases
//!
//! ## Related
//! - `unit/parser/argument_parsing.rs` - Parameter parsing functionality
//! - `unit/semantic/argument_binding.rs` - General argument binding logic
//! - `regression/parameter_collection.rs` - Regression prevention for Task 024

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Simple test routine for multiple parameter tests
/// Returns minimal successful output - actual execution not tested here
#[allow(clippy::unnecessary_wraps)]
fn test_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Test executed successfully".to_string(),
    format : "text".to_string(),
      execution_time_ms : None,
  })
}

/// Helper to create a command definition supporting multiple parameters
fn create_multiple_command( name : &str, param_name : &str, multiple_attr : bool ) -> CommandDefinition
{
  // Use Kind::List when multiple:true to prevent wplan bug pattern
  let kind = if multiple_attr
  {
    Kind::List( Box::new( Kind::String ), None )
  }
  else
  {
    Kind::String
  };

  CommandDefinition::former()
    .name( name )
    .description( "Test command for multiple parameter validation" )
    .arguments( vec![
      ArgumentDefinition {
        name : param_name.to_string(),
        description : "Parameter that can appear multiple times".to_string(),
        kind,
        hint : "Test parameter".to_string(),
        attributes : ArgumentAttributes {
          multiple : multiple_attr,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "alias1".to_string(), "a".to_string() ],
        tags : vec![],
      }
    ])
    .end()
}

/// Helper to create command with mixed parameter types
fn create_mixed_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".mixed" )
    .description( "Test command with mixed parameter types" )
    .arguments( vec![
      ArgumentDefinition {
        name : "files".to_string(),
        description : "File paths".to_string(),
        kind : Kind::String,
        hint : "File paths".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // Test auto-collection even with multiple=false
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "counts".to_string(),
        description : "Count values".to_string(),
        kind : Kind::Integer,
        hint : "Numeric counts".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // Test auto-collection even with multiple=false
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "enabled".to_string(),
        description : "Boolean flags".to_string(),
        kind : Kind::Boolean,
        hint : "Boolean values".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // Test auto-collection even with multiple=false
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

/// Helper to parse and analyze a command
fn parse_and_analyze( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input )
    .map_err( |e| format!( "Parse error: {e:?}" ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  analyzer.analyze().map_err( |e| format!( "Semantic analysis error: {e:?}" ) )
}

#[test]
fn test_basic_multiple_parameter_collection()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "param", false ); // multiple=false
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test param::"value1" param::"value2" param::"value3""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  assert_eq!( verified_commands.len(), 1 );
  let verified_cmd = &verified_commands[0];

  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should exist" );
  match param_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3 );
      let values : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in list" ),
      }).collect();
      assert_eq!( values, vec![ "value1", "value2", "value3" ] );
    },
    _ => panic!( "Expected List for multiple parameters, got: {param_value:?}" ),
  }
}

#[test]
fn test_multiple_parameter_collection_with_multiple_true()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "param", true ); // multiple=true
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test param::"value1" param::"value2""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should exist" );

  match param_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
    },
    _ => panic!( "Expected List for multiple=true parameters" ),
  }
}

#[test]
fn test_single_parameter_with_multiple_false()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "param", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test param::"single_value""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should exist" );

  // Single parameter with multiple=false should remain as String (backward compatibility)
  match param_value {
    Value::String( s ) => {
      assert_eq!( s, "single_value" );
    },
    _ => panic!( "Expected String for single parameter with multiple=false" ),
  }
}

#[test]
fn test_single_parameter_with_multiple_true()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "param", true );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test param::"single_value""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should exist" );

  // Single parameter with multiple=true should be wrapped in List for consistency
  match param_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 1 );
      match &list[0] {
        Value::String( s ) => assert_eq!( s, "single_value" ),
        Value::List( inner_list ) => {
          // With Kind::List, we get nested lists: outer from multiple:true, inner from Kind::List
          assert_eq!( inner_list.len(), 1 );
          match &inner_list[0] {
            Value::String( s ) => assert_eq!( s, "single_value" ),
            other => panic!( "Expected String in nested list, got: {other:?}" ),
          }
        },
        _ => panic!( "Expected String in list" ),
      }
    },
    _ => panic!( "Expected List for multiple=true parameter" ),
  }
}

#[test]
fn test_multiple_parameters_with_aliases()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "param", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Mix canonical name and aliases
  let input = r#".test param::"value1" alias1::"value2" a::"value3""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should exist" );

  match param_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3 );
      let values : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in list" ),
      }).collect();
      assert_eq!( values, vec![ "value1", "value2", "value3" ] );
    },
    _ => panic!( "Expected List when collecting parameters across aliases" ),
  }
}

#[test]
fn test_mixed_parameter_types()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_mixed_command();
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".mixed files::"file1.txt" files::"file2.txt" counts::10 counts::20 enabled::true enabled::false"#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];

  // Check files (strings)
  let files_value = verified_cmd.arguments.get( "files" ).expect( "files should exist" );
  match files_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      let files : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in files list" ),
      }).collect();
      assert_eq!( files, vec![ "file1.txt", "file2.txt" ] );
    },
    _ => panic!( "Expected List for files" ),
  }

  // Check counts (integers)
  let counts_value = verified_cmd.arguments.get( "counts" ).expect( "counts should exist" );
  match counts_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      let counts : Vec< i64 > = list.iter().map( |v| match v {
        Value::Integer( i ) => *i,
        _ => panic!( "Expected integer in counts list" ),
      }).collect();
      assert_eq!( counts, vec![ 10, 20 ] );
    },
    _ => panic!( "Expected List for counts" ),
  }

  // Check enabled (booleans)
  let enabled_value = verified_cmd.arguments.get( "enabled" ).expect( "enabled should exist" );
  match enabled_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      let enabled : Vec< bool > = list.iter().map( |v| match v {
        Value::Boolean( b ) => *b,
        _ => panic!( "Expected boolean in enabled list" ),
      }).collect();
      assert_eq!( enabled, vec![ true, false ] );
    },
    _ => panic!( "Expected List for enabled" ),
  }
}

#[test]
fn test_quoted_string_preservation()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "commands", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test commands::"cargo build" commands::"echo hello world" commands::"ls -la /path/with spaces/""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let commands_value = verified_cmd.arguments.get( "commands" ).expect( "commands should exist" );

  match commands_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3 );
      let commands : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in commands list" ),
      }).collect();

      assert_eq!( commands[0], "cargo build" );
      assert_eq!( commands[1], "echo hello world" );
      assert_eq!( commands[2], "ls -la /path/with spaces/" );
    },
    _ => panic!( "Expected List for quoted strings" ),
  }
}

#[test]
fn test_unicode_and_special_characters()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "text", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test text::"hello world" text::"special: chars & symbols""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let text_value = verified_cmd.arguments.get( "text" ).expect( "text should exist" );

  match text_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      let texts : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in text list" ),
      }).collect();

      assert_eq!( texts[0], "hello world" );
      assert_eq!( texts[1], "special: chars & symbols" );
    },
    _ => panic!( "Expected List for special characters" ),
  }
}

#[test]
fn test_performance_with_many_parameters()
{
  use std::time::Instant;

  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "data", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  // Create input with 50 parameters
  let mut input_parts = vec![ ".test".to_string() ];
  for i in 1..=50 {
    input_parts.push( format!( r#"data::"value{i}""# ) );
  }
  let input = input_parts.join( " " );

  let start = Instant::now();
  let verified_commands = parse_and_analyze( &registry, &input ).expect( "Should succeed" );
  let duration = start.elapsed();

  // Performance check: should complete quickly
  assert!( duration.as_millis() < 100, "Processing 50 parameters took too long: {duration:?}" );

  let verified_cmd = &verified_commands[0];
  let data_value = verified_cmd.arguments.get( "data" ).expect( "data should exist" );

  match data_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 50 );
    },
    _ => panic!( "Expected List for performance test" ),
  }
}

#[test]
fn test_edge_case_empty_values()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_multiple_command( ".test", "values", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".test values::"" values::"non-empty" values::"""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let values_value = verified_cmd.arguments.get( "values" ).expect( "values should exist" );

  match values_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 3 );
      let values : Vec< String > = list.iter().map( |v| match v {
        Value::String( s ) => s.clone(),
        _ => panic!( "Expected string in values list" ),
      }).collect();

      assert_eq!( values[0], "" );
      assert_eq!( values[1], "non-empty" );
      assert_eq!( values[2], "" );
    },
    _ => panic!( "Expected List for empty values test" ),
  }
}

#[test]
fn test_backward_compatibility_single_parameters()
{
  let mut registry = CommandRegistry::new();

  // Command with multiple=false
  let cmd = create_multiple_command( ".single", "param", false );
  registry.command_add_runtime( &cmd, Box::new( test_routine ) ).unwrap();

  let input = r#".single param::"single_value""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Should succeed" );

  let verified_cmd = &verified_commands[0];
  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should exist" );

  // Single parameter should remain as String for backward compatibility
  match param_value {
    Value::String( s ) => {
      assert_eq!( s, "single_value" );
    },
    _ => panic!( "Expected String for single parameter (backward compatibility)" ),
  }
}