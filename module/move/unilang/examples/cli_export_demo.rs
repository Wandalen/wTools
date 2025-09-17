//! CLI Export Demo
//!
//! Demonstrates exporting CLI commands from one module/crate to another
//! and combining multiple CLIs with optional prefixes.

use unilang::prelude::*;

// =============================================================================
// Module A: Math CLI
// =============================================================================

mod math_cli {
    use unilang::prelude::*;

    /// Create math-related commands that can be exported
    pub fn create_math_registry() -> CommandRegistry {
        let mut registry = CommandRegistry::new();
        registry.enable_help_conventions(true);

        // Math add command
        let add_cmd = CommandDefinition::former()
            .name(".add")
            .description("Add two numbers")
            .arguments(vec![
                ArgumentDefinition::former()
                    .name("a")
                    .kind(Kind::Integer)
                    .description("First number")
                    .end(),
                ArgumentDefinition::former()
                    .name("b")
                    .kind(Kind::Integer)
                    .description("Second number")
                    .end()
            ])
            .end();

        let add_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let a = match cmd.arguments.get("a").unwrap() {
                unilang::types::Value::Integer(val) => *val,
                _ => return Err(unilang::data::ErrorData {
                    code: "TYPE_ERROR".to_string(),
                    message: "Expected integer for 'a'".to_string(),
                    source: None,
                }),
            };
            let b = match cmd.arguments.get("b").unwrap() {
                unilang::types::Value::Integer(val) => *val,
                _ => return Err(unilang::data::ErrorData {
                    code: "TYPE_ERROR".to_string(),
                    message: "Expected integer for 'b'".to_string(),
                    source: None,
                }),
            };
            Ok(unilang::data::OutputData {
                content: format!("Result: {}", a + b),
                format: "text".to_string(),
            })
        });

        registry.register_with_auto_help(add_cmd, add_routine).unwrap();

        // Math multiply command
        let mul_cmd = CommandDefinition::former()
            .name(".multiply")
            .description("Multiply two numbers")
            .arguments(vec![
                ArgumentDefinition::former()
                    .name("a")
                    .kind(Kind::Integer)
                    .description("First number")
                    .end(),
                ArgumentDefinition::former()
                    .name("b")
                    .kind(Kind::Integer)
                    .description("Second number")
                    .end()
            ])
            .end();

        let mul_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let a = match cmd.arguments.get("a").unwrap() {
                unilang::types::Value::Integer(val) => *val,
                _ => return Err(unilang::data::ErrorData {
                    code: "TYPE_ERROR".to_string(),
                    message: "Expected integer for 'a'".to_string(),
                    source: None,
                }),
            };
            let b = match cmd.arguments.get("b").unwrap() {
                unilang::types::Value::Integer(val) => *val,
                _ => return Err(unilang::data::ErrorData {
                    code: "TYPE_ERROR".to_string(),
                    message: "Expected integer for 'b'".to_string(),
                    source: None,
                }),
            };
            Ok(unilang::data::OutputData {
                content: format!("Result: {}", a * b),
                format: "text".to_string(),
            })
        });

        registry.register_with_auto_help(mul_cmd, mul_routine).unwrap();

        registry
    }

    /// Export individual command definitions (alternative approach)
    #[allow(dead_code)]
    pub fn get_math_commands() -> Vec<(CommandDefinition, Box<dyn Fn(unilang::semantic::VerifiedCommand, unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData> + Send + Sync>)> {
        vec![
            // Could return individual commands for more granular control
        ]
    }
}

// =============================================================================
// Module B: File CLI
// =============================================================================

mod file_cli {
    use unilang::prelude::*;

    pub fn create_file_registry() -> CommandRegistry {
        let mut registry = CommandRegistry::new();
        registry.enable_help_conventions(true);

        // File list command
        let list_cmd = CommandDefinition::former()
            .name(".list")
            .description("List files in directory")
            .arguments(vec![ArgumentDefinition::former()
                .name("path")
                .kind(Kind::Directory)
                .description("Directory path")
                .attributes(ArgumentAttributes {
                    optional: true,
                    default: Some(".".to_string()),
                    ..Default::default()
                })
                .end()])
            .end();

        let list_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
            let path = if let Some(unilang::types::Value::String(s)) = cmd.arguments.get("path") {
                s
            } else {
                "."
            };
            Ok(unilang::data::OutputData {
                content: format!("Files in {}: file1.txt, file2.txt", path),
                format: "text".to_string(),
            })
        });

        registry.register_with_auto_help(list_cmd, list_routine).unwrap();

        registry
    }
}

// =============================================================================
// Aggregating CLI: Combining Multiple CLIs
// =============================================================================

/// Merge multiple registries with optional prefixes
#[allow(dead_code)]
fn merge_registries_with_prefix(
    _target: &mut CommandRegistry,
    _source: CommandRegistry,
    prefix: Option<&str>
) -> Result<(), unilang::Error> {
    // Extract commands from source registry
    // Note: This is a conceptual implementation - actual extraction would need
    // additional methods in CommandRegistry for full functionality

    println!("Merging registry with prefix: {:?}", prefix);

    // For now, we'll demonstrate the concept by manually adding prefixed commands
    // In a full implementation, we'd need CommandRegistry::extract_commands() method

    Ok(())
}

