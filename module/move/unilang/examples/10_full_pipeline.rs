//! # Full Pipeline Demo
//!
//! This example demonstrates the complete Unilang pipeline from command
//! registration through parsing, semantic analysis, and execution,
//! showing how all components work together.

use std::collections::HashMap;
use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes, OutputData, ErrorData};
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::interpreter::{Interpreter, ExecutionContext};
use unilang::help::HelpGenerator;
use unilang::types::Value;
use unilang_parser::{Parser, UnilangParserOptions};

fn main() -> Result<(), unilang::error::Error> {
    println!("=== Full Unilang Pipeline Demo ===\n");

    // ========================================
    // PHASE 1: COMMAND REGISTRY SETUP
    // ========================================
    println!("🏗️  PHASE 1: Setting up Command Registry");
    println!("==========================================");

    let mut registry = CommandRegistry::new();

    // Create a comprehensive file management system
    setup_file_commands(&mut registry)?;
    setup_text_commands(&mut registry)?;
    setup_network_commands(&mut registry)?;
    setup_utility_commands(&mut registry)?;

    println!("✅ Registry setup complete with {} commands", registry.commands.len());

    // ========================================
    // PHASE 2: HELP SYSTEM DEMONSTRATION
    // ========================================
    println!("\n📚 PHASE 2: Help System");
    println!("========================");

    let help_generator = HelpGenerator::new(&registry);
    println!("{}", help_generator.list_commands());

    // Show detailed help for a complex command
    if let Some(detailed_help) = help_generator.command("file.sync") {
        println!("\n--- Detailed Help Example ---");
        println!("{}", detailed_help);
    }

    // ========================================
    // PHASE 3: INTERACTIVE COMMAND PROCESSING
    // ========================================
    println!("\n🔄 PHASE 3: Interactive Command Processing");
    println!("===========================================");

    let parser = Parser::new(UnilangParserOptions::default());

    // Simulate a series of user commands
    let user_commands = vec![
        "help",
        "util.echo 'Starting file operations...'",
        "file.list /tmp --format table",
        "text.analyze 'The quick brown fox jumps over the lazy dog' --metrics words,chars,vowels",
        "file.sync --source ./src --target ./backup --dry-run --exclude '*.tmp|*.log'",
        "network.ping google.com --count 3 --timeout 5000",
        "util.timestamp --format iso",
        "invalid.command", // This should fail
        "file.list", // Missing required argument
        "text.analyze", // Missing required argument
    ];

    for (i, command_str) in user_commands.iter().enumerate() {
        println!("\n--- Command {} ---", i + 1);
        println!("User input: '{}'", command_str);

        // Handle help command specially
        if command_str == &"help" {
            println!("📋 Showing help:");
            println!("{}", help_generator.list_commands());
            continue;
        }

        // Process the command through the full pipeline
        process_command(command_str, &parser, &registry)?;
    }

    // ========================================
    // PHASE 4: BATCH PROCESSING
    // ========================================
    println!("\n📦 PHASE 4: Batch Processing");
    println!("=============================");

    let batch_script = vec![
        "util.echo 'Batch processing started'",
        "util.timestamp --format unix",
        "text.analyze 'Batch processing example' --metrics all",
        "file.list . --format json",
        "util.echo 'Batch processing completed'",
    ];

    println!("Processing batch of {} commands:", batch_script.len());

    let mut all_instructions = Vec::new();
    for cmd_str in &batch_script {
        match parser.parse_single_instruction(cmd_str) {
            Ok(instruction) => {
                println!("✓ Parsed: {}", cmd_str);
                all_instructions.push(instruction);
            }
            Err(e) => {
                println!("❌ Parse failed for '{}': {}", cmd_str, e);
            }
        }
    }

    if !all_instructions.is_empty() {
        let analyzer = SemanticAnalyzer::new(&all_instructions, &registry);

        match analyzer.analyze() {
            Ok(verified_commands) => {
                println!("✓ Semantic analysis passed for {} commands", verified_commands.len());

                let interpreter = Interpreter::new(&verified_commands, &registry);
                let mut context = ExecutionContext::default();

                match interpreter.run(&mut context) {
                    Ok(outputs) => {
                        println!("✅ Batch execution completed successfully");
                        println!("Generated {} outputs", outputs.len());
                    }
                    Err(error) => {
                        println!("❌ Batch execution failed: {}", error);
                    }
                }
            }
            Err(error) => {
                println!("❌ Batch semantic analysis failed: {}", error);
            }
        }
    }

    // ========================================
    // PHASE 5: PIPELINE SUMMARY
    // ========================================
    println!("\n📊 PHASE 5: Pipeline Summary");
    println!("=============================");

    println!("🎯 Unilang Pipeline Components:");
    println!("  1. 📝 Command Definition - Declarative command specs");
    println!("  2. 🏪 Registry Management - Centralized command storage");
    println!("  3. 📄 External Loading - YAML/JSON command definitions");
    println!("  4. 🔍 Parsing - Text to structured instructions");
    println!("  5. 🧠 Semantic Analysis - Validation and verification");
    println!("  6. ⚡ Execution - Command routine invocation");
    println!("  7. 📚 Help Generation - Automatic documentation");
    println!("  8. 🛡️  Error Handling - Comprehensive error management");

    println!("\n✨ Key Features Demonstrated:");
    println!("  • Multiple argument types and validation");
    println!("  • Namespace organization and aliases");
    println!("  • Collection types (Lists, Maps) with custom delimiters");
    println!("  • Default values and optional arguments");
    println!("  • Complex validation rules");
    println!("  • Structured error reporting");
    println!("  • Batch command processing");
    println!("  • Interactive help system");
    println!("  • Type-safe argument binding");
    println!("  • Execution context management");

    println!("\n🏁 Pipeline demo completed successfully!");

    Ok(())
}

