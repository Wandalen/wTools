//! # Validation Rules Demo
//! 
//! This example demonstrates how to apply validation rules to command arguments,
//! including min/max values, string patterns, and length constraints.

use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes, OutputData};
use unilang::registry::CommandRegistry;
use unilang::types::Value;

fn main() -> Result<(), unilang::error::Error> {
    println!("=== Validation Rules Demo ===\n");

    let mut registry = CommandRegistry::new();

    // Command demonstrating various validation rules
    let validation_demo = CommandDefinition::former()
        .name("validate")
        .namespace("validation".to_string())
        .description("Demonstrates argument validation rules".to_string())
        .hint("Shows different validation constraints")
        .status("stable")
        .version("1.0.0")
        .aliases(vec!["check".to_string()])
        .tags(vec!["validation".to_string(), "demo".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("POST".to_string())
        .examples(vec![
            "validation.validate age::25 name::Alice email::alice@example.com".to_string(),
            "validation.validate score::85.5 password::secretkey123".to_string(),
        ])
        .arguments(vec![
            // Numeric validation with min/max
            ArgumentDefinition::former()
                .name("age")
                .description("Person's age (must be 0-120)".to_string())
                .kind(Kind::Integer)
                .hint("Age in years")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![
                    "min:0".to_string(),
                    "max:120".to_string()
                ])
                .aliases(vec!["a".to_string()])
                .tags(vec!["personal".to_string()])
                .end(),

            // Float validation with min constraint
            ArgumentDefinition::former()
                .name("score")
                .description("Test score (must be 0.0 or higher)".to_string())
                .kind(Kind::Float)
                .hint("Score as decimal")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![
                    "min:0.0".to_string(),
                    "max:100.0".to_string()
                ])
                .aliases(vec!["s".to_string()])
                .tags(vec!["academic".to_string()])
                .end(),

            // String length validation
            ArgumentDefinition::former()
                .name("name")
                .description("Person's name (2-50 characters)".to_string())
                .kind(Kind::String)
                .hint("Full name")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![
                    "min_length:2".to_string(),
                    "max_length:50".to_string() // Note: max_length not implemented in the semantic analyzer, but shown for completeness
                ])
                .aliases(vec!["n".to_string()])
                .tags(vec!["personal".to_string()])
                .end(),

            // Regex pattern validation
            ArgumentDefinition::former()
                .name("email")
                .description("Email address (must match email pattern)".to_string())
                .kind(Kind::String)
                .hint("Valid email format")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![
                    "regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$".to_string()
                ])
                .aliases(vec!["e".to_string()])
                .tags(vec!["contact".to_string()])
                .end(),

            // Multiple validation rules
            ArgumentDefinition::former()
                .name("password")
                .description("Password (8+ chars, must contain letters and numbers)".to_string())
                .kind(Kind::String)
                .hint("Secure password")
                .attributes(
                    ArgumentAttributes::former()
                        .optional(true)
                        .sensitive(true) // Mark as sensitive
                        .end()
                )
                .validation_rules(vec![
                    "min_length:8".to_string(),
                    "regex:^(?=.*[A-Za-z])(?=.*\\d).+$".to_string() // Must contain letters and digits
                ])
                .aliases(vec!["pwd".to_string()])
                .tags(vec!["security".to_string()])
                .end(),

            // List with length validation
            ArgumentDefinition::former()
                .name("tags")
                .description("List of tags (2-10 items)".to_string())
                .kind(Kind::List(Box::new(Kind::String), Some(',')))
                .hint("Comma-separated tags")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![
                    "min_length:2".to_string(), // Minimum 2 items in list
                ])
                .aliases(vec!["t".to_string()])
                .tags(vec!["metadata".to_string()])
                .end(),

            // URL with pattern validation
            ArgumentDefinition::former()
                .name("website")
                .description("Website URL (must be HTTPS)".to_string())
                .kind(Kind::Url)
                .hint("HTTPS URL only")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![
                    "regex:^https://".to_string() // Must start with https://
                ])
                .aliases(vec!["url".to_string()])
                .tags(vec!["web".to_string(), "security".to_string()])
                .end(),
        ])
        .end();

    let validation_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        println!("✓ All validation rules passed!");
        println!("\nValidated arguments:");
        
        for (name, value) in &cmd.arguments {
            let value_str = match value {
                Value::String(s) if name == "password" => "*".repeat(s.len()), // Hide sensitive data
                _ => format!("{:?}", value),
            };
            println!("  {}: {}", name, value_str);
        }
        
        Ok(OutputData {
            content: "All arguments validated successfully".to_string(),
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&validation_demo, validation_routine)?;
    println!("✓ Registered validation demonstration command");

    println!("\n=== Supported Validation Rules ===");
    println!("Numeric constraints:");
    println!("  • min:N - Minimum value for integers/floats");
    println!("  • max:N - Maximum value for integers/floats");
    
    println!("\nString constraints:");
    println!("  • min_length:N - Minimum string/list length");
    println!("  • regex:PATTERN - Must match regular expression");
    
    println!("\nCollection constraints:");
    println!("  • min_length:N - Minimum number of items in lists/maps");

    println!("\n=== Argument Attributes ===");
    println!("  • optional - Argument is not required");
    println!("  • multiple - Argument can be specified multiple times");
    println!("  • sensitive - Argument contains sensitive data");
    println!("  • interactive - Argument may require user interaction");
    println!("  • is_default_arg - Use default value when not specified");

    println!("\n=== Example Usage ===");
    println!("# Valid examples:");
    println!("cargo run --bin unilang_cli validation.validate age::25 name::Alice");
    println!("cargo run --bin unilang_cli validation.validate score::95.5 email::alice@example.com");
    println!("cargo run --bin unilang_cli validation.validate password::mypass123 website::https://example.com");
    
    println!("\n# These will fail validation:");
    println!("cargo run --bin unilang_cli validation.validate age::150  # Age too high");
    println!("cargo run --bin unilang_cli validation.validate name::A    # Name too short");
    println!("cargo run --bin unilang_cli validation.validate email::invalid-email  # Bad email format");
    println!("cargo run --bin unilang_cli validation.validate password::short  # Password too short");

    Ok(())
}