#![allow(clippy::all)]
//! # Pipeline API Demo
//!
//! This example demonstrates the high-level Pipeline API that simplifies
//! common Unilang workflows by combining parsing, semantic analysis, and
//! execution into convenient helper functions.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::interpreter::ExecutionContext;
use unilang::pipeline::{ Pipeline, process_single_command, validate_single_command };
use unilang::registry::CommandRegistry;
use unilang::types::Value;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== High-Level Pipeline API Demo ===\n" );

  // Step 1: Set up a registry with example commands
  let registry = setup_demo_registry()?;

  println!( "✅ Set up registry with {} commands", registry.commands().len() );

  // ========================================
  // SECTION 1: BASIC PIPELINE USAGE
  // ========================================
  println!( "\n🔄 SECTION 1: Basic Pipeline Usage" );
  println!( "===================================" );

  // Create a pipeline instance (takes ownership)
  let pipeline = Pipeline::new( registry );

  // Process single commands with default context
  let test_commands = vec!
  [
    "calc.add a::15 b::25",
    "text.reverse text::'Hello World'",
    "util.timestamp",
    "help calc.add", // This will fail since help isn't implemented as a command
    "invalid.command", // This will fail
  ];

  for cmd in &test_commands
  {
    println!( "\n📝 Processing: '{cmd}'" );
    let result = pipeline.process_command_simple( cmd );

    if result.success
    {
      println!( "✅ Success!" );
      for output in &result.outputs
      {
        if !output.content.is_empty()
        {
          println!( "   📤 Output: {}", output.content );
        }
      }
    }
    else
    {
      println!( "❌ Failed: {}", result.error.as_ref().unwrap() );
    }
  }

  // ========================================
  // SECTION 2: BATCH PROCESSING
  // ========================================
  println!( "\n📦 SECTION 2: Batch Processing" );
  println!( "===============================" );

  let batch_commands = vec!
  [
    "calc.add a::10 b::20",
    "calc.multiply a::5 b::6",
    "text.reverse 'batch processing'",
    "util.timestamp",
  ];

  println!( "Processing batch of {} commands:", batch_commands.len() );
  let batch_result = pipeline.process_batch( &batch_commands, ExecutionContext::default() );

  println!( "\n📊 Batch Results:" );
  println!( "  Total commands: {}", batch_result.total_commands );
  println!( "  Successful: {}", batch_result.successful_commands );
  println!( "  Failed: {}", batch_result.failed_commands );
  println!( "  Success rate: {:.1}%", batch_result.success_rate() );

  for ( i, result ) in batch_result.results.iter().enumerate()
  {
    let status = if result.success { "✅" } else { "❌" };
    println!( "  {}: {} '{}'", i + 1, status, result.command );
    if let Some( error ) = &result.error
    {
      println!( "     Error: {error}" );
    }
  }

  // ========================================
  // SECTION 3: SEQUENCE PROCESSING (FAIL-FAST)
  // ========================================
  println!( "\n⚡ SECTION 3: Sequence Processing (Fail-Fast)" );
  println!( "=============================================" );

  let sequence_commands = vec!
  [
    "calc.add a::1 b::2",
    "calc.multiply a::3 b::4",
    "invalid.command", // This will cause early termination
    "text.reverse 'this will not run'",
  ];

  println!( "Processing sequence with early termination on failure:" );
  let sequence_result = pipeline.process_sequence( &sequence_commands, ExecutionContext::default() );

  println!( "\n📊 Sequence Results:" );
  println!( "  Commands attempted: {}", sequence_result.results.len() );
  println!( "  Total in sequence: {}", sequence_result.total_commands );
  println!( "  Successful: {}", sequence_result.successful_commands );
  println!( "  Failed: {}", sequence_result.failed_commands );

  if sequence_result.any_failed()
  {
    println!( "  ⚠️  Sequence terminated early due to failure" );
  }

  // ========================================
  // SECTION 4: COMMAND VALIDATION
  // ========================================
  println!( "\n🔍 SECTION 4: Command Validation" );
  println!( "=================================" );

  let validation_tests = vec!
  [
    "calc.add a::10 b::20",           // Valid
    "text.reverse text::hello",       // Valid
    "util.timestamp",           // Valid
    "invalid.command",          // Invalid - command not found
    "calc.add",                 // Invalid - missing arguments
    "calc.add a::10 b::20 c::30",       // Invalid - too many arguments
  ];

  println!( "Validating commands without execution:" );
  for cmd in &validation_tests
  {
    print!( "  '{cmd}' -> " );
    match pipeline.validate_command( cmd )
    {
      Ok( () ) => println!( "✅ Valid" ),
      Err( e ) => println!( "❌ Invalid: {e}" ),
    }
  }

  // Batch validation
  println!( "\nBatch validation:" );
  let validation_results = pipeline.validate_batch( &validation_tests );
  let valid_count = validation_results.iter().filter( | r | r.is_ok() ).count();
  println!( "  {}/{} commands are valid", valid_count, validation_tests.len() );

  // ========================================
  // SECTION 5: CONVENIENCE FUNCTIONS
  // ========================================
  println!( "\n🎯 SECTION 5: Convenience Functions" );
  println!( "====================================" );

  // Single command processing without creating a pipeline
  println!( "Using convenience functions for one-off operations:" );

  // Create a new registry for convenience functions since pipeline took ownership
  let convenience_registry = setup_demo_registry()?;
  let result = process_single_command( "calc.add a::100 b::200", &convenience_registry, ExecutionContext::default() );
  if result.success
  {
    println!( "✅ Single command result: {}", result.outputs[ 0 ].content );
  }

  // Single command validation
  match validate_single_command( "text.reverse 'hello'", &convenience_registry )
  {
    Ok( () ) => println!( "✅ Command validation passed" ),
    Err( e ) => println!( "❌ Command validation failed: {e}" ),
  }

  // ========================================
  // SECTION 6: ERROR HANDLING PATTERNS
  // ========================================
  println!( "\n🛡️  SECTION 6: Error Handling Patterns" );
  println!( "=======================================" );

  let error_test_commands = vec!
  [
    ( "calc.divide 10 0", "Division by zero" ),
    ( "text.process", "Missing required argument" ),
    ( "nonexistent.command", "Command not found" ),
    ( "calc.add a::abc b::def", "Type conversion error" ),
  ];

  for ( cmd, expected_error_type ) in &error_test_commands
  {
    println!( "\n🧪 Testing {expected_error_type}: '{cmd}'" );
    let result = pipeline.process_command_simple( cmd );

    if result.success {
      println!( "   ⚠️  Unexpected success" );
    } else {
      println!( "   ❌ Expected failure: {}", result.error.as_ref().unwrap() );
    }
  }

  // ========================================
  // SECTION 7: PERFORMANCE COMPARISON
  // ========================================
  println!( "\n⚡ SECTION 7: Performance Comparison" );
  println!( "====================================" );

  let repeated_command = "calc.add a::1 b::1";
  let iterations = 10;

  // Using pipeline (reuses parser and registry)
  let start = std::time::Instant::now();
  for _ in 0..iterations
  {
    let _ = pipeline.process_command_simple( repeated_command );
  }
  let pipeline_duration = start.elapsed();

  // Using convenience function (creates new pipeline each time)
  let start = std::time::Instant::now();
  for _ in 0..iterations
  {
    let _ = process_single_command( repeated_command, &convenience_registry, ExecutionContext::default() );
  }
  let convenience_duration = start.elapsed();

  println!( "Performance comparison ({iterations} iterations):" );
  println!( "  Pipeline (reused): {pipeline_duration:?}" );
  println!( "  Convenience func:  {convenience_duration:?}" );
  println!( "  Ratio: {:.2}x", convenience_duration.as_nanos() as f64 / pipeline_duration.as_nanos() as f64 );

  println!( "\n=== Pipeline API Features Summary ===" );
  println!( "🎯 The Pipeline API provides:" );
  println!( "  • High-level command processing with error handling" );
  println!( "  • Batch processing with success/failure tracking" );
  println!( "  • Sequence processing with fail-fast behavior" );
  println!( "  • Command validation without execution" );
  println!( "  • Convenience functions for one-off operations" );
  println!( "  • Structured result objects with detailed information" );
  println!( "  • Performance benefits through component reuse" );

  println!( "\n💡 Usage Recommendations:" );
  println!( "  • Use Pipeline for repeated operations (better performance)" );
  println!( "  • Use convenience functions for simple one-off commands" );
  println!( "  • Use batch processing for independent command sets" );
  println!( "  • Use sequence processing when order matters and failures should stop execution" );
  println!( "  • Use validation for command verification without side effects" );

  Ok(())
}

