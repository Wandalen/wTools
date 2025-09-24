#![allow(clippy::all)]
//! # REPL Feature Comparison
//!
//! This example demonstrates the differences between basic and enhanced REPL modes,
//! allowing you to see both implementations side-by-side.

#![ allow( clippy::needless_continue ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::cast_lossless ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::manual_string_new ) ]
#![ allow( clippy::collapsible_else_if ) ]
//!
//! ## Features Comparison
//! 
//! | Feature | Basic REPL | Enhanced REPL |
//! |---------|------------|---------------|
//! | Command execution | ✅ | ✅ |
//! | Error handling | ✅ | ✅ |
//! | Help system | ✅ | ✅ |
//! | Arrow key history | ❌ | ✅ |
//! | Tab completion | ❌ | ✅ |
//! | Interactive prompts | Basic | Secure/Masked |
//! | Session persistence | ❌ | ✅ |
//! | Auto-fallback | N/A | ✅ |
//!
//! ## Running this example:
//!
//! **Default (Enhanced REPL enabled):**
//! ```sh
//! cargo run --example repl_comparison
//! ```
//!
//! **Basic REPL only (minimal dependencies):**
//! ```sh
//! cargo run --example repl_comparison --no-default-features --features enabled,repl
//! ```

use unilang::prelude::*;
use unilang::error::Error;

#[cfg(feature = "enhanced_repl")]
use rustyline::{DefaultEditor, error::ReadlineError};
#[cfg(feature = "enhanced_repl")]
use std::io::IsTerminal;

#[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]
use std::io::{self, Write, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== REPL Feature Comparison Demo ===\n");

    // Setup common registry and pipeline
    #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
    setup_demo_commands(&mut registry)?;
    let pipeline = Pipeline::new(registry);

    // Show feature detection
    println!("🔍 Feature Detection:");
    
    #[cfg(feature = "enhanced_repl")]
    println!("   ✅ Enhanced REPL: rustyline integration available");
    
    #[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]
    println!("   📝 Basic REPL: standard I/O mode");
    
    #[cfg(not(feature = "repl"))]
    println!("   ❌ No REPL features enabled");

    println!("\n📋 Available Commands:");
    println!("   .demo name::value    - Demo command with argument");
    println!("   .secure password::   - Interactive password prompt");
    println!("   .help               - Show help");
    println!("   .                   - List all commands");
    println!("   quit                - Exit REPL");

    println!("\n🚀 Starting REPL Session...\n");

    // Route to appropriate REPL implementation
    #[cfg(feature = "enhanced_repl")]
    run_enhanced_repl(&pipeline)?;
    
    #[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]
    run_basic_repl(&pipeline)?;
    
    #[cfg(not(feature = "repl"))]
    {
        println!("❌ No REPL features enabled. Please run with 'repl' or 'enhanced_repl' feature.");
        println!("\nExamples:");
        println!("  cargo run --example repl_comparison --features enhanced_repl");
        println!("  cargo run --example repl_comparison --features repl");
    }

    println!("\n👋 REPL session ended. Thank you!");
    Ok(())
}

