//! Example 20: Rust DSL → Dynamic `HashMap` with Inline Closures (Row 7)
//!
//! This example demonstrates the Row 7 approach: using Rust DSL with inline
//! closures for maximum development flexibility. This approach prioritizes
//! rapid development and prototyping over raw performance.
//!
//! **Trade-offs:**
//! - ✅ Fastest development time (write closures inline)
//! - ✅ Maximum flexibility (closures can capture variables)
//! - ✅ Perfect for prototyping and rapid iteration
//! - ⚠️ Runtime lookup cost (~4,200ns per command via `HashMap`)
//! - ⚠️ No compile-time validation
//!
//! **When to use:**
//! - Prototyping new features
//! - Small CLI tools (<100 commands)
//! - Applications where development speed > performance
//! - Dynamic command registration scenarios

#![ allow( clippy::too_many_lines ) ]

use unilang::
{
  registry::CommandRegistry,
  data::OutputData,
};

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Row 7: Inline Closures Example ===" );
  println!();

  // Example 1: Basic inline closure registration
  println!( "1. Basic inline closure:" );

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".greet",
      "Greets the user by name",
      |cmd, _ctx| {
        // Access command arguments
        let name = cmd.arguments.get( "name" )
          .and_then( |v| {
            if let unilang::types::Value::String( s ) = v {
              Some( s.as_str() )
            } else {
              None
            }
          })
          .unwrap_or( "World" );

        Ok( OutputData {
          content: format!( "Hello, {name}!" ),
          format: "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  println!( "  Registered command: .greet" );
  println!( "  Description: {}", registry.command( ".greet" ).unwrap().description() );
  println!();

  // Example 2: Closure capturing external variables
  println!( "2. Closure with variable capture:" );

  let app_version = "1.2.3".to_string();
  let build_date = "2025-01-15".to_string();

  let _registry_with_capture = CommandRegistry::builder()
    .command_with_routine(
      ".version",
      "Shows application version",
      move |_cmd, _ctx| {
        Ok( OutputData {
          content: format!( "Version: {app_version}\nBuild: {build_date}" ),
          format: "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  println!( "  Registered command: .version" );
  println!( "  Captures: app_version, build_date" );
  println!();

  // Example 3: Multiple inline closures
  println!( "3. Multiple inline closures in fluent chain:" );

  let registry_multi = CommandRegistry::builder()
    .command_with_routine(
      ".add",
      "Adds two numbers",
      |_cmd, _ctx| {
        // In a real app, you'd parse arguments properly
        Ok( OutputData {
          content: "42".to_string(),
          format: "number".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".subtract",
      "Subtracts two numbers",
      |_cmd, _ctx| {
        Ok( OutputData {
          content: "10".to_string(),
          format: "number".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".multiply",
      "Multiplies two numbers",
      |_cmd, _ctx| {
        Ok( OutputData {
          content: "100".to_string(),
          format: "number".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  println!( "  Registered {} commands:", registry_multi.commands().len() );
  for (name, _) in registry_multi.commands() {
    println!( "    - {name}" );
  }
  println!();

  // Example 4: Mixing YAML and inline closures
  println!( "4. Mixing YAML definitions with inline closures:" );

  let yaml_commands = r#"
- name: ".config"
  namespace: ""
  description: "Shows configuration"
  hint: "Display current config"
  status: "stable"
  version: "1.0.0"
  arguments: []
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
  auto_help_enabled: true
"#;

  let registry_mixed = CommandRegistry::builder()
    .load_from_yaml_str( yaml_commands )?
    .command_with_routine(
      ".runtime",
      "Shows runtime info",
      |_cmd, _ctx| {
        Ok( OutputData {
          content: "Runtime: 42.5ms".to_string(),
          format: "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  println!( "  YAML command: .config (no routine)" );
  println!( "  Inline command: .runtime (with routine)" );
  println!( "  Has routine for .config: {}", registry_mixed.get_routine( ".config" ).is_some() );
  println!( "  Has routine for .runtime: {}", registry_mixed.get_routine( ".runtime" ).is_some() );
  println!();

  // Example 5: Auto-help generation
  println!( "5. Automatic help command generation:" );

  let registry_help = CommandRegistry::builder()
    .command_with_routine(
      ".deploy",
      "Deploys the application",
      |_cmd, _ctx| {
        Ok( OutputData {
          content: "Deploying...".to_string(),
          format: "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  println!( "  Base command: .deploy" );
  println!( "  Auto-generated: .deploy.help" );
  println!( "  Help exists: {}", registry_help.command( ".deploy.help" ).is_some() );
  println!();

  // Performance note
  println!( "⚠️  Performance Characteristics:" );
  println!( "  - Lookup time: ~4,200ns per command (HashMap)" );
  println!( "  - Registration: Runtime only" );
  println!( "  - Binary size: Minimal overhead" );
  println!( "  - Trade-off: Flexibility > Performance" );
  println!();

  println!( "✅ Row 7 example complete!" );
  println!();
  println!( "Next: See example 21 for Row 8 (Compile-Time Static with const fn)" );

  Ok(())
}