/// Set up a demo registry with various commands for testing
fn setup_demo_registry() -> Result< CommandRegistry, unilang::error::Error >
{
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Calculator commands
  setup_calc_commands( &mut registry )?;

  // Text processing commands
  setup_text_commands( &mut registry )?;

  // Utility commands
  setup_util_commands( &mut registry )?;

  Ok( registry )
}

/// Set up calculator commands
#[allow(clippy::too_many_lines)]
fn setup_calc_commands( registry : &mut CommandRegistry ) -> Result< (), unilang::error::Error >
{
  // Add command
  let add_cmd = CommandDefinition::former()
  .name( "add" )
  .namespace( ".calc".to_string() )
  .description( "Adds two numbers".to_string() )
  .hint( "Addition operation" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "plus".to_string() ] )
  .tags( vec![ "math".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "calc.add a::10 b::20".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "a".to_string(),
      description: "First number".to_string(),
      kind: Kind::Integer,
      hint: "First addend".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    },
    ArgumentDefinition {
      name: "b".to_string(),
      description: "Second number".to_string(),
      kind: Kind::Integer,
      hint: "Second addend".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    },
  ])
  .end();

  let add_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let a = cmd.arguments.get( "a" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let b = cmd.arguments.get( "b" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let result = a + b;
    println!( "🧮 {a} + {b} = {result}" );

    Ok( OutputData
    {
      content : result.to_string(),
      format : "integer".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &add_cmd, add_routine )?;

  // Multiply command
  let multiply_cmd = CommandDefinition::former()
  .name( "multiply" )
  .namespace( ".calc".to_string() )
  .description( "Multiplies two numbers".to_string() )
  .hint( "Multiplication operation" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "mul".to_string(), "times".to_string() ] )
  .tags( vec![ "math".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "calc.multiply 5 6".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "a".to_string(),
      description: "First number".to_string(),
      kind: Kind::Integer,
      hint: "Multiplicand".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    },
    ArgumentDefinition {
      name: "b".to_string(),
      description: "Second number".to_string(),
      kind: Kind::Integer,
      hint: "Multiplier".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    },
  ])
  .end();

  let multiply_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let a = cmd.arguments.get( "a" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let b = cmd.arguments.get( "b" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let result = a * b;
    println!( "🧮 {a} × {b} = {result}" );

    Ok( OutputData
    {
      content : result.to_string(),
      format : "integer".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &multiply_cmd, multiply_routine )?;

  // Divide command (with error handling)
  let divide_cmd = CommandDefinition::former()
  .name( "divide" )
  .namespace( ".calc".to_string() )
  .description( "Divides two numbers".to_string() )
  .hint( "Division operation" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "div".to_string() ] )
  .tags( vec![ "math".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "calc.divide 20 4".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "dividend".to_string(),
      description: "Number to be divided".to_string(),
      kind: Kind::Integer,
      hint: "Dividend".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    },
    ArgumentDefinition {
      name: "divisor".to_string(),
      description: "Number to divide by".to_string(),
      kind: Kind::Integer,
      hint: "Divisor".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    },
  ])
  .end();

  let divide_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let dividend = cmd.arguments.get( "dividend" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );
    let divisor = cmd.arguments.get( "divisor" ).and_then( | v | if let Value::Integer( i ) = v { Some( i ) } else { None } ).unwrap_or( &0 );

    if *divisor == 0
    {
      return Err( unilang::data::ErrorData::new(
        "DIVISION_BY_ZERO".to_string(),
        "Cannot divide by zero".to_string(),
      ));
    }

    let result = dividend / divisor;
    println!( "🧮 {dividend} ÷ {divisor} = {result}" );

    Ok( OutputData
    {
      content : result.to_string(),
      format : "integer".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &divide_cmd, divide_routine )?;

  Ok(())
}

/// Set up text processing commands
fn setup_text_commands( registry : &mut CommandRegistry ) -> Result< (), unilang::error::Error >
{
  let reverse_cmd = CommandDefinition::former()
  .name( "reverse" )
  .namespace( ".text".to_string() )
  .description( "Reverses a text string".to_string() )
  .hint( "String reversal" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "rev".to_string() ] )
  .tags( vec![ "text".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "text.reverse 'hello world'".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "text".to_string(),
      description: "Text to reverse".to_string(),
      kind: Kind::String,
      hint: "Input text".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength(1) ],
      aliases: vec![],
      tags: vec![],
    },
  ])
  .end();

  let reverse_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let default_text = String::new();
    let text = cmd.arguments.get( "text" ).and_then( | v | if let Value::String( s ) = v { Some( s ) } else { None } ).unwrap_or( &default_text );
    let reversed : String = text.chars().rev().collect();
    println!( "🔄 '{text}' → '{reversed}'" );

    Ok( OutputData
    {
      content : reversed,
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &reverse_cmd, reverse_routine )?;

  Ok(())
}

/// Set up utility commands
fn setup_util_commands( registry : &mut CommandRegistry ) -> Result< (), unilang::error::Error >
{
  let timestamp_cmd = CommandDefinition::former()
  .name( "timestamp" )
  .namespace( ".util".to_string() )
  .description( "Shows current timestamp".to_string() )
  .hint( "Current time" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ "time".to_string(), "now".to_string() ] )
  .tags( vec![ "utility".to_string(), "time".to_string() ] )
  .permissions( vec![] )
  .idempotent( false ) // Time changes
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "util.timestamp".to_string() ] )
  .arguments( vec![] )
  .end();

  let timestamp_routine = Box::new( | _cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since( std::time::UNIX_EPOCH ).unwrap();
    let timestamp = duration.as_secs();
    println!( "🕐 Current timestamp: {timestamp}" );

    Ok( OutputData
    {
      content : timestamp.to_string(),
      format : "integer".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &timestamp_cmd, timestamp_routine )?;

  Ok(())
}