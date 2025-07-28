//! # Basic Command Registration
//! 
//! This example demonstrates how to register commands in the Unilang command registry
//! and execute them using the basic CLI functionality.

use std::collections::HashMap;
use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes, OutputData};
use unilang::registry::CommandRegistry;
use unilang::types::Value;

fn main() -> Result<(), unilang::error::Error> {
    println!("=== Basic Command Registration Example ===\n");

    // Step 1: Create a new command registry
    let mut registry = CommandRegistry::new();
    println!("✓ Created command registry");

    // Step 2: Define a simple greeting command
    let greet_command = CommandDefinition::former()
        .name("greet")
        .namespace("".to_string()) // Global namespace
        .description("A simple greeting command".to_string())
        .hint("Greets a person by name")
        .status("stable")
        .version("1.0.0")
        .aliases(vec!["hello".to_string()])
        .tags(vec!["greeting".to_string(), "demo".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("GET".to_string())
        .examples(vec!["greet Alice".to_string(), "greet".to_string()])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("name")
                .description("Name of the person to greet".to_string())
                .kind(Kind::String)
                .hint("Person's name")
                .default_value(Some("World".to_string()))
                .attributes(
                    ArgumentAttributes::former()
                        .optional(true)
                        .multiple(false)
                        .is_default_arg(true)
                        .interactive(false)
                        .sensitive(false)
                        .end()
                )
                .validation_rules(vec!["min_length:1".to_string()])
                .aliases(vec!["n".to_string()])
                .tags(vec!["required".to_string()])
                .end()
        ])
        .end();

    // Step 3: Define a routine (execution logic) for the command
    let greet_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let name = match cmd.arguments.get("name") {
            Some(Value::String(n)) => n.clone(),
            _ => "World".to_string(),
        };
        
        let greeting = format!("Hello, {}!", name);
        println!("{}", greeting);
        
        Ok(OutputData {
            content: greeting,
            format: "text".to_string(),
        })
    });

    // Step 4: Register the command with its routine
    registry.command_add_runtime(&greet_command, greet_routine)?;
    println!("✓ Registered 'greet' command with routine");

    // Step 5: Verify registration by listing commands
    println!("\nRegistered commands:");
    for (name, cmd) in &registry.commands {
        println!("  {} - {}", name, cmd.description);
    }

    println!("\n=== Example Complete ===");
    println!("To test this command, run:");
    println!("  cargo run --bin unilang_cli greet Alice");
    println!("  cargo run --bin unilang_cli greet");
    
    Ok(())
}