/// Process a single command through the complete pipeline
fn process_command(
    command_str: &str,
    parser: &Parser,
    registry: &CommandRegistry,
) -> Result<(), unilang::error::Error> {

    // Step 1: Parsing
    match parser.parse_single_instruction(command_str) {
        Ok(instruction) => {
            println!("✓ Parsing successful");

            // Step 2: Semantic Analysis
            let instructions = [instruction];
            let analyzer = SemanticAnalyzer::new(&instructions, registry);
            match analyzer.analyze() {
                Ok(verified_commands) => {
                    println!("✓ Semantic analysis successful");

                    // Step 3: Execution
                    let interpreter = Interpreter::new(&verified_commands, registry);
                    let mut context = ExecutionContext::default();

                    match interpreter.run(&mut context) {
                        Ok(outputs) => {
                            println!("✅ Execution successful");
                            for output in outputs {
                                if !output.content.is_empty() {
                                    println!("📤 Output: {}", output.content);
                                }
                            }
                        }
                        Err(error) => {
                            println!("❌ Execution failed: {}", error);
                        }
                    }
                }
                Err(error) => {
                    println!("❌ Semantic analysis failed: {}", error);
                }
            }
        }
        Err(error) => {
            println!("❌ Parsing failed: {}", error);
        }
    }

    Ok(())
}

