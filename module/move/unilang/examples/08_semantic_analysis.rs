//! # Semantic Analysis Demo
//! 
//! This example demonstrates the semantic analysis phase, showing how
//! parsed commands are validated against the registry and converted
//! to verified commands ready for execution.

use std::collections::HashMap;
use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes, OutputData};
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;
use unilang_parser::{Parser, UnilangParserOptions, GenericInstruction, GenericArgument};

fn main() -> Result<(), unilang::error::Error> {
    println!("=== Semantic Analysis Demo ===\n");

    // Step 1: Set up a registry with test commands
    let mut registry = CommandRegistry::new();

    // Math command for testing
    let math_command = CommandDefinition::former()
        .name("calculate")
        .namespace(".math".to_string())
        .description("Performs mathematical calculations".to_string())
        .hint("Calculator utility")
        .status("stable")
        .version("1.0.0")
        .aliases(vec!["calc".to_string()])
        .tags(vec!["math".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("GET".to_string())
        .examples(vec!["math.calculate --x 10 --y 5 --operation add".to_string()])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("x")
                .description("First number".to_string())
                .kind(Kind::Integer)
                .hint("First operand")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec!["min:-1000".to_string(), "max:1000".to_string()])
                .aliases(vec!["first".to_string()])
                .tags(vec!["numeric".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("y")
                .description("Second number".to_string())
                .kind(Kind::Integer)
                .hint("Second operand")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec!["min:-1000".to_string(), "max:1000".to_string()])
                .aliases(vec!["second".to_string()])
                .tags(vec!["numeric".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("operation")
                .description("Mathematical operation to perform".to_string())
                .kind(Kind::Enum(vec!["add".to_string(), "subtract".to_string(), "multiply".to_string(), "divide".to_string()]))
                .hint("Operation type")
                .default_value(Some("add".to_string()))
                .attributes(
                    ArgumentAttributes::former()
                        .optional(true)
                        .is_default_arg(true)
                        .end()
                )
                .validation_rules(vec![])
                .aliases(vec!["op".to_string(), "o".to_string()])
                .tags(vec!["operation".to_string()])
                .end(),
        ])
        .end();

    let math_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let x = cmd.arguments.get("x").and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let y = cmd.arguments.get("y").and_then(|v| if let Value::Integer(i) = v { Some(*i) } else { None }).unwrap_or(0);
        let op = cmd.arguments.get("operation").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None }).unwrap_or("add");

        let result = match op {
            "add" => x + y,
            "subtract" => x - y,
            "multiply" => x * y,
            "divide" => if y != 0 { x / y } else { 0 },
            _ => 0,
        };

        println!("Calculation: {} {} {} = {}", x, op, y, result);
        
        Ok(OutputData {
            content: result.to_string(),
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&math_command, math_routine)?;

    // Text processing command for testing
    let text_command = CommandDefinition::former()
        .name("process")
        .namespace(".text".to_string())
        .description("Processes text with various transformations".to_string())
        .hint("Text processing utility")
        .status("stable")
        .version("2.0.0")
        .aliases(vec!["transform".to_string()])
        .tags(vec!["text".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("POST".to_string())
        .examples(vec!["text.process 'hello world' --operations upper,reverse".to_string()])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("input")
                .description("Text to process".to_string())
                .kind(Kind::String)
                .hint("Input text")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec!["min_length:1".to_string()])
                .aliases(vec!["text".to_string(), "t".to_string()])
                .tags(vec!["input".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("operations")
                .description("List of operations to apply".to_string())
                .kind(Kind::List(Box::new(Kind::String), Some(',')))
                .hint("Comma-separated operations")
                .default_value(Some("none".to_string()))
                .attributes(
                    ArgumentAttributes::former()
                        .optional(true)
                        .is_default_arg(true)
                        .end()
                )
                .validation_rules(vec!["min_length:1".to_string()])
                .aliases(vec!["ops".to_string(), "o".to_string()])
                .tags(vec!["transformation".to_string()])
                .end(),
        ])
        .end();

    let text_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let input = cmd.arguments.get("input")
            .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
            .unwrap_or_default();

        let operations = cmd.arguments.get("operations")
            .and_then(|v| if let Value::List(list) = v { 
                Some(list.iter().filter_map(|item| 
                    if let Value::String(s) = item { Some(s.clone()) } else { None }
                ).collect::<Vec<_>>()) 
            } else { None })
            .unwrap_or_else(|| vec!["none".to_string()]);

        let mut result = input;
        for op in &operations {
            result = match op.as_str() {
                "upper" => result.to_uppercase(),
                "lower" => result.to_lowercase(),
                "reverse" => result.chars().rev().collect(),
                "trim" => result.trim().to_string(),
                _ => result,
            };
        }

        println!("Text processing: '{}' -> '{}'", input, result);
        println!("Operations applied: {:?}", operations);
        
        Ok(OutputData {
            content: result,
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&text_command, text_routine)?;

    println!("✓ Registered test commands for semantic analysis");

    // Step 2: Create sample parsed instructions manually (simulating parser output)
    let test_cases = vec![
        // Valid case 1: Math command with named arguments
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "math".to_string(), "calculate".to_string()],
            positional_arguments: vec![],
            named_arguments: {
                let mut map = HashMap::new();
                map.insert("x".to_string(), GenericArgument { value: "15".to_string() });
                map.insert("y".to_string(), GenericArgument { value: "3".to_string() });
                map.insert("operation".to_string(), GenericArgument { value: "multiply".to_string() });
                map
            },
        },

        // Valid case 2: Math command with positional arguments and alias
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "math".to_string(), "calculate".to_string()],
            positional_arguments: vec![
                GenericArgument { value: "20".to_string() },
                GenericArgument { value: "4".to_string() },
            ],
            named_arguments: {
                let mut map = HashMap::new();
                map.insert("op".to_string(), GenericArgument { value: "divide".to_string() }); // Using alias
                map
            },
        },

        // Valid case 3: Text command with default values
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "text".to_string(), "process".to_string()],
            positional_arguments: vec![
                GenericArgument { value: "Hello World".to_string() },
            ],
            named_arguments: HashMap::new(),
        },

        // Valid case 4: Text command with list argument
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "text".to_string(), "process".to_string()],
            positional_arguments: vec![
                GenericArgument { value: "Test String".to_string() },
            ],
            named_arguments: {
                let mut map = HashMap::new();
                map.insert("operations".to_string(), GenericArgument { value: "upper,reverse,trim".to_string() });
                map
            },
        },

        // Invalid case 1: Non-existent command
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "nonexistent".to_string(), "command".to_string()],
            positional_arguments: vec![],
            named_arguments: HashMap::new(),
        },

        // Invalid case 2: Missing required argument
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "math".to_string(), "calculate".to_string()],
            positional_arguments: vec![],
            named_arguments: {
                let mut map = HashMap::new();
                map.insert("x".to_string(), GenericArgument { value: "10".to_string() });
                // Missing 'y' argument
                map
            },
        },

        // Invalid case 3: Validation rule failure
        GenericInstruction {
            command_path_slices: vec!["".to_string(), "math".to_string(), "calculate".to_string()],
            positional_arguments: vec![],
            named_arguments: {
                let mut map = HashMap::new();
                map.insert("x".to_string(), GenericArgument { value: "2000".to_string() }); // Exceeds max:1000
                map.insert("y".to_string(), GenericArgument { value: "5".to_string() });
                map
            },
        },
    ];

    // Step 3: Perform semantic analysis on each test case
    println!("\n=== Semantic Analysis Test Cases ===");

    for (i, instruction) in test_cases.iter().enumerate() {
        println!("\n--- Test Case {} ---", i + 1);
        println!("Command: {}", instruction.command_path_slices.join("."));
        println!("Positional args: {:?}", instruction.positional_arguments.iter().map(|a| &a.value).collect::<Vec<_>>());
        println!("Named args: {:?}", instruction.named_arguments.iter().map(|(k, v)| (k, &v.value)).collect::<HashMap<_, _>>());

        let analyzer = SemanticAnalyzer::new(&[instruction.clone()], &registry);
        
        match analyzer.analyze() {
            Ok(verified_commands) => {
                println!("✅ Semantic analysis PASSED");
                for verified_cmd in &verified_commands {
                    println!("  Command: {} v{}", verified_cmd.definition.name, verified_cmd.definition.version);
                    println!("  Namespace: {}", verified_cmd.definition.namespace);
                    println!("  Verified arguments:");
                    for (name, value) in &verified_cmd.arguments {
                        println!("    {}: {:?}", name, value);
                    }
                }
            }
            Err(error) => {
                println!("❌ Semantic analysis FAILED");
                println!("  Error: {}", error);
            }
        }
    }

    // Step 4: Demonstrate the complete pipeline with actual parser
    println!("\n=== Complete Pipeline Demo ===");
    
    let parser = Parser::new(UnilangParserOptions::default());
    let test_commands = vec![
        "math.calculate --x 100 --y 25 --operation divide",
        "text.process 'semantic analysis demo' --operations upper,reverse",
        "calc 50 75", // Using alias and positional args
    ];

    for cmd_str in test_commands {
        println!("\n🔍 Analyzing: '{}'", cmd_str);
        
        match parser.parse_single_instruction(cmd_str) {
            Ok(instruction) => {
                println!("✓ Parsing successful");
                
                let analyzer = SemanticAnalyzer::new(&[instruction], &registry);
                match analyzer.analyze() {
                    Ok(verified_commands) => {
                        println!("✓ Semantic analysis successful");
                        
                        // Execute the verified command
                        for verified_cmd in verified_commands {
                            if let Some(routine) = registry.get_routine(&format!(".{}.{}", verified_cmd.definition.namespace.trim_start_matches('.'), verified_cmd.definition.name)) {
                                let context = unilang::interpreter::ExecutionContext::default();
                                match routine(verified_cmd, context) {
                                    Ok(output) => println!("✓ Execution successful: {}", output.content),
                                    Err(e) => println!("❌ Execution failed: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => println!("❌ Semantic analysis failed: {}", e),
                }
            }
            Err(e) => println!("❌ Parsing failed: {}", e),
        }
    }

    println!("\n=== Semantic Analysis Features ===");
    println!("🔍 The semantic analyzer performs:");
    println!("  • Command existence validation");
    println!("  • Argument binding (named → positional → defaults)");
    println!("  • Type checking and conversion");
    println!("  • Validation rule enforcement");
    println!("  • Alias resolution");
    println!("  • Required argument verification");
    println!("  • Argument count validation");
    println!("  • Creation of verified command objects");

    println!("\n=== Error Detection Capabilities ===");
    println!("❌ Common errors caught by semantic analysis:");
    println!("  • COMMAND_NOT_FOUND - Unknown commands");
    println!("  • MISSING_ARGUMENT - Required arguments not provided");
    println!("  • TOO_MANY_ARGUMENTS - Excess positional arguments");
    println!("  • VALIDATION_RULE_FAILED - Constraint violations");
    println!("  • TYPE_CONVERSION_ERROR - Invalid data types");

    println!("\n=== Best Practices ===");
    println!("💡 For robust semantic analysis:");
    println!("  • Define clear validation rules");
    println!("  • Provide meaningful error messages");
    println!("  • Use appropriate default values");
    println!("  • Implement comprehensive type checking");
    println!("  • Test edge cases and error conditions");
    println!("  • Document argument requirements clearly");

    Ok(())
}