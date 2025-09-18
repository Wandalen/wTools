#![allow(clippy::all)]
//! # Pipeline API Basics
//!
//! This example demonstrates the Pipeline API - the recommended high-level way
//! to work with unilang commands. The Pipeline handles the complete flow:
//! parsing → validation → execution.
//!
//! Run with: `cargo run --example 00_pipeline_basics`

use unilang::prelude::*;

fn main() -> Result<(), unilang::Error> {
    println!("=== Pipeline API Basics ===\n");
    
    // Step 1: Set up a registry with some commands
    #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
    
    // Register a simple math command
    let add_cmd = CommandDefinition {
        name: "add".to_string(),
        namespace: ".math".to_string(),
        description: "Adds two numbers".to_string(),
        hint: "Addition operation".to_string(),
        arguments: vec![
            ArgumentDefinition {
                name: "a".to_string(),
                description: "First number".to_string(),
                kind: Kind::Integer,
                hint: "First operand".to_string(),
                attributes: ArgumentAttributes::default(),
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
            ArgumentDefinition {
                name: "b".to_string(),
                description: "Second number".to_string(),
                kind: Kind::Integer,
                hint: "Second operand".to_string(),
                attributes: ArgumentAttributes::default(),
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
        ],
        status: "stable".to_string(),
        version: "1.0.0".to_string(),
        aliases: vec![],
        tags: vec![],
        permissions: vec![],
        idempotent: true,
        deprecation_message: String::new(),
        http_method_hint: String::new(),
        examples: vec![],
        routine_link: None,
        auto_help_enabled: false,
    };
    
    let add_routine = Box::new(|cmd: VerifiedCommand, _ctx: ExecutionContext| {
        if let (Some(Value::Integer(a)), Some(Value::Integer(b))) = 
            (cmd.arguments.get("a"), cmd.arguments.get("b")) {
            let result = a + b;
            println!("{a} + {b} = {result}");
            
            Ok(OutputData {
                content: result.to_string(),
                format: "text".to_string(),
            })
        } else {
            unreachable!("Arguments already validated")
        }
    });
    
    #[allow(deprecated)]
    registry.command_add_runtime(&add_cmd, add_routine)?;
    
    // Step 2: Create a Pipeline
    // The Pipeline wraps the registry and provides high-level execution methods
    let pipeline = Pipeline::new(registry);
    
    // Step 3: Execute commands using process_command_simple()
    println!("--- Simple Command Execution ---");
    
    // Success case
    let result = pipeline.process_command_simple("math.add a::5 b::3");
    println!("Command: math.add a::5 b::3");
    println!("Success: {}", result.success);
    println!("Output: {}", result.outputs[0].content);
    println!("Error: {:?}\n", result.error);
    
    // Error case - missing argument
    let result = pipeline.process_command_simple("math.add a::5");
    println!("Command: math.add a::5 (missing b)");
    println!("Success: {}", result.success);
    println!("Output: {:?}", result.outputs.first().map(|o| &o.content));
    println!("Error: {:?}\n", result.error);
    
    // Error case - invalid command
    let result = pipeline.process_command_simple("math.multiply a::5 b::3");
    println!("Command: math.multiply a::5 b::3 (unknown command)");
    println!("Success: {}", result.success);
    println!("Error: {:?}\n", result.error);
    
    // Step 4: Batch Processing
    println!("--- Batch Processing ---");
    let commands = vec![
        "math.add a::1 b::2",
        "math.add a::10 b::20",
        "math.add a::100 b::200",
        "math.add a::invalid b::3",  // This will fail
    ];
    
    let batch_result = pipeline.process_batch(&commands, ExecutionContext::default());
    println!("Processed {} commands", batch_result.total_commands);
    println!("Successful: {}", batch_result.successful_commands);
    println!("Failed: {}", batch_result.failed_commands);
    println!("Success rate: {:.1}%", batch_result.success_rate());
    
    // Show individual results
    for (i, result) in batch_result.results.iter().enumerate() {
        println!("\nCommand {}: {}", i + 1, commands[i]);
        if result.success {
            println!("  ✓ Output: {}", result.outputs[0].content);
        } else {
            println!("  ✗ Error: {}", result.error.as_ref().unwrap());
        }
    }
    
    // Step 5: Sequential Processing with Early Exit
    println!("\n--- Sequential Processing (stops on first error) ---");
    let sequence_result = pipeline.process_sequence(&commands, ExecutionContext::default());
    println!("Stopped after {} commands", sequence_result.results.len());
    println!("Last command successful: {}", 
        sequence_result.results.last().is_some_and(|r| r.success));
    
    Ok(())
}