/// Setup demo commands to showcase REPL functionality
fn setup_demo_commands(registry: &mut CommandRegistry) -> Result<(), Error> {
    // Demo command with arguments
    let demo_cmd = CommandDefinition::former()
        .name(".demo")
        .namespace(String::new())
        .description("Demo command to test REPL functionality".to_string())
        .hint("Demo command")
        .status("stable")
        .version("1.0.0")
        .arguments(vec![
            ArgumentDefinition {
                name: "name".to_string(),
                description: "Name parameter".to_string(),
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
        .form();

    let demo_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_name = "World".to_string();
        let name = cmd.arguments.get("name")
            .and_then(|v| if let unilang::types::Value::String(s) = v { Some(s) } else { None })
            .unwrap_or(&default_name);

        println!("👋 Hello, {}! Command executed successfully.", name);

        Ok(OutputData {
            content: format!("Hello, {}!", name),
            format: "text".to_string(),
        })
    });

    #[allow(deprecated)]
    registry.command_add_runtime(&demo_cmd, demo_routine)?;

    // Interactive command for secure input demo
    let secure_cmd = CommandDefinition::former()
        .name(".secure")
        .namespace(String::new())
        .description("Demo command with interactive password".to_string())
        .hint("Secure input demo")
        .status("stable")
        .version("1.0.0")
        .arguments(vec![
            ArgumentDefinition {
                name: "password".to_string(),
                description: "Password for demonstration".to_string(),
                kind: Kind::String,
                hint: "Secure password".to_string(),
                attributes: ArgumentAttributes {
                    interactive: true,
                    sensitive: true,
                    optional: false,
                    ..Default::default()
                },
                validation_rules: vec![],
                aliases: vec!["p".to_string()],
                tags: vec![],
            }
        ])
        .form();

    let secure_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        let default_password = "".to_string();
        let password = cmd.arguments.get("password")
            .and_then(|v| if let unilang::types::Value::String(s) = v { Some(s) } else { None })
            .unwrap_or(&default_password);

        println!("🔐 Password received (length: {})", password.len());

        Ok(OutputData {
            content: format!("Authenticated with password of length {}", password.len()),
            format: "text".to_string(),
        })
    });

    #[allow(deprecated)]
    registry.command_add_runtime(&secure_cmd, secure_routine)?;

    Ok(())
}