/// Set up file management commands
fn setup_file_commands(registry: &mut CommandRegistry) -> Result<(), unilang::error::Error> {
    // File listing command
    let list_command = CommandDefinition::former()
        .name("list")
        .namespace(".file".to_string())
        .description("Lists files and directories with various formatting options".to_string())
        .hint("Directory listing utility")
        .status("stable")
        .version("2.3.1")
        .aliases(vec!["ls".to_string(), "dir".to_string()])
        .tags(vec!["filesystem".to_string(), "utility".to_string()])
        .permissions(vec!["read_directory".to_string()])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("GET".to_string())
        .examples(vec![
            "file.list /home/user --format table".to_string(),
            "ls . --format json".to_string()
        ])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("path")
                .description("Directory path to list".to_string())
                .kind(Kind::Directory)
                .hint("Target directory")
                .default_value(".".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec![])
                .aliases(vec!["p".to_string(), "dir".to_string()])
                .tags(vec!["filesystem".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("format")
                .description("Output format".to_string())
                .kind(Kind::Enum(vec!["table".to_string(), "list".to_string(), "json".to_string()]))
                .hint("Display format")
                .default_value("list".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec![])
                .aliases(vec!["f".to_string()])
                .tags(vec!["formatting".to_string()])
                .end(),
        ])
        .end();

    let list_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_path = ".".to_string();
        let path = cmd.arguments.get("path").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_path);
        let default_format = "list".to_string();
        let format = cmd.arguments.get("format").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_format);

        println!("📁 Listing directory: {} (format: {})", path, format);

        match std::fs::read_dir(path) {
            Ok(entries) => {
                let files: Vec<String> = entries.filter_map(|e|
                    e.ok().and_then(|entry| entry.file_name().to_str().map(|s| s.to_string()))
                ).collect();

                match format.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(&files).unwrap_or_default()),
                    "table" => {
                        println!("┌─────────────────────────────┐");
                        println!("│          Files              │");
                        println!("├─────────────────────────────┤");
                        for file in &files {
                            println!("│ {:<27} │", file);
                        }
                        println!("└─────────────────────────────┘");
                    },
                    _ => {
                        for file in &files {
                            println!("  {}", file);
                        }
                    }
                }

                Ok(OutputData {
                    content: files.join("\n"),
                    format: format.clone(),
                })
            }
            Err(e) => Err(ErrorData {
                code: "DIRECTORY_READ_ERROR".to_string(),
                message: format!("Cannot read directory '{}': {}", path, e),
            })
        }
    });

    registry.command_add_runtime(&list_command, list_routine)?;

    // File sync command
    let sync_command = CommandDefinition::former()
        .name("sync")
        .namespace(".file".to_string())
        .description("Synchronizes files between source and target directories".to_string())
        .hint("File synchronization utility")
        .status("beta")
        .version("1.8.0")
        .aliases(vec!["backup".to_string(), "mirror".to_string()])
        .tags(vec!["filesystem".to_string(), "backup".to_string(), "sync".to_string()])
        .permissions(vec!["read_file".to_string(), "write_file".to_string()])
        .idempotent(false)
        .deprecation_message("".to_string())
        .http_method_hint("POST".to_string())
        .examples(vec![
            "file.sync --source ./docs --target ./backup/docs --dry-run".to_string(),
            "sync -s /home/user -t /backup --exclude '*.tmp|*.log'".to_string()
        ])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("source")
                .description("Source directory to sync from".to_string())
                .kind(Kind::Directory)
                .hint("Source directory path")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec![])
                .aliases(vec!["s".to_string(), "src".to_string()])
                .tags(vec!["required".to_string(), "input".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("target")
                .description("Target directory to sync to".to_string())
                .kind(Kind::Directory)
                .hint("Target directory path")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec![])
                .aliases(vec!["t".to_string(), "dest".to_string()])
                .tags(vec!["required".to_string(), "output".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("dry_run")
                .description("Show what would be done without making changes".to_string())
                .kind(Kind::Boolean)
                .hint("Simulation mode")
                .default_value("false".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec![])
                .aliases(vec!["dry".to_string(), "simulate".to_string()])
                .tags(vec!["safety".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("exclude")
                .description("Patterns to exclude from sync".to_string())
                .kind(Kind::List(Box::new(Kind::String), Some('|')))
                .hint("Pipe-separated exclusion patterns")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![])
                .aliases(vec!["x".to_string(), "ignore".to_string()])
                .tags(vec!["filtering".to_string()])
                .end(),
        ])
        .end();

    let sync_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_source = "".to_string();
        let default_target = "".to_string();
        let source = cmd.arguments.get("source").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_source);
        let target = cmd.arguments.get("target").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_target);
        let dry_run = cmd.arguments.get("dry_run").and_then(|v| if let Value::Boolean(b) = v { Some(b) } else { None }).unwrap_or(&false);

        let exclude_patterns = cmd.arguments.get("exclude")
            .and_then(|v| if let Value::List(list) = v {
                Some(list.iter().filter_map(|item|
                    if let Value::String(s) = item { Some(s.clone()) } else { None }
                ).collect::<Vec<_>>())
            } else { None })
            .unwrap_or_default();

        println!("🔄 File Sync Operation");
        println!("Source: {}", source);
        println!("Target: {}", target);
        println!("Dry Run: {}", if *dry_run { "Yes" } else { "No" });
        if !exclude_patterns.is_empty() {
            println!("Exclusions: {:?}", exclude_patterns);
        }

        if *dry_run {
            println!("📋 DRY RUN - No files will be modified");
            println!("  • Would copy files from {} to {}", source, target);
            println!("  • Would exclude patterns: {:?}", exclude_patterns);
        } else {
            println!("✨ Sync operation would execute here");
        }

        Ok(OutputData {
            content: format!("Sync from {} to {} completed", source, target),
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&sync_command, sync_routine)?;

    Ok(())
}

