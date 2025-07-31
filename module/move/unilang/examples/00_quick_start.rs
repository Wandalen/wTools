//! # Quick Start Example
//!
//! This example shows the complete flow from command definition to execution
//! in the simplest possible way. It matches the example from the README.
//!
//! Run with: `cargo run --example 00_quick_start`

use unilang::prelude::*;

fn main() -> Result<(), unilang::Error> {
    // Create a command registry
    let mut registry = CommandRegistry::new();
    
    // Define a simple greeting command
    let greet_cmd = CommandDefinition {
        name: "greet".to_string(),
        namespace: String::new(),  // Global namespace
        description: "A friendly greeting command".to_string(),
        hint: "Says hello to someone".to_string(),
        arguments: vec![
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
        ],
        // ... other fields with defaults
        aliases: vec!["hello".to_string()],
        status: "stable".to_string(),
        version: "1.0.0".to_string(),
        tags: vec![],
        permissions: vec![],
        idempotent: true,
        deprecation_message: String::new(),
        http_method_hint: String::new(),
        examples: vec![],
        routine_link: None,
    };
    
    // Define the command's execution logic
    let greet_routine = Box::new(|cmd: VerifiedCommand, _ctx: ExecutionContext| {
        let name = match cmd.arguments.get("name") {
            Some(Value::String(s)) => s.clone(),
            _ => "World".to_string(),
        };
        
        println!("Hello, {}!", name);
        
        Ok(OutputData {
            content: format!("Hello, {}!", name),
            format: "text".to_string(),
        })
    });
    
    // Register the command
    registry.command_add_runtime(&greet_cmd, greet_routine)?;
    
    // Use the Pipeline API to execute commands
    let pipeline = Pipeline::new(registry);
    
    // Execute a command
    println!("=== Executing: greet name::Alice ===");
    let result = pipeline.process_command_simple("greet name::Alice");
    println!("Success: {}", result.success);
    println!("Output: {}\n", result.outputs[0].content);
    
    // Execute using the default value
    println!("=== Executing: greet (using default) ===");
    let result = pipeline.process_command_simple("greet");
    println!("Success: {}", result.success);
    println!("Output: {}\n", result.outputs[0].content);
    
    // Execute using the alias
    println!("=== Executing: hello name::Bob (using alias) ===");
    let result = pipeline.process_command_simple("hello name::Bob");
    println!("Success: {}", result.success);
    println!("Output: {}", result.outputs[0].content);
    
    Ok(())
}