//! Argument Binding Unit Tests
//!
//! ## Scope
//! Tests the semantic analyzer's ability to bind parsed arguments to command definitions.
//! This covers the critical logic that maps parser output to typed command arguments
//! with validation and type conversion.
//!
//! ## Coverage
//! - Basic argument binding (named, positional)
//! - Type conversion and validation
//! - Optional and required argument handling
//! - Default value assignment
//! - Validation rule enforcement
//! - Alias resolution and binding
//! - Error conditions and edge cases
//!
//! ## Related
//! - `unit/semantic/multiple_parameters.rs` - Multiple parameter collection
//! - `unit/parser/argument_parsing.rs` - Parser argument extraction
//! - `unit/data/types.rs` - Value types and conversions

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Mock routine for argument binding tests
#[allow(clippy::unnecessary_wraps)]
fn mock_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Test executed successfully".to_string(),
    format : "text".to_string(),
  })
}

/// Helper to create command with specific argument configuration
fn create_binding_test_command( name : &str, arguments : Vec< ArgumentDefinition > ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Test command for argument binding validation" )
    .arguments( arguments )
    .end()
}

/// Helper to parse and analyze a command
fn parse_and_bind( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input )
    .map_err( |e| format!( "Parse error: {:?}", e ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  analyzer.analyze().map_err( |e| format!( "Binding error: {:?}", e ) )
}

#[test]
fn test_basic_named_argument_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "param".to_string(),
      description : "Test parameter".to_string(),
      kind : Kind::String,
      hint : "String parameter".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test param::"value""# ).expect( "Binding should succeed" );

  assert_eq!( verified_commands.len(), 1 );
  let verified_cmd = &verified_commands[0];

  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should be bound" );
  match param_value {
    Value::String( s ) => assert_eq!( s, "value" ),
    _ => panic!( "Expected String value" ),
  }
}

#[test]
fn test_positional_argument_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "first".to_string(),
      description : "First parameter".to_string(),
      kind : Kind::String,
      hint : "First string".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
    ArgumentDefinition {
      name : "second".to_string(),
      description : "Second parameter".to_string(),
      kind : Kind::Integer,
      hint : "Second integer".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test "hello" 42"# ).expect( "Positional binding should succeed" );

  let verified_cmd = &verified_commands[0];

  let first_value = verified_cmd.arguments.get( "first" ).expect( "first should be bound" );
  match first_value {
    Value::String( s ) => assert_eq!( s, "hello" ),
    _ => panic!( "Expected String value for first" ),
  }

  let second_value = verified_cmd.arguments.get( "second" ).expect( "second should be bound" );
  match second_value {
    Value::Integer( i ) => assert_eq!( *i, 42 ),
    _ => panic!( "Expected Integer value for second" ),
  }
}

#[test]
fn test_mixed_named_and_positional_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "pos1".to_string(),
      description : "Positional 1".to_string(),
      kind : Kind::String,
      hint : "Position 1".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
    ArgumentDefinition {
      name : "named".to_string(),
      description : "Named parameter".to_string(),
      kind : Kind::String,
      hint : "Named value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
    ArgumentDefinition {
      name : "pos2".to_string(),
      description : "Positional 2".to_string(),
      kind : Kind::String,
      hint : "Position 2".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test "first" named::"middle" "last""# ).expect( "Mixed binding should succeed" );

  let verified_cmd = &verified_commands[0];

  let pos1_value = verified_cmd.arguments.get( "pos1" ).unwrap();
  assert_eq!( pos1_value, &Value::String( "first".to_string() ) );

  let named_value = verified_cmd.arguments.get( "named" ).unwrap();
  assert_eq!( named_value, &Value::String( "middle".to_string() ) );

  let pos2_value = verified_cmd.arguments.get( "pos2" ).unwrap();
  assert_eq!( pos2_value, &Value::String( "last".to_string() ) );
}

#[test]
fn test_type_conversion_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "string_val".to_string(),
      description : "String value".to_string(),
      kind : Kind::String,
      hint : "String".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "int_val".to_string(),
      description : "Integer value".to_string(),
      kind : Kind::Integer,
      hint : "Integer".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "bool_val".to_string(),
      description : "Boolean value".to_string(),
      kind : Kind::Boolean,
      hint : "Boolean".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "float_val".to_string(),
      description : "Float value".to_string(),
      kind : Kind::Float,
      hint : "Float".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test string_val::"hello" int_val::42 bool_val::true float_val::3.14"# )
    .expect( "Type conversion binding should succeed" );

  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "string_val" ).unwrap(), &Value::String( "hello".to_string() ) );
  assert_eq!( verified_cmd.arguments.get( "int_val" ).unwrap(), &Value::Integer( 42 ) );
  assert_eq!( verified_cmd.arguments.get( "bool_val" ).unwrap(), &Value::Boolean( true ) );
  assert_eq!( verified_cmd.arguments.get( "float_val" ).unwrap(), &Value::Float( 3.14 ) );
}

#[test]
fn test_optional_argument_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "required".to_string(),
      description : "Required parameter".to_string(),
      kind : Kind::String,
      hint : "Required value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "optional".to_string(),
      description : "Optional parameter".to_string(),
      kind : Kind::String,
      hint : "Optional value".to_string(),
      attributes : ArgumentAttributes {
        optional : true,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test with only required argument
  let verified_commands = parse_and_bind( &registry, r#".test required::"value""# ).expect( "Should bind with only required" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "required" ).unwrap(), &Value::String( "value".to_string() ) );
  assert!( verified_cmd.arguments.get( "optional" ).is_none(), "Optional argument should not be present" );

  // Test with both arguments
  let verified_commands = parse_and_bind( &registry, r#".test required::"req" optional::"opt""# ).expect( "Should bind both arguments" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "required" ).unwrap(), &Value::String( "req".to_string() ) );
  assert_eq!( verified_cmd.arguments.get( "optional" ).unwrap(), &Value::String( "opt".to_string() ) );
}

#[test]
fn test_default_value_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "param".to_string(),
      description : "Parameter with default".to_string(),
      kind : Kind::String,
      hint : "String with default".to_string(),
      attributes : ArgumentAttributes {
        optional : true,
        default : Some( "default_value".to_string() ),
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test without providing the parameter (should use default)
  let verified_commands = parse_and_bind( &registry, r#".test"# ).expect( "Should bind with default value" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "param" ).unwrap(), &Value::String( "default_value".to_string() ) );

  // Test with providing the parameter (should override default)
  let verified_commands = parse_and_bind( &registry, r#".test param::"custom""# ).expect( "Should bind with custom value" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "param" ).unwrap(), &Value::String( "custom".to_string() ) );
}

#[test]
fn test_alias_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "parameter".to_string(),
      description : "Parameter with aliases".to_string(),
      kind : Kind::String,
      hint : "String parameter".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![ "param".to_string(), "p".to_string() ],
      tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test binding with canonical name
  let verified_commands = parse_and_bind( &registry, r#".test parameter::"canonical""# ).expect( "Should bind with canonical name" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "parameter" ).unwrap(), &Value::String( "canonical".to_string() ) );

  // Test binding with first alias
  let verified_commands = parse_and_bind( &registry, r#".test param::"alias1""# ).expect( "Should bind with first alias" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "parameter" ).unwrap(), &Value::String( "alias1".to_string() ) );

  // Test binding with second alias
  let verified_commands = parse_and_bind( &registry, r#".test p::"alias2""# ).expect( "Should bind with second alias" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "parameter" ).unwrap(), &Value::String( "alias2".to_string() ) );
}

#[test]
fn test_validation_rule_enforcement()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "min_length".to_string(),
      description : "Parameter with minimum length".to_string(),
      kind : Kind::String,
      hint : "String with min length".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![ ValidationRule::MinLength( 5 ) ],
      aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "range_value".to_string(),
      description : "Parameter with range validation".to_string(),
      kind : Kind::Integer,
      hint : "Integer in range".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![ ValidationRule::Min( 1.0 ), ValidationRule::Max( 100.0 ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test valid values (should succeed)
  let result = parse_and_bind( &registry, r#".test min_length::"valid_string" range_value::50"# );
  assert!( result.is_ok(), "Valid values should pass validation" );

  // Test invalid min length (should fail)
  let result = parse_and_bind( &registry, r#".test min_length::"bad" range_value::50"# );
  assert!( result.is_err(), "Too short string should fail validation" );

  // Test invalid range (should fail)
  let result = parse_and_bind( &registry, r#".test min_length::"valid_string" range_value::150"# );
  assert!( result.is_err(), "Out of range value should fail validation" );
}

#[test]
fn test_missing_required_argument_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "required".to_string(),
      description : "Required parameter".to_string(),
      kind : Kind::String,
      hint : "Required value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test without required argument (should fail)
  let result = parse_and_bind( &registry, r#".test"# );
  assert!( result.is_err(), "Missing required argument should fail" );

  let error_message = result.unwrap_err();
  assert!( error_message.contains( "required" ) || error_message.contains( "missing" ), "Error should mention missing required argument" );
}

#[test]
fn test_type_conversion_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "number".to_string(),
      description : "Integer parameter".to_string(),
      kind : Kind::Integer,
      hint : "Integer value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test with invalid integer value (should fail)
  let result = parse_and_bind( &registry, r#".test number::"not_a_number""# );
  assert!( result.is_err(), "Invalid type conversion should fail" );

  let error_message = result.unwrap_err();
  assert!( error_message.contains( "number" ) || error_message.contains( "integer" ) || error_message.contains( "type" ),
           "Error should mention type conversion issue" );
}

#[test]
fn test_excess_arguments_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "only_arg".to_string(),
      description : "Only argument".to_string(),
      kind : Kind::String,
      hint : "Single string".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test with too many positional arguments (should fail)
  let result = parse_and_bind( &registry, r#".test "arg1" "arg2""# );
  assert!( result.is_err(), "Excess arguments should fail" );

  let error_message = result.unwrap_err();
  assert!( error_message.contains( "too many" ) || error_message.contains( "excess" ), "Error should mention excess arguments" );
}

#[test]
fn test_binding_performance()
{
  use std::time::Instant;

  let mut registry = CommandRegistry::new();

  // Create command with many arguments
  let mut arguments = Vec::new();
  for i in 0..50 {
    arguments.push( ArgumentDefinition {
      name : format!( "arg{}", i ),
      description : format!( "Argument {}", i ),
      kind : Kind::String,
      hint : "String argument".to_string(),
      attributes : ArgumentAttributes {
        optional : true,
        default : Some( format!( "default{}", i ) ),
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    });
  }

  let cmd = create_binding_test_command( ".perf", arguments );
  registry.command_add_runtime( &cmd, Box::new( mock_routine ) ).unwrap();

  // Test with all default values (many arguments to bind)
  let start = Instant::now();
  let result = parse_and_bind( &registry, r#".perf"# );
  let duration = start.elapsed();

  assert!( result.is_ok(), "Performance test should succeed" );
  assert!( duration.as_millis() < 50, "Binding should be fast: {:?}", duration );

  let verified_cmd = &result.unwrap()[0];
  assert_eq!( verified_cmd.arguments.len(), 50, "All default arguments should be bound" );
}