/// Set up text processing commands
fn setup_text_commands(registry: &mut CommandRegistry) -> Result<(), unilang::error::Error> {
    let analyze_command = CommandDefinition::former()
        .name("analyze")
        .namespace(".text".to_string())
        .description("Analyzes text with various metrics and statistics".to_string())
        .hint("Text analysis and metrics")
        .status("stable")
        .version("3.1.2")
        .aliases(vec!["stats".to_string(), "metrics".to_string()])
        .tags(vec!["text".to_string(), "analysis".to_string(), "nlp".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("POST".to_string())
        .examples(vec![
            "text.analyze 'Hello world' --metrics words,chars".to_string(),
            "stats 'The quick brown fox' --metrics all".to_string()
        ])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("text")
                .description("Text to analyze".to_string())
                .kind(Kind::String)
                .hint("Input text string")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec!["min_length:1".to_string()])
                .aliases(vec!["input".to_string(), "content".to_string()])
                .tags(vec!["required".to_string(), "input".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("metrics")
                .description("Metrics to calculate".to_string())
                .kind(Kind::List(Box::new(Kind::String), Some(',')))
                .hint("Comma-separated metric names")
                .default_value("words,chars".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec!["min_length:1".to_string()])
                .aliases(vec!["m".to_string(), "stats".to_string()])
                .tags(vec!["configuration".to_string()])
                .end(),
        ])
        .end();

    let analyze_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_text = "".to_string();
        let text = cmd.arguments.get("text").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_text);

        let metrics = cmd.arguments.get("metrics")
            .and_then(|v| if let Value::List(list) = v {
                Some(list.iter().filter_map(|item|
                    if let Value::String(s) = item { Some(s.clone()) } else { None }
                ).collect::<Vec<_>>())
            } else { None })
            .unwrap_or_else(|| vec!["words".to_string(), "chars".to_string()]);

        println!("📊 Text Analysis Results");
        println!("Text: '{}'", text);
        println!("Metrics: {:?}", metrics);
        println!("─────────────────────");

        let mut results = HashMap::new();

        for metric in &metrics {
            match metric.as_str() {
                "all" => {
                    let word_count = text.split_whitespace().count();
                    results.insert("words".to_string(), word_count.to_string());
                    println!("Words: {}", word_count);
                    
                    let char_count = text.chars().count();
                    results.insert("chars".to_string(), char_count.to_string());
                    println!("Characters: {}", char_count);
                    
                    let vowel_count = text.chars().filter(|c| "aeiouAEIOU".contains(*c)).count();
                    results.insert("vowels".to_string(), vowel_count.to_string());
                    println!("Vowels: {}", vowel_count);
                    
                    let sentence_count = text.matches(['.', '!', '?']).count();
                    results.insert("sentences".to_string(), sentence_count.to_string());
                    println!("Sentences: {}", sentence_count);
                }
                "words" => {
                    let word_count = text.split_whitespace().count();
                    results.insert("words".to_string(), word_count.to_string());
                    println!("Words: {}", word_count);
                }
                "chars" => {
                    let char_count = text.chars().count();
                    results.insert("chars".to_string(), char_count.to_string());
                    println!("Characters: {}", char_count);
                }
                "vowels" => {
                    let vowel_count = text.chars().filter(|c| "aeiouAEIOU".contains(*c)).count();
                    results.insert("vowels".to_string(), vowel_count.to_string());
                    println!("Vowels: {}", vowel_count);
                }
                "sentences" => {
                    let sentence_count = text.matches(['.', '!', '?']).count();
                    results.insert("sentences".to_string(), sentence_count.to_string());
                    println!("Sentences: {}", sentence_count);
                }
                _ => {}
            }
        }

        let result_json = serde_json::to_string(&results).unwrap_or_default();

        Ok(OutputData {
            content: result_json,
            format: "json".to_string(),
        })
    });

    registry.command_add_runtime(&analyze_command, analyze_routine)?;

    Ok(())
}