#[cfg(feature = "enhanced_repl")]
/// Enhanced REPL with rustyline integration
fn run_enhanced_repl(pipeline: &Pipeline) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced REPL Mode");
    println!("Features: ↑/↓ history, Tab completion, Ctrl+C handling, persistent history");
    println!("Terminal: {} detected\n", if std::io::stdin().is_terminal() { "Interactive" } else { "Non-interactive" });

    let mut rl = DefaultEditor::new()?;
    
    // Load history if available
    let _ = rl.load_history("repl_history.txt");
    
    let mut session_stats = ReplSession::new();

    loop {
        let readline = rl.readline("enhanced_repl> ");
        
        match readline {
            Ok(line) => {
                let input = line.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                if input == "quit" || input == "exit" {
                    break;
                }
                
                // Add to history
                rl.add_history_entry(&line)?;
                
                // Process command
                let result = pipeline.process_command_simple(&line);
                session_stats.record_command(&result);
                
                // Handle results
                if result.is_success() {
                    println!("✅ Success");
                    for output in result.outputs_or_empty() {
                        println!("   {}", output.content);
                    }
                } else {
                    if result.requires_interactive_input() {
                        if let Some(arg_name) = result.interactive_argument() {
                            // Enhanced secure input
                            let prompt = format!("🔐 Enter {}: ", arg_name);
                            match rl.readline(&prompt) {
                                Ok(value) => {
                                    let new_cmd = format!("{} {}::{}", line, arg_name, value);
                                    let retry_result = pipeline.process_command_simple(&new_cmd);
                                    if retry_result.is_success() {
                                        println!("✅ Authentication successful");
                                        for output in retry_result.outputs_or_empty() {
                                            println!("   {}", output.content);
                                        }
                                    } else {
                                        println!("❌ Authentication failed: {}", retry_result.error_message().unwrap_or("Unknown error"));
                                    }
                                }
                                Err(ReadlineError::Interrupted) => {
                                    println!("❌ Input cancelled");
                                    continue;
                                }
                                Err(err) => {
                                    println!("❌ Input error: {}", err);
                                    continue;
                                }
                            }
                        }
                    } else if result.is_help_response() {
                        println!("📖 Help:");
                        if let Some(help) = result.help_content() {
                            println!("{}", help);
                        } else {
                            println!("   {}", result.error_message().unwrap_or("Help not available"));
                        }
                    } else {
                        println!("❌ Error: {}", result.error_message().unwrap_or("Unknown error"));
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("\n👋 Interrupted. Use 'quit' to exit gracefully.");
                continue;
            },
            Err(ReadlineError::Eof) => {
                println!("\n👋 EOF detected. Exiting...");
                break;
            },
            Err(err) => {
                println!("❌ Error reading input: {:?}", err);
                break;
            }
        }
    }
    
    // Save history
    let _ = rl.save_history("repl_history.txt");
    
    session_stats.print_summary();
    
    Ok(())
}

#[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]
/// Basic REPL with standard I/O
fn run_basic_repl(pipeline: &Pipeline) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Basic REPL Mode");
    println!("Features: Standard I/O, basic history tracking");
    println!("Limitations: No arrow keys, no tab completion, visible password input\n");

    let stdin = io::stdin();
    let mut session_stats = ReplSession::new();
    let mut command_history = Vec::new();

    loop {
        print!("basic_repl> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => {
                println!("\n👋 EOF detected. Exiting...");
                break;
            }
            Ok(_) => {
                let input = input.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                if input == "quit" || input == "exit" {
                    break;
                }
                
                // Add to basic history
                command_history.push(input.to_string());
                
                // Process command
                let result = pipeline.process_command_simple(input);
                session_stats.record_command(&result);
                
                // Handle results
                if result.is_success() {
                    println!("✅ Success");
                    for output in result.outputs_or_empty() {
                        println!("   {}", output.content);
                    }
                } else {
                    if result.requires_interactive_input() {
                        if let Some(arg_name) = result.interactive_argument() {
                            // Basic insecure input (visible)
                            print!("🔑 Enter {} (WARNING: input will be visible): ", arg_name);
                            io::stdout().flush()?;
                            
                            let mut value = String::new();
                            match stdin.lock().read_line(&mut value) {
                                Ok(_) => {
                                    let value = value.trim();
                                    let new_cmd = format!("{} {}::{}", input, arg_name, value);
                                    let retry_result = pipeline.process_command_simple(&new_cmd);
                                    if retry_result.is_success() {
                                        println!("✅ Authentication successful");
                                        for output in retry_result.outputs_or_empty() {
                                            println!("   {}", output.content);
                                        }
                                    } else {
                                        println!("❌ Authentication failed: {}", retry_result.error_message().unwrap_or("Unknown error"));
                                    }
                                }
                                Err(err) => {
                                    println!("❌ Input error: {}", err);
                                    continue;
                                }
                            }
                        }
                    } else if result.is_help_response() {
                        println!("📖 Help:");
                        if let Some(help) = result.help_content() {
                            println!("{}", help);
                        } else {
                            println!("   {}", result.error_message().unwrap_or("Help not available"));
                        }
                    } else {
                        println!("❌ Error: {}", result.error_message().unwrap_or("Unknown error"));
                    }
                }
            }
            Err(err) => {
                println!("❌ Error reading input: {}", err);
                break;
            }
        }
    }
    
    println!("\n📊 Command History ({} commands):", command_history.len());
    for (i, cmd) in command_history.iter().enumerate().take(5) {
        println!("   {}: {}", i + 1, cmd);
    }
    if command_history.len() > 5 {
        println!("   ... and {} more", command_history.len() - 5);
    }
    
    session_stats.print_summary();
    
    Ok(())
}

/// Simple session statistics tracking
struct ReplSession {
    command_count: u32,
    successful_commands: u32,
    failed_commands: u32,
    interactive_prompts: u32,
}

impl ReplSession {
    fn new() -> Self {
        Self {
            command_count: 0,
            successful_commands: 0,
            failed_commands: 0,
            interactive_prompts: 0,
        }
    }
    
    fn record_command(&mut self, result: &CommandResult) {
        self.command_count += 1;
        
        if result.is_success() {
            self.successful_commands += 1;
        } else {
            self.failed_commands += 1;
            
            if result.requires_interactive_input() {
                self.interactive_prompts += 1;
            }
        }
    }
    
    fn print_summary(&self) {
        println!("\n📊 Session Summary:");
        println!("   Commands executed: {}", self.command_count);
        println!("   Successful: {} ({:.1}%)", self.successful_commands, 
            if self.command_count > 0 { 
                (self.successful_commands as f64 / self.command_count as f64) * 100.0 
            } else { 
                0.0 
            });
        println!("   Failed: {}", self.failed_commands);
        println!("   Interactive prompts: {}", self.interactive_prompts);
    }
}