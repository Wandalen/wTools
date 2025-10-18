#![allow(clippy::all)]
//! # REPL Loop Example
//!
//! **✅ APPROPRIATE USE OF RUNTIME REGISTRATION:** REPL applications are a legitimate
//! use case for runtime command registration as commands may be defined interactively
//! or loaded dynamically during the session.
//!
//! This example demonstrates a basic Read-Eval-Print Loop (REPL) implementation
//! using Unilang's stateless pipeline components for native applications.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;
use std::io::{ self, Write };

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "=== Basic REPL Loop Example ===\n" );

  // Step 1: Create command registry with sample commands
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();
  register_sample_commands( &mut registry )?;

  // Step 2: Create stateless pipeline for REPL operation
  let pipeline = Pipeline::new( registry );
  println!( "✓ Initialized stateless pipeline\n" );

  // Step 3: Run the REPL loop
  run_repl( &pipeline )?;

  Ok( () )
}

/// Register sample commands for REPL demonstration
#[allow(clippy::too_many_lines)]
fn register_sample_commands( registry : &mut CommandRegistry ) -> Result< (), unilang::error::Error >
{
  // Echo command
  let echo_cmd = CommandDefinition::former()
  .name( ".echo" )
  .namespace( "" )
  .description( "Echoes the provided text".to_string() )
  .hint( "Simple text echo" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".print".to_string() ] )
  .tags( vec![ "utility".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "GET".to_string() )
  .examples( vec![ "echo message::'Hello World'".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "message".to_string(),
      description: "Message to echo".to_string(),
      kind: Kind::String,
      hint: "Text to display".to_string(),
      attributes: ArgumentAttributes { 
        optional: false,
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "msg".to_string(), "text".to_string() ],
      tags: vec![ "required".to_string() ],
    },
  ])
  .end();

  let echo_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx : ExecutionContext |
  {
    let message = cmd.arguments.get( "message" ).map_or_else(|| "No message provided".to_string(), std::string::ToString::to_string);
    
    println!( "🔊 Echo: {message}" );
    
    Ok( OutputData
    {
      content : message,
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &echo_cmd, echo_routine )?;

  // Math command
  let math_cmd = CommandDefinition::former()
  .name( ".add" )
  .namespace( ".math" )
  .description( "Adds two numbers".to_string() )
  .hint( "Simple addition" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".plus".to_string() ] )
  .tags( vec![ "math".to_string(), "arithmetic".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "math.add a::5 b::3".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "a".to_string(),
      description: "First number".to_string(),
      kind: Kind::Integer,
      hint: "First operand".to_string(),
      attributes: ArgumentAttributes { 
        optional: false,
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "first".to_string() ],
      tags: vec![ "required".to_string(), "operand".to_string() ],
    },
    ArgumentDefinition {
      name: "b".to_string(),
      description: "Second number".to_string(),
      kind: Kind::Integer,
      hint: "Second operand".to_string(),
      attributes: ArgumentAttributes { 
        optional: false,
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "second".to_string() ],
      tags: vec![ "required".to_string(), "operand".to_string() ],
    },
  ])
  .end();

  let math_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx : ExecutionContext |
  {
    let a = cmd.arguments.get( "a" )
      .and_then( |v| v.as_integer() )
      .copied()
      .unwrap_or( 0 );
    
    let b = cmd.arguments.get( "b" )
      .and_then( |v| v.as_integer() )
      .copied()
      .unwrap_or( 0 );
    
    let result = a + b;
    let result_msg = format!( "{a} + {b} = {result}" );
    
    println!( "🧮 Math: {result_msg}" );
    
    Ok( OutputData
    {
      content : result.to_string(),
      format : "number".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &math_cmd, math_routine )?;

  println!( "✓ Registered {} sample commands", registry.commands().len() );

  Ok( () )
}

/// Core REPL loop implementation demonstrating stateless operation
fn run_repl( pipeline : &Pipeline ) -> Result< (), Box< dyn core::error::Error > >
{
  println!( "🚀 Starting REPL Session" );
  println!( "Type commands or 'help' for usage, 'quit' to exit\n" );

  let mut session_count = 0u32;

  loop
  {
    // Display prompt
    print!( "repl[{session_count}]> " );
    io::stdout().flush()?;

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line( &mut input )
    {
      Ok( 0 ) => break, // EOF
      Ok( _ ) =>
      {
        let input = input.trim();
        
        // Handle REPL-specific commands
        match input
        {
          "" => continue, // Empty input
          "quit" | "exit" =>
          {
            println!( "👋 Goodbye!" );
            break;
          },
          "help" =>
          {
            display_help();
            continue;
          },
          _ => {}
        }

        // Process command through stateless pipeline
        session_count += 1;
        println!( "🔄 Processing command #{session_count}" );
        
        let context = ExecutionContext::default();
        let result = pipeline.process_command( input, context );
        
        // Handle results (demonstrating stateless operation)
        match result.error
        {
          None =>
          {
            if result.outputs.is_empty() {
              println!( "✅ Command completed (no output)" );
            } else {
              println!( "✅ Command executed successfully" );
              for output in &result.outputs
              {
                if !output.content.is_empty()
                {
                  println!( "📤 Output: {}", output.content );
                }
              }
            }
          },
          Some( error ) =>
          {
            println!( "❌ Error: {error}" );
            println!( "💡 Try 'help' for available commands" );
          }
        }
        
        println!(); // Add spacing
      },
      Err( error ) =>
      {
        println!( "❌ Input error: {error}" );
        break;
      }
    }
  }

  println!( "\n📊 Session completed. Processed {session_count} commands." );
  Ok( () )
}

/// Display help information for REPL users
fn display_help()
{
  println!( "=== REPL Help ===" );
  println!( "📋 Available Commands:" );
  println!( "  • echo message::'text'        - Echo a message" );
  println!( "  • print message::'text'       - Alias for echo" );
  println!( "  • math.add a::5 b::3          - Add two numbers" );
  println!( "  • plus a::10 b::20            - Alias for math.add" );
  
  println!( "\n🛠️ REPL Commands:" );
  println!( "  • help                        - Show this help" );
  println!( "  • quit, exit                  - Exit REPL" );
  
  println!( "\n💡 Examples:" );
  println!( "  echo message::'Hello REPL!'" );
  println!( "  math.add a::42 b::58" );
  println!( "  print text::'Stateless operation demo'" );
  
  println!( "\n🔄 Key Features:" );
  println!( "  ✨ Stateless pipeline - each command is independent" );
  println!( "  ✨ Reusable components - parser, analyzer, interpreter" );
  println!( "  ✨ Memory efficient - no accumulated state between commands" );
  println!( "  ✨ Error isolation - failures don't affect subsequent commands" );
}