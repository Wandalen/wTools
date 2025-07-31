//! # Basic Command Registration Example
//!
//! This example demonstrates the fundamental concepts of unilang:
//! 1. Creating a command registry to store all commands
//! 2. Defining a command with its metadata and arguments
//! 3. Creating an execution routine (the actual logic)
//! 4. Registering the command with the registry
//!
//! This is the simplest possible example - a "Hello World" style greeting command.
//!
//! ## What You'll Learn:
//! - How to create and configure a CommandRegistry
//! - How to define a CommandDefinition with arguments
//! - How to implement command execution logic
//! - How to register commands for runtime execution

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::types::Value;

fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Basic Command Registration Example ===\n" );

  // Step 1: Create the Command Registry
  // The registry is the central storage for all command definitions and their execution routines.
  // Think of it as a dictionary that maps command names to their implementations.
  let mut registry = CommandRegistry::new();
  println!( "✓ Created command registry" );

  // Step 2: Define a Command
  // A CommandDefinition describes everything about a command:
  // - Its name and namespace (for organization)
  // - Description and help text
  // - Arguments it accepts
  // - Metadata like version, status, aliases
  let greet_command = CommandDefinition::former()
  .name( "greet" )                              // The command name users will type
  .namespace( "".to_string() )                  // Empty = global namespace (no prefix needed)
  .description( "A simple greeting command".to_string() )
  .hint( "Greets a person by name" )           // Short hint shown in command lists
  .status( "stable" )                           // Can be: stable, beta, experimental, deprecated
  .version( "1.0.0" )                          // Semantic versioning
  .aliases( vec![ "hello".to_string() ] )       // Alternative names (users can type 'hello' instead)
  .tags( vec![ "greeting".to_string(), "demo".to_string() ] )  // For categorization
  .permissions( vec![] )                        // Empty = no special permissions needed
  .idempotent( true )                          // Safe to run multiple times
  .deprecation_message( "".to_string() )        // Used when status is "deprecated"
  .http_method_hint( "GET".to_string() )        // Hint for REST API generation
  .examples( vec![ 
    "greet name::\"Alice\"".to_string(),        // Example with argument
    "greet".to_string()                         // Example using default
  ])
  .arguments( vec!
  [
    // Define the 'name' argument
    ArgumentDefinition {
      name: "name".to_string(),                 // Argument identifier
      description: "Name of the person to greet".to_string(),
      kind: Kind::String,                       // Data type (String, Integer, Boolean, etc.)
      hint: "Person's name".to_string(),        // Short hint for this argument
      
      // Argument behavior configuration
      attributes: ArgumentAttributes {
        optional: true,                         // User doesn't have to provide this
        multiple: false,                        // Can't provide multiple values
        default: Some("World".to_string()),     // Default value when not provided
        interactive: false,                     // Don't prompt user for input
        sensitive: false,                       // Not sensitive (like passwords)
      },
      
      // Validation rules - ensure minimum length of 1 character
      validation_rules: vec![ ValidationRule::MinLength(1) ],
      
      // Users can type 'n' instead of 'name'
      aliases: vec![ "n".to_string() ],
      
      // Tags for this argument (useful for documentation/filtering)
      tags: vec![ "input".to_string() ],
    }
  ])
  .end();

  // Step 3: Define the Execution Logic
  // This is the actual code that runs when the command is executed.
  // It receives:
  // - cmd: A VerifiedCommand with parsed and validated arguments
  // - _ctx: ExecutionContext for environment variables, config, etc. (unused here)
  let greet_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    // Extract the 'name' argument value
    // Arguments are stored as a HashMap<String, Value>
    let name = match cmd.arguments.get( "name" )
    {
      Some( Value::String( n ) ) => n.clone(),  // User provided a name
      _ => "World".to_string(),                 // Use default (shouldn't happen due to default value)
    };

    // Format the greeting message
    let greeting = format!( "Hello, {}!", name );
    
    // Print to console (for CLI mode)
    println!( "{}", greeting );

    // Return the output data
    // This allows the same command to work in different contexts (CLI, API, etc.)
    Ok( OutputData
    {
      content : greeting,       // The actual output
      format : "text".to_string(),  // Format hint (text, json, xml, etc.)
    })
  });

  // Step 4: Register the Command
  // This connects the command definition with its execution routine.
  // After this, the command can be looked up and executed by name.
  registry.command_add_runtime( &greet_command, greet_routine )?;
  println!( "✓ Registered 'greet' command with routine" );

  // Step 5: Verify Registration
  // Let's list all registered commands to confirm our command is there
  println!( "\nRegistered commands:" );
  for ( name, cmd ) in &registry.commands()
  {
    println!( "  {} - {}", name, cmd.description );
  }

  // Show how to test the command
  println!( "\n=== Example Complete ===" );
  println!( "\nTo test this command using the CLI, run:" );
  println!( "  cargo run --bin unilang_cli greet name::\"Alice\"" );
  println!( "  cargo run --bin unilang_cli greet" );
  println!( "\nOr use the hello alias:" );
  println!( "  cargo run --bin unilang_cli hello name::\"Bob\"" );
  
  println!( "\nNote: This example only registers the command." );
  println!( "To actually execute it, you need to use the Pipeline API" );
  println!( "or run it through unilang_cli as shown above." );

  Ok(())
}