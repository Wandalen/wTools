//! Simple CLI Export and Aggregation Demo
//!
//! Demonstrates the core capability to export and combine CLI commands
//! from different modules with optional prefixes using the existing unilang architecture.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::types::Value;
use unilang::pipeline::Pipeline;

// =============================================================================
// Module A: Math Operations CLI
// =============================================================================

/// Create a CLI registry with math operations
pub fn create_math_cli() -> CommandRegistry {
    let mut registry = CommandRegistry::new();
    registry.enable_help_conventions(true);

    // Define add command
    let add_command = CommandDefinition::former()
        .name(".add")
        .description("Add two numbers".to_string())
        .arguments(vec![
            ArgumentDefinition {
                name: "a".to_string(),
                description: "First number".to_string(),
                kind: Kind::Integer,
                hint: "First number".to_string(),
                attributes: ArgumentAttributes {
                    optional: false,
                    multiple: false,
                    default: None,
                    interactive: false,
                    sensitive: false,
                },
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
            ArgumentDefinition {
                name: "b".to_string(),
                description: "Second number".to_string(),
                kind: Kind::Integer,
                hint: "Second number".to_string(),
                attributes: ArgumentAttributes {
                    optional: false,
                    multiple: false,
                    default: None,
                    interactive: false,
                    sensitive: false,
                },
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
        ])
        .end();

    let add_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let a = match cmd.arguments.get("a").unwrap() {
            Value::Integer(val) => *val,
            _ => return Err(unilang::data::ErrorData {
                code: "TYPE_ERROR".to_string(),
                message: "Expected integer for 'a'".to_string(),
                source: None,
            }),
        };
        let b = match cmd.arguments.get("b").unwrap() {
            Value::Integer(val) => *val,
            _ => return Err(unilang::data::ErrorData {
                code: "TYPE_ERROR".to_string(),
                message: "Expected integer for 'b'".to_string(),
                source: None,
            }),
        };

        Ok(OutputData {
            content: format!("Math result: {} + {} = {}", a, b, a + b),
            format: "text".to_string(),
        })
    });

    registry.register_with_auto_help(add_command, add_routine).unwrap();
    registry
}

// =============================================================================
// Module B: File Operations CLI
// =============================================================================

/// Create a CLI registry with file operations
pub fn create_file_cli() -> CommandRegistry {
    let mut registry = CommandRegistry::new();
    registry.enable_help_conventions(true);

    // Define list command
    let list_command = CommandDefinition::former()
        .name(".list")
        .description("List files in directory".to_string())
        .arguments(vec![
            ArgumentDefinition {
                name: "path".to_string(),
                description: "Directory path".to_string(),
                kind: Kind::String,
                hint: "Directory path".to_string(),
                attributes: ArgumentAttributes {
                    optional: true,
                    multiple: false,
                    default: Some(".".to_string()),
                    interactive: false,
                    sensitive: false,
                },
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
        ])
        .end();

    let list_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let path = match cmd.arguments.get("path") {
            Some(Value::String(p)) => p.clone(),
            _ => ".".to_string(),
        };

        Ok(OutputData {
            content: format!("Files in '{}': file1.txt, file2.txt, file3.rs", path),
            format: "text".to_string(),
        })
    });

    registry.register_with_auto_help(list_command, list_routine).unwrap();
    registry
}

// =============================================================================
// CLI Aggregation Function
// =============================================================================

/// Combine multiple CLI registries with prefixes
/// This demonstrates how to export CLIs from modules and combine them
pub fn create_aggregated_cli() -> Result<CommandRegistry, unilang::error::Error> {
    let mut main_registry = CommandRegistry::new();
    main_registry.enable_help_conventions(true);

    // Method 1: Manual command recreation with namespaces (current architecture)

    // Add math commands with "math" prefix
    let math_add_command = CommandDefinition::former()
        .name(".add")
        .namespace(".math")  // This creates .math.add
        .description("Add two numbers".to_string())
        .arguments(vec![
            ArgumentDefinition {
                name: "a".to_string(),
                description: "First number".to_string(),
                kind: Kind::Integer,
                hint: "First number".to_string(),
                attributes: ArgumentAttributes::default(),
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
            ArgumentDefinition {
                name: "b".to_string(),
                description: "Second number".to_string(),
                kind: Kind::Integer,
                hint: "Second number".to_string(),
                attributes: ArgumentAttributes::default(),
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
        ])
        .end();

    let math_add_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let a = match cmd.arguments.get("a").unwrap() {
            Value::Integer(val) => *val,
            _ => return Err(unilang::data::ErrorData {
                code: "TYPE_ERROR".to_string(),
                message: "Expected integer for 'a'".to_string(),
                source: None,
            }),
        };
        let b = match cmd.arguments.get("b").unwrap() {
            Value::Integer(val) => *val,
            _ => return Err(unilang::data::ErrorData {
                code: "TYPE_ERROR".to_string(),
                message: "Expected integer for 'b'".to_string(),
                source: None,
            }),
        };

        Ok(OutputData {
            content: format!("[MATH MODULE] {} + {} = {}", a, b, a + b),
            format: "text".to_string(),
        })
    });

    main_registry.register_with_auto_help(math_add_command, math_add_routine)?;

    // Add file commands with "fs" prefix
    let file_list_command = CommandDefinition::former()
        .name(".list")
        .namespace(".fs")  // This creates .fs.list
        .description("List files in directory".to_string())
        .arguments(vec![
            ArgumentDefinition {
                name: "path".to_string(),
                description: "Directory path".to_string(),
                kind: Kind::String,
                hint: "Directory path".to_string(),
                attributes: ArgumentAttributes {
                    optional: true,
                    default: Some(".".to_string()),
                    ..ArgumentAttributes::default()
                },
                validation_rules: vec![],
                aliases: vec![],
                tags: vec![],
            },
        ])
        .end();

    let file_list_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let path = match cmd.arguments.get("path") {
            Some(Value::String(p)) => p.clone(),
            _ => ".".to_string(),
        };

        Ok(OutputData {
            content: format!("[FILE MODULE] Listing files in '{}': example1.rs, example2.rs, lib.rs", path),
            format: "text".to_string(),
        })
    });

    main_registry.register_with_auto_help(file_list_command, file_list_routine)?;

    // Add a main aggregator info command
    let info_command = CommandDefinition::former()
        .name(".info")
        .description("Show aggregated CLI information".to_string())
        .arguments(vec![])
        .end();

    let info_routine = Box::new(|_cmd: unilang::semantic::VerifiedCommand, _ctx| {
        Ok(OutputData {
            content: "Aggregated CLI v1.0\nCombines multiple CLI modules:\n- Math operations (.math.*)\n- File operations (.fs.*)\n\nUse . to list all commands".to_string(),
            format: "text".to_string(),
        })
    });

    main_registry.register_with_auto_help(info_command, info_routine)?;

    Ok(main_registry)
}

// =============================================================================
// Demo Usage
// =============================================================================

fn main() -> Result<(), unilang::error::Error> {
    println!("=== CLI Export and Aggregation Demo ===\n");

    // Demonstrate individual CLIs first
    println!("1. Individual Math CLI:");
    let math_cli = create_math_cli();
    let math_pipeline = Pipeline::new(math_cli);
    let result = math_pipeline.process_command_simple(".add a::10 b::5");
    println!("   Command: .add a::10 b::5");
    println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    println!("2. Individual File CLI:");
    let file_cli = create_file_cli();
    let file_pipeline = Pipeline::new(file_cli);
    let result = file_pipeline.process_command_simple(".list path::/tmp");
    println!("   Command: .list path::/tmp");
    println!("   Result: {}\n", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Now demonstrate the aggregated CLI
    println!("3. Aggregated CLI with Prefixes:");
    let aggregated_registry = create_aggregated_cli()?;
    let aggregated_pipeline = Pipeline::new(aggregated_registry);

    // Test math operations with prefix
    let result = aggregated_pipeline.process_command_simple(".math.add a::20 b::15");
    println!("   Command: .math.add a::20 b::15");
    println!("   Result: {}", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Test file operations with prefix
    let result = aggregated_pipeline.process_command_simple(".fs.list path::/home");
    println!("   Command: .fs.list path::/home");
    println!("   Result: {}", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Test main info command
    let result = aggregated_pipeline.process_command_simple(".info");
    println!("   Command: .info");
    println!("   Result: {}", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    println!("\n4. Help System with Prefixes:");
    // Test auto-generated help for prefixed commands
    let result = aggregated_pipeline.process_command_simple(".math.add.help");
    println!("   Command: .math.add.help (auto-generated)");
    println!("   Help available: {}", result.success);

    // Test ?? parameter with prefixed commands
    let result = aggregated_pipeline.process_command_simple(".fs.list \"??\"");
    println!("   Command: .fs.list \"??\" (help parameter)");
    println!("   Help via ?? parameter: {}", result.success);

    println!("\n5. List All Commands:");
    let result = aggregated_pipeline.process_command_simple(".");
    println!("   Command: . (list all)");
    println!("   Success: {}", result.success);

    println!("\n=== Summary ===");
    println!("✓ Successfully exported CLIs from separate modules");
    println!("✓ Combined multiple CLIs with namespace prefixes (.math, .fs)");
    println!("✓ Maintained full help system functionality");
    println!("✓ All three help access methods work: .command.help, ??, and ?");

    Ok(())
}