/// Create an aggregated CLI that combines math and file operations
fn create_aggregated_cli() -> CommandRegistry {
    let mut main_registry = CommandRegistry::new();
    main_registry.enable_help_conventions(true);

    // Get sub-CLIs
    let _math_registry = math_cli::create_math_registry();
    let _file_registry = file_cli::create_file_registry();

    // Method 1: Manual combination with prefixes using namespaces
    // Math commands with "math" prefix
    let math_add_cmd = CommandDefinition::former()
        .name(".add")
        .namespace(".math")  // This creates .math.add
        .description("Add two numbers")
        .arguments(vec![
            ArgumentDefinition::former()
                .name("a")
                .kind(Kind::Integer)
                .description("First number")
                .end(),
            ArgumentDefinition::former()
                .name("b")
                .kind(Kind::Integer)
                .description("Second number")
                .end()
        ])
        .end();

    let math_add_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let a = match cmd.arguments.get("a").unwrap() {
            unilang::types::Value::Integer(val) => *val,
            _ => return Err(unilang::data::ErrorData {
                code: "TYPE_ERROR".to_string(),
                message: "Expected integer for 'a'".to_string(),
                source: None,
            }),
        };
        let b = match cmd.arguments.get("b").unwrap() {
            unilang::types::Value::Integer(val) => *val,
            _ => return Err(unilang::data::ErrorData {
                code: "TYPE_ERROR".to_string(),
                message: "Expected integer for 'b'".to_string(),
                source: None,
            }),
        };
        Ok(unilang::data::OutputData {
            content: format!("Math result: {}", a + b),
            format: "text".to_string(),
        })
    });

    main_registry.register_with_auto_help(math_add_cmd, math_add_routine).unwrap();

    // File commands with "fs" prefix
    let file_list_cmd = CommandDefinition::former()
        .name(".list")
        .namespace(".fs")  // This creates .fs.list
        .description("List files in directory")
        .arguments(vec![ArgumentDefinition::former()
            .name("path")
            .kind(Kind::Directory)
            .description("Directory path")
            .attributes(ArgumentAttributes {
                optional: true,
                default: Some(".".to_string()),
                ..Default::default()
            })
            .end()])
        .end();

    let file_list_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let path = if let Some(unilang::types::Value::String(s)) = cmd.arguments.get("path") {
            s
        } else {
            "."
        };
        Ok(unilang::data::OutputData {
            content: format!("Files in {}: example1.rs, example2.rs", path),
            format: "text".to_string(),
        })
    });

    main_registry.register_with_auto_help(file_list_cmd, file_list_routine).unwrap();

    // Add a main command for the aggregated CLI
    let main_cmd = CommandDefinition::former()
        .name(".info")
        .description("Show information about this aggregated CLI")
        .end();

    let main_routine = Box::new(|_cmd: unilang::semantic::VerifiedCommand, _ctx| {
        Ok(unilang::data::OutputData {
            content: "Aggregated CLI v1.0 - combines math and file operations\nUse .math.add, .fs.list, etc.".to_string(),
            format: "text".to_string(),
        })
    });

    main_registry.register_with_auto_help(main_cmd, main_routine).unwrap();

    main_registry
}

// =============================================================================
// Demo Usage
// =============================================================================

fn main() -> Result<(), unilang::Error> {
    println!("=== CLI Export and Aggregation Demo ===\n");

    // Create the aggregated CLI
    let registry = create_aggregated_cli();
    let pipeline = Pipeline::new(registry);

    // Demo 1: Use math commands with prefix
    println!("1. Math operations (prefixed with 'math'):");
    let result = pipeline.process_command_simple(".math.add a::5 b::3");
    println!("   Command: .math.add a::5 b::3");
    println!("   Result: {}", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Demo 2: Use file commands with prefix
    println!("\n2. File operations (prefixed with 'fs'):");
    let result = pipeline.process_command_simple(".fs.list path::/tmp");
    println!("   Command: .fs.list path::/tmp");
    println!("   Result: {}", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Demo 3: Help system works with prefixes
    println!("\n3. Help system with prefixes:");
    let result = pipeline.process_command_simple(".math.add.help");
    println!("   Command: .math.add.help");
    println!("   Help available: {}", result.success);

    // Demo 4: Main CLI info
    println!("\n4. Main CLI information:");
    let result = pipeline.process_command_simple(".info");
    println!("   Command: .info");
    println!("   Result: {}", result.outputs.get(0).map(|o| &o.content).unwrap_or(&"No output".to_string()));

    // Demo 5: List all available commands
    println!("\n5. All available commands:");
    let result = pipeline.process_command_simple(".");
    println!("   Command: .");
    println!("   Success: {}", result.success);

    Ok(())
}