/// Set up network commands
fn setup_network_commands(registry: &mut CommandRegistry) -> Result<(), unilang::error::Error> {
    let ping_command = CommandDefinition::former()
        .name("ping")
        .namespace(".network".to_string())
        .description("Tests network connectivity to a host".to_string())
        .hint("Network connectivity test")
        .status("stable")
        .version("2.0.1")
        .aliases(vec!["test".to_string(), "check".to_string()])
        .tags(vec!["network".to_string(), "connectivity".to_string(), "diagnostic".to_string()])
        .permissions(vec!["network_access".to_string()])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("GET".to_string())
        .examples(vec![
            "network.ping google.com --count 4".to_string(),
            "ping 8.8.8.8 --timeout 3000".to_string()
        ])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("host")
                .description("Host to ping (hostname or IP address)".to_string())
                .kind(Kind::String)
                .hint("Target host")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec!["min_length:1".to_string()])
                .aliases(vec!["target".to_string(), "address".to_string()])
                .tags(vec!["required".to_string(), "network".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("count")
                .description("Number of ping packets to send".to_string())
                .kind(Kind::Integer)
                .hint("Packet count")
                .default_value("4".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec!["min:1".to_string(), "max:100".to_string()])
                .aliases(vec!["c".to_string(), "packets".to_string()])
                .tags(vec!["configuration".to_string()])
                .end(),
            ArgumentDefinition::former()
                .name("timeout")
                .description("Timeout in milliseconds".to_string())
                .kind(Kind::Integer)
                .hint("Timeout (ms)")
                .default_value("5000".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec!["min:100".to_string(), "max:60000".to_string()])
                .aliases(vec!["t".to_string(), "wait".to_string()])
                .tags(vec!["configuration".to_string()])
                .end(),
        ])
        .end();

    let ping_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_host = "localhost".to_string();
        let host = cmd.arguments.get("host").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_host);
        let count = cmd.arguments.get("count").and_then(|v| if let Value::Integer(i) = v { Some(i) } else { None }).unwrap_or(&4);
        let timeout = cmd.arguments.get("timeout").and_then(|v| if let Value::Integer(i) = v { Some(i) } else { None }).unwrap_or(&5000);

        println!("🌐 Ping Test Results");
        println!("Target: {}", host);
        println!("Packets: {}, Timeout: {}ms", count, timeout);
        println!("─────────────────────");

        // Simulate ping results
        for i in 1..=*count {
            let response_time = 20 + (i * 3); // Simulated response time
            println!("Ping {}: Reply from {} time={}ms", i, host, response_time);
        }

        let summary = format!("Sent {} packets to {}, simulated successful pings", count, host);
        println!("\n✅ {}", summary);

        Ok(OutputData {
            content: summary,
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&ping_command, ping_routine)?;

    Ok(())
}

/// Set up utility commands
fn setup_utility_commands(registry: &mut CommandRegistry) -> Result<(), unilang::error::Error> {
    // Echo command
    let echo_command = CommandDefinition::former()
        .name("echo")
        .namespace(".util".to_string())
        .description("Prints text to output".to_string())
        .hint("Text output utility")
        .status("stable")
        .version("1.0.0")
        .aliases(vec!["print".to_string(), "say".to_string()])
        .tags(vec!["utility".to_string(), "output".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("GET".to_string())
        .examples(vec!["util.echo 'Hello, World!'".to_string()])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("message")
                .description("Message to print".to_string())
                .kind(Kind::String)
                .hint("Text message")
                .attributes(ArgumentAttributes::former().optional(false).end())
                .validation_rules(vec![])
                .aliases(vec!["text".to_string(), "msg".to_string()])
                .tags(vec!["required".to_string()])
                .end(),
        ])
        .end();

    let echo_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_message = "".to_string();
        let message = cmd.arguments.get("message").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_message);
        println!("🔊 {}", message);

        Ok(OutputData {
            content: message.clone(),
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&echo_command, echo_routine)?;

    // Timestamp command
    let timestamp_command = CommandDefinition::former()
        .name("timestamp")
        .namespace(".util".to_string())
        .description("Shows current timestamp in various formats".to_string())
        .hint("Timestamp utility")
        .status("stable")
        .version("1.1.0")
        .aliases(vec!["time".to_string(), "now".to_string()])
        .tags(vec!["utility".to_string(), "time".to_string()])
        .permissions(vec![])
        .idempotent(false) // Time changes
        .deprecation_message("".to_string())
        .http_method_hint("GET".to_string())
        .examples(vec![
            "util.timestamp --format iso".to_string(),
            "time --format unix".to_string()
        ])
        .arguments(vec![
            ArgumentDefinition::former()
                .name("format")
                .description("Timestamp format".to_string())
                .kind(Kind::Enum(vec!["iso".to_string(), "unix".to_string(), "human".to_string()]))
                .hint("Output format")
                .default_value("human".to_string())
                .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
                .validation_rules(vec![])
                .aliases(vec!["f".to_string(), "fmt".to_string()])
                .tags(vec!["formatting".to_string()])
                .end(),
        ])
        .end();

    let timestamp_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_format = "human".to_string();
        let format = cmd.arguments.get("format").and_then(|v| if let Value::String(s) = v { Some(s) } else { None }).unwrap_or(&default_format);

        let now = std::time::SystemTime::now();
        let timestamp = match format.as_str() {
            "unix" => {
                let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                duration.as_secs().to_string()
            }
            "iso" => {
                // Simplified ISO format simulation
                "2024-01-15T10:30:45Z".to_string()
            }
            _ => {
                // Human readable format
                "Monday, January 15, 2024 at 10:30:45 AM".to_string()
            }
        };

        println!("🕐 Current time ({}): {}", format, timestamp);

        Ok(OutputData {
            content: timestamp,
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&timestamp_command, timestamp_routine)?;

    Ok(())
}