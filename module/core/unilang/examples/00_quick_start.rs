#![allow(clippy::all)]
//! # Quick Start Example
//!
//! **⚠️ PERFORMANCE WARNING:** This example uses runtime command registration which has
//! **10-50x slower lookup performance** than the recommended compile-time approach.
//!
//! **For production applications, use:** `static_01_basic_compile_time.rs` instead.
//!
//! This example demonstrates the runtime API for educational purposes and is appropriate
//! for REPL applications, plugin systems, or rapid prototyping.
//!
//! Run with: `cargo run --example 00_quick_start`

use unilang::prelude::*;

fn main() -> Result<(), unilang::Error> {
    // Create a command registry
    #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
    
    // Define a simple greeting command
    let greet_cmd = CommandDefinition::former()
      .name( ".greet" )
      .description( "A friendly greeting command" )
      .hint( "Says hello to someone" )
      .arguments( vec![
        ArgumentDefinition {
          name: "name".to_string(),
          description: "Name of the person to greet".to_string(),
          kind: Kind::String,
          hint: "Your name".to_string(),
          attributes: ArgumentAttributes {
            optional: true,
            default: Some("World".to_string()),
            ..Default::default()
          },
          validation_rules: vec![],
          aliases: vec!["n".to_string()],
          tags: vec![],
        }
      ])
      .status( "stable" )
      .version( "1.0.0" )
      .idempotent( true )
      .auto_help_enabled( false )
      .end();
    
    // Define the command's execution logic
    let greet_routine = Box::new(|cmd: VerifiedCommand, _ctx: ExecutionContext| {
        let name = match cmd.arguments.get("name") {
            Some(Value::String(s)) => s.clone(),
            _ => "World".to_string(),
        };
        
        println!("Hello, {name}!");
        
        Ok(OutputData {
            content: format!("Hello, {name}!"),
            format: "text".to_string(),
            execution_time_ms: None,
        })
    });
    
    // Register the command
    #[allow(deprecated)]
    registry.command_add_runtime(&greet_cmd, greet_routine)?;
    
    // Use the Pipeline API to execute commands
    let pipeline = Pipeline::new(registry);
    
    // Execute a command
    println!("=== Executing: .greet name::Alice ===");
    let result = pipeline.process_command_simple(".greet name::Alice");
    println!("Success: {}", result.success);
    println!("Output: {}\n", result.outputs[0].content);
    
    // Execute using the default value
    println!("=== Executing: .greet (using default) ===");
    let result = pipeline.process_command_simple(".greet");
    println!("Success: {}", result.success);
    println!("Output: {}\n", result.outputs[0].content);

    Ok(())
}