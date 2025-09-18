#![allow(clippy::all)]
//! # Command Execution Demo
//!
//! This example demonstrates the command execution phase, showing how
//! verified commands are interpreted and executed with proper context
//! and error handling.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorData, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Command Execution Demo ===\n" );

  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Step 1: Create commands with different execution patterns

  // 1. Simple successful command
  let hello_command = CommandDefinition::former()
  .name( "hello" )
  .namespace( String::new() )
  .description( "Prints a greeting message".to_string() )
  .hint( "Simple greeting" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "hi".to_string() ] )
  .tags( vec![ "greeting".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "hello Alice".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "name".to_string(),
      description: "Name to greet".to_string(),
      kind: Kind::String,
      hint: "Person's name".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("World".to_string()),
        ..Default::default()
      },
      validation_rules: vec![ ValidationRule::MinLength(1) ],
      aliases: vec![ "n".to_string() ],
      tags: vec![ "personal".to_string() ],
    }
  ])
  .end();

  let hello_routine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    let default_name = "World".to_string();
    let name = cmd.arguments.get( "name" )
    .and_then( | v | if let Value::String( s ) = v { Some( s ) } else { None } )
    .unwrap_or( &default_name );

    let greeting = format!( "Hello, {name}! 👋" );
    println!( "{greeting}" );

    Ok( OutputData
    {
      content : greeting,
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &hello_command, hello_routine )?;

  // 2. Command that uses execution context
  let status_command = CommandDefinition::former()
  .name( "status" )
  .namespace( ".system".to_string() )
  .description( "Shows system status information".to_string() )
  .hint( "System diagnostics" )
  .status( "stable" )
  .version( "2.1.0" )
  .aliases( vec![ "info".to_string(), "diag".to_string() ] )
  .tags( vec![ "system".to_string(), "monitoring".to_string() ] )
  .permissions( vec![ "read_system".to_string() ] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "system.status --verbose".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "verbose".to_string(),
      description: "Show detailed information".to_string(),
      kind: Kind::Boolean,
      hint: "Enable verbose output".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("false".to_string()),
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![ "v".to_string() ],
      tags: vec![ "output".to_string() ],
    }
  ])
  .end();

  let status_routine = Box::new( | cmd : VerifiedCommand, ctx : ExecutionContext |
  {
    let verbose = cmd.arguments.get( "verbose" )
    .and_then( | v | if let Value::Boolean( b ) = v { Some( b ) } else { None } )
    .unwrap_or( &false );

    println!( "🖥️  System Status Report" );
    println!( "========================" );
    println!( "Status: Online ✅" );
    println!( "Uptime: 5 days, 3 hours" );

    if *verbose
    {
      println!( "\nDetailed Information:" );
      println!( "  • Memory Usage: 4.2GB / 16GB" );
      println!( "  • CPU Usage: 23%" );
      println!( "  • Disk Space: 256GB / 1TB" );
      println!( "  • Network: Connected" );
      println!( "  • Services: 12 running, 0 stopped" );
    }

    // Demonstrate context usage (in real applications, context would contain useful data)
    println!( "\nExecution Context: {ctx:?}" );

    let content = if *verbose
    {
      "Detailed system status: All systems operational"
    }
    else
    {
      "System status: Online"
    };

    Ok( OutputData
    {
      content : content.to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &status_command, status_routine )?;

  // 3. Command that can fail with error
  let divide_command = CommandDefinition::former()
  .name( "divide" )
  .namespace( ".math".to_string() )
  .description( "Divides two numbers with error handling".to_string() )
  .hint( "Safe division operation" )
  .status( "stable" )
  .version( "1.2.0" )
  .aliases( vec![ "div".to_string() ] )
  .tags( vec![ "math".to_string(), "arithmetic".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "math.divide 10 2".to_string(), "math.divide 15 0".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "dividend".to_string(),
      description: "Number to be divided".to_string(),
      kind: Kind::Float,
      hint: "Dividend (numerator)".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "a".to_string(), "numerator".to_string() ],
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition {
      name: "divisor".to_string(),
      description: "Number to divide by".to_string(),
      kind: Kind::Float,
      hint: "Divisor (denominator)".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "b".to_string(), "denominator".to_string() ],
      tags: vec![ "required".to_string() ],
    },
  ])
  .end();

  let divide_routine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    let dividend = cmd.arguments.get( "dividend" )
    .and_then( | v | if let Value::Float( f ) = v { Some( f ) } else { None } )
    .unwrap_or( &0.0 );

    let divisor = cmd.arguments.get( "divisor" )
    .and_then( | v | if let Value::Float( f ) = v { Some( f ) } else { None } )
    .unwrap_or( &0.0 );

    if *divisor == 0.0
    {
      return Err( ErrorData::new(
        "DIVISION_BY_ZERO".to_string(),
        format!( "Cannot divide {dividend} by zero. Division by zero is undefined." ),
      ));
    }

    if divisor.abs() < f64::EPSILON && dividend.abs() > f64::EPSILON
    {
      return Err( ErrorData::new(
        "DIVISION_BY_NEAR_ZERO".to_string(),
        "Division by very small number may result in numerical instability".to_string(),
      ));
    }

    let result = dividend / divisor;

    if result.is_infinite()
    {
      return Err( ErrorData::new(
        "RESULT_OVERFLOW".to_string(),
        "Division result is infinite (overflow)".to_string(),
      ));
    }

    if result.is_nan()
    {
      return Err( ErrorData::new(
        "INVALID_RESULT".to_string(),
        "Division result is not a number (NaN)".to_string(),
      ));
    }

    let output = format!( "{dividend} ÷ {divisor} = {result}" );
    println!( "🧮 {output}" );

    Ok( OutputData
    {
      content : result.to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &divide_command, divide_routine )?;

  // 4. Command with complex data processing
  let analyze_command = CommandDefinition::former()
  .name( "analyze" )
  .namespace( ".data".to_string() )
  .description( "Analyzes a list of numbers with statistics".to_string() )
  .hint( "Statistical analysis" )
  .status( "beta" )
  .version( "0.9.0" )
  .aliases( vec![ "stats".to_string() ] )
  .tags( vec![ "data".to_string(), "statistics".to_string(), "analysis".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "data.analyze --numbers 1,5,3,9,2,7,4".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "numbers".to_string(),
      description: "List of numbers to analyze".to_string(),
      kind: Kind::List( Box::new( Kind::Float ), Some( ',' ) ),
      hint: "Comma-separated numbers".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinItems(2) ],
      aliases: vec![ "data".to_string(), "values".to_string() ],
      tags: vec![ "required".to_string(), "numeric".to_string() ],
    },
  ])
  .end();

  let analyze_routine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    let numbers = cmd.arguments.get( "numbers" )
    .and_then( | v | if let Value::List( list ) = v
    {
      Some( list.iter().filter_map( | item |
        if let Value::Float( f ) = item { Some( f ) } else { None }
      ).collect::< Vec< _ > >() )
    }
    else
    { None })
    .unwrap_or_default();

    if numbers.is_empty()
    {
      return Err( ErrorData::new(
        "NO_DATA".to_string(),
        "No valid numbers provided for analysis".to_string(),
      ));
    }

    // Calculate statistics
    let count = numbers.len();
    let sum : f64 = numbers.iter().map( | x | **x ).sum();
    let mean = sum / count as f64;

    let mut sorted = numbers.clone();
    sorted.sort_by( | a, b | a.partial_cmp( b ).unwrap_or( core::cmp::Ordering::Equal ) );

    let median = if count % 2 == 0
    {
      f64::midpoint(*sorted[ count / 2 - 1 ], *sorted[ count / 2 ])
    }
    else
    {
      *sorted[ count / 2 ]
    };

    let min = *sorted[ 0 ];
    let max = *sorted[ count - 1 ];
    let range = max - min;

    // Calculate standard deviation
    let variance : f64 = numbers.iter()
    .map( | x | ( **x - mean ).powi( 2 ) )
    .sum::< f64 >() / count as f64;
    let std_dev = variance.sqrt();

    println!( "📊 Statistical Analysis Results" );
    println!( "================================" );
    println!( "Dataset: {numbers:?}" );
    println!( "Count: {count}" );
    println!( "Sum: {sum:.2}" );
    println!( "Mean: {mean:.2}" );
    println!( "Median: {median:.2}" );
    println!( "Min: {min:.2}" );
    println!( "Max: {max:.2}" );
    println!( "Range: {range:.2}" );
    println!( "Std Dev: {std_dev:.2}" );

    let result = format!
    (
      "count={count}, mean={mean:.2}, median={median:.2}, min={min:.2}, max={max:.2}, std_dev={std_dev:.2}"
    );

    Ok( OutputData
    {
      content : result,
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &analyze_command, analyze_routine )?;

  println!( "✓ Registered commands for execution demonstration" );

  // Step 2: Execute commands demonstrating different scenarios
  let parser = Parser::new( UnilangParserOptions::default() );

  let test_cases = [
    // Successful executions
    ( "hello Alice", "Simple successful execution" ),
    ( "system.status --verbose", "Command with context and detailed output" ),
    ( "math.divide 42.0 6.0", "Mathematical operation" ),
    ( "data.analyze --numbers 1.5,2.3,4.1,3.7,5.2,2.8,4.6", "Complex data processing" ),

    // Error cases
    ( "math.divide 10.0 0.0", "Division by zero error" ),
    ( "data.analyze --numbers 5.0", "Insufficient data error (needs 2+ numbers)" ),
  ];

  println!( "\n=== Command Execution Test Cases ===" );

  for ( i, ( command_str, description ) ) in test_cases.iter().enumerate()
  {
    println!( "\n--- Test Case {}: {} ---", i + 1, description );
    println!( "🔍 Executing: '{command_str}'" );

    match parser.parse_single_instruction( command_str )
    {
      Ok( instruction ) =>
      {
        let instructions = [ instruction ];
        let analyzer = SemanticAnalyzer::new( &instructions, &registry );

        match analyzer.analyze()
        {
          Ok( verified_commands ) =>
          {
            let interpreter = Interpreter::new( &verified_commands, &registry );
            let mut context = ExecutionContext::default();

            match interpreter.run( &mut context )
            {
              Ok( outputs ) =>
              {
                println!( "✅ Execution completed successfully" );
                for ( j, output ) in outputs.iter().enumerate()
                {
                  println!( "  Output {}: {} (format: {})", j + 1, output.content, output.format );
                }
              },
              Err( error ) =>
              {
                println!( "❌ Execution failed with error:" );
                println!( "  {error}" );
              }
            }
          },
          Err( error ) =>
          {
            println!( "❌ Semantic analysis failed: {error}" );
          }
        }
      },
      Err( error ) =>
      {
        println!( "❌ Parsing failed: {error}" );
      }
    }
  }

  // Step 3: Demonstrate batch execution
  println!( "\n=== Batch Command Execution ===" );

  let batch_commands = vec!
  [
    "hello John",
    "hello Jane",
    "math.divide 100.0 4.0",
    "system.status",
  ];

  println!( "Executing batch of {} commands:", batch_commands.len() );

  let mut all_instructions = Vec::new();
  for cmd_str in &batch_commands
  {
    match parser.parse_single_instruction( cmd_str )
    {
      Ok( instruction ) => all_instructions.push( instruction ),
      Err( e ) => println!( "❌ Failed to parse '{cmd_str}': {e}" ),
    }
  }

  if !all_instructions.is_empty()
  {
    let analyzer = SemanticAnalyzer::new( &all_instructions, &registry );

    match analyzer.analyze()
    {
      Ok( verified_commands ) =>
      {
        println!( "✓ All {} commands verified", verified_commands.len() );

        let interpreter = Interpreter::new( &verified_commands, &registry );
        let mut context = ExecutionContext::default();

        match interpreter.run( &mut context )
        {
          Ok( outputs ) =>
          {
            println!( "✅ Batch execution completed" );
            println!( "  Total outputs: {}", outputs.len() );
          },
          Err( error ) =>
          {
            println!( "❌ Batch execution failed: {error}" );
          }
        }
      },
      Err( error ) =>
      {
        println!( "❌ Batch verification failed: {error}" );
      }
    }
  }

  println!( "\n=== Command Execution Features ===" );
  println!( "🚀 The execution system provides:" );
  println!( "  • Sequential command execution" );
  println!( "  • Proper error handling and propagation" );
  println!( "  • Execution context for shared state" );
  println!( "  • Structured output data" );
  println!( "  • Routine-based command implementation" );
  println!( "  • Type-safe argument access" );
  println!( "  • Batch processing capabilities" );
  println!( "  • Early termination on errors" );

  println!( "\n=== Error Handling Patterns ===" );
  println!( "❌ Commands can fail with structured errors:" );
  println!( "  • Business logic errors (division by zero)" );
  println!( "  • Validation errors (insufficient data)" );
  println!( "  • Resource errors (file not found)" );
  println!( "  • Permission errors (access denied)" );
  println!( "  • Network errors (connection timeout)" );

  println!( "\n=== Best Practices for Command Routines ===" );
  println!( "💡 When implementing command routines:" );
  println!( "  • Validate inputs even after semantic analysis" );
  println!( "  • Provide meaningful error messages" );
  println!( "  • Use appropriate error codes" );
  println!( "  • Handle edge cases gracefully" );
  println!( "  • Return structured output data" );
  println!( "  • Use execution context for shared state" );
  println!( "  • Keep routines focused and testable" );
  println!( "  • Log important operations" );

  Ok(())
}