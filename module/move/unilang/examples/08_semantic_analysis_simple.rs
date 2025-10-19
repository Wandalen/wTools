#![allow(clippy::all)]
//! # Semantic Analysis Demo (Simplified)
//!
//! **⚠️ NOTE:** This example uses runtime registration for demonstration purposes.
//! For production use, define commands in YAML and use compile-time generation.
//!
//! This example demonstrates the semantic analysis phase, showing how
//! parsed commands are validated against the registry and converted
//! to verified commands ready for execution.


use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Semantic Analysis Demo ===\n" );

  // Step 1: Set up a registry with test commands
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Math command for testing
  let math_command = CommandDefinition::former()
  .name( ".calculate" )
  .namespace( ".math".to_string() )
  .description( "Performs mathematical calculations".to_string() )
  .hint( "Calculator utility" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".calc".to_string() ] )
  .tags( vec![ "math".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "math.calculate --x 10 --y 5 --operation add".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "x".to_string(),
      description: "First number".to_string(),
      kind: Kind::Integer,
      hint: "First operand".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::Min(-1000.0), ValidationRule::Max(1000.0) ],
      aliases: vec![ "first".to_string() ],
      tags: vec![ "numeric".to_string() ],
    },
    ArgumentDefinition {
      name: "y".to_string(),
      description: "Second number".to_string(),
      kind: Kind::Integer,
      hint: "Second operand".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::Min(-1000.0), ValidationRule::Max(1000.0) ],
      aliases: vec![ "second".to_string() ],
      tags: vec![ "numeric".to_string() ],
    },
    ArgumentDefinition {
      name: "operation".to_string(),
      description: "Mathematical operation to perform".to_string(),
      kind: Kind::Enum( vec![ "add".to_string(), "subtract".to_string(), "multiply".to_string(), "divide".to_string() ] ),
      hint: "Operation type".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("add".to_string()),
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![ "op".to_string(), "o".to_string() ],
      tags: vec![ "operation".to_string() ],
    },
  ])
  .end();

  let math_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let x = cmd.arguments.get( "x" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let y = cmd.arguments.get( "y" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let op = cmd.arguments.get( "operation" ).and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } ).unwrap_or( "add" );

    let result = match op
    {
      "add" => x + y,
      "subtract" => x - y,
      "multiply" => x * y,
      "divide" => if *y != 0 { x / y } else { 0 },
      _ => 0,
    };

    println!( "Calculation: {x} {op} {y} = {result}" );

    Ok( OutputData
    {
      content : result.to_string(),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &math_command, math_routine )?;

  // Text processing command for testing
  let text_command = CommandDefinition::former()
  .name( ".process" )
  .namespace( ".text".to_string() )
  .description( "Processes text with various transformations".to_string() )
  .hint( "Text processing utility" )
  .status( "stable" )
  .version( "2.0.0" )
  .aliases( vec![ ".transform".to_string() ] )
  .tags( vec![ "text".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "text.process 'hello world' --operations upper,reverse".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "input".to_string(),
      description: "Text to process".to_string(),
      kind: Kind::String,
      hint: "Input text".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength(1) ],
      aliases: vec![ "text".to_string(), "t".to_string() ],
      tags: vec![ "input".to_string() ],
    },
    ArgumentDefinition {
      name: "operations".to_string(),
      description: "List of operations to apply".to_string(),
      kind: Kind::List( Box::new( Kind::String ), Some( ',' ) ),
      hint: "Comma-separated operations".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("none".to_string()),
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::MinItems(1) ],
      aliases: vec![ "ops".to_string(), "o".to_string() ],
      tags: vec![ "transformation".to_string() ],
    },
  ])
  .end();

  let text_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let input = cmd.arguments.get( "input" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
    .unwrap_or_default();

    let operations = cmd.arguments.get( "operations" )
    .and_then( | v | if let Value::List( list ) = v
    {
      Some( list.iter().filter_map( | item |
        if let Value::String( s ) = item { Some( s.clone() ) } else { None }
      ).collect::< Vec< _ > >() )
    }
    else
    { None })
    .unwrap_or_else( || vec![ "none".to_string() ] );

    let mut result = input.clone();
    for op in &operations
    {
      result = match op.as_str()
      {
        "upper" => result.to_uppercase(),
        "lower" => result.to_lowercase(),
        "reverse" => result.chars().rev().collect(),
        "trim" => result.trim().to_string(),
        _ => result,
      };
    }

    println!( "Text processing: '{input}' -> '{result}'" );
    println!( "Operations applied: {operations:?}" );

    Ok( OutputData
    {
      content : result,
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &text_command, text_routine )?;

  println!( "✓ Registered test commands for semantic analysis" );

  // Step 2: Demonstrate semantic analysis using the parser
  println!( "\n=== Semantic Analysis Test Cases ===" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let test_command_strings = vec!
  [
    // Valid cases
    ( "math.calculate --x 15 --y 3 --operation multiply", "Valid named arguments" ),
    ( "math.calculate 20 4 --op divide", "Positional args with alias" ),
    ( "text.process 'Hello World'", "Default values used" ),
    ( "text.process 'Test String' --operations upper,reverse,trim", "List argument" ),

    // Invalid cases
    ( "nonexistent.command", "Non-existent command" ),
    ( "math.calculate --x 10", "Missing required argument" ),
    ( "math.calculate --x 2000 --y 5", "Validation rule failure" ),
  ];

  for ( i, ( cmd_str, description ) ) in test_command_strings.iter().enumerate()
  {
    println!( "\n--- Test Case {}: {} ---", i + 1, description );
    println!( "Command: '{cmd_str}'" );

    match parser.parse_single_instruction( cmd_str )
    {
      Ok( instruction ) =>
      {
        println!( "✓ Parsing successful" );

        let instructions = [ instruction ];
        let analyzer = SemanticAnalyzer::new( &instructions, &registry );
        match analyzer.analyze()
        {
          Ok( verified_commands ) =>
          {
            println!( "✅ Semantic analysis PASSED" );
            for verified_cmd in &verified_commands
            {
              println!( "  Command: {} v{}", verified_cmd.definition.name, verified_cmd.definition.version );
              println!( "  Namespace: {}", verified_cmd.definition.namespace );
              println!( "  Verified arguments:" );
              for ( name, value ) in &verified_cmd.arguments
              {
                println!( "    {name}: {value:?}" );
              }
            }
          },
          Err( error ) =>
          {
            println!( "❌ Semantic analysis FAILED" );
            println!( "  Error: {error}" );
          }
        }
      },
      Err( error ) =>
      {
        println!( "❌ Parsing FAILED" );
        println!( "  Error: {error}" );
      }
    }
  }

  // Step 3: Demonstrate the complete pipeline with actual parser
  println!( "\n=== Complete Pipeline Demo ===" );

  let test_commands = vec!
  [
    "math.calculate --x 100 --y 25 --operation divide",
    "text.process 'semantic analysis demo' --operations upper,reverse",
    "calc 50 75", // Using alias and positional args
  ];

  for cmd_str in test_commands
  {
    println!( "\n🔍 Analyzing: '{cmd_str}'" );

    match parser.parse_single_instruction( cmd_str )
    {
      Ok( instruction ) =>
      {
        println!( "✓ Parsing successful" );

        let instructions = [ instruction ];
        let analyzer = SemanticAnalyzer::new( &instructions, &registry );
        match analyzer.analyze()
        {
          Ok( verified_commands ) =>
          {
            println!( "✓ Semantic analysis successful" );

            // Execute the verified command
            for verified_cmd in verified_commands
            {
              if let Some( routine ) = registry.get_routine( &format!( ".{}.{}", verified_cmd.definition.namespace.trim_start_matches( '.' ), verified_cmd.definition.name ) )
              {
                let context = unilang::interpreter::ExecutionContext::default();
                match routine( verified_cmd, context )
                {
                  Ok( output ) => println!( "✓ Execution successful: {}", output.content ),
                  Err( e ) => println!( "❌ Execution failed: {e}" ),
                }
              }
            }
          },
          Err( e ) => println!( "❌ Semantic analysis failed: {e}" ),
        }
      },
      Err( e ) => println!( "❌ Parsing failed: {e}" ),
    }
  }

  println!( "\n=== Semantic Analysis Features ===" );
  println!( "🔍 The semantic analyzer performs:" );
  println!( "  • Command existence validation" );
  println!( "  • Argument binding (named → positional → defaults)" );
  println!( "  • Type checking and conversion" );
  println!( "  • Validation rule enforcement" );
  println!( "  • Alias resolution" );
  println!( "  • Required argument verification" );
  println!( "  • Argument count validation" );
  println!( "  • Creation of verified command objects" );

  println!( "\n=== Error Detection Capabilities ===" );
  println!( "❌ Common errors caught by semantic analysis:" );
  println!( "  • COMMAND_NOT_FOUND - Unknown commands" );
  println!( "  • MISSING_ARGUMENT - Required arguments not provided" );
  println!( "  • TOO_MANY_ARGUMENTS - Excess positional arguments" );
  println!( "  • VALIDATION_RULE_FAILED - Constraint violations" );
  println!( "  • TYPE_CONVERSION_ERROR - Invalid data types" );

  println!( "\n=== Best Practices ===" );
  println!( "💡 For robust semantic analysis:" );
  println!( "  • Define clear validation rules" );
  println!( "  • Provide meaningful error messages" );
  println!( "  • Use appropriate default values" );
  println!( "  • Implement comprehensive type checking" );
  println!( "  • Test edge cases and error conditions" );
  println!( "  • Document argument requirements clearly" );

  Ok(())
}