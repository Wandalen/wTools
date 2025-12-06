#![allow(clippy::all)]
//! # Quick Start Example - Runtime Registration
//!
//! **⚠️ PERFORMANCE NOTICE:** This example demonstrates runtime command registration,
//! which has **10-50x slower performance** than compile-time registration.
//!
//! **When runtime registration IS appropriate:**
//! - ✅ REPL applications with interactive command loading
//! - ✅ Plugin systems with runtime command discovery
//! - ✅ Rapid prototyping and development iteration
//!
//! **When to use compile-time registration instead:**
//! - ⚡ Production CLIs (see `static_01_basic_compile_time.rs`)
//! - ⚡ Performance-critical applications
//! - ⚡ Large command sets (100+ commands)
//!
//! **Performance comparison:** ~500ns (runtime) vs ~80ns (compile-time) per lookup.
//!
//! Run with: `cargo run --example 00_quick_start`

use unilang::prelude::*;

fn main() -> Result<(), unilang::Error> {
    // Create a command registry
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