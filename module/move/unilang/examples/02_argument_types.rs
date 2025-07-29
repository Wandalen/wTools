//! # Argument Types Demo
//! 
//! This example demonstrates all the supported argument types in Unilang,
//! including basic types, collections, and complex validation.

use unilang::data::{CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes, OutputData};
use unilang::registry::CommandRegistry;
use unilang::types::Value;

fn main() -> Result<(), unilang::error::Error> {
    println!("=== Argument Types Demo ===\n");

    let mut registry = CommandRegistry::new();

    // Step 1: Command with various basic argument types
    let types_demo = CommandDefinition::former()
        .name("types_demo")
        .namespace("".to_string())
        .description("Demonstrates all supported argument types".to_string())
        .hint("Shows how different data types work")
        .status("stable")
        .version("1.0.0")
        .aliases(vec![])
        .tags(vec!["demo".to_string(), "types".to_string()])
        .permissions(vec![])
        .idempotent(true)
        .deprecation_message("".to_string())
        .http_method_hint("POST".to_string())
        .examples(vec![
            "types_demo text::hello number::42 flag::true".to_string(),
            "types_demo url::https://example.com path::/tmp/file".to_string()
        ])
        .arguments(vec![
            // String argument
            ArgumentDefinition::former()
                .name("text")
                .description("A text string argument".to_string())
                .kind(Kind::String)
                .hint("Any text string")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec!["min_length:3".to_string()])
                .aliases(vec!["t".to_string()])
                .tags(vec!["string".to_string()])
                .end(),

            // Integer argument
            ArgumentDefinition::former()
                .name("number")
                .description("An integer number".to_string())
                .kind(Kind::Integer)
                .hint("Whole number")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec!["min:0".to_string(), "max:100".to_string()])
                .aliases(vec!["n".to_string()])
                .tags(vec!["numeric".to_string()])
                .end(),

            // Float argument
            ArgumentDefinition::former()
                .name("decimal")
                .description("A floating-point number".to_string())
                .kind(Kind::Float)
                .hint("Decimal number")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec!["min:0.0".to_string()])
                .aliases(vec!["d".to_string()])
                .tags(vec!["numeric".to_string()])
                .end(),

            // Boolean argument
            ArgumentDefinition::former()
                .name("flag")
                .description("A boolean flag".to_string())
                .kind(Kind::Boolean)
                .hint("True or false value")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![])
                .aliases(vec!["f".to_string()])
                .tags(vec!["boolean".to_string()])
                .end(),

            // Path argument
            ArgumentDefinition::former()
                .name("path")
                .description("A file system path".to_string())
                .kind(Kind::Path)
                .hint("File or directory path")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![])
                .aliases(vec!["p".to_string()])
                .tags(vec!["filesystem".to_string()])
                .end(),

            // URL argument
            ArgumentDefinition::former()
                .name("url")
                .description("A web URL".to_string())
                .kind(Kind::Url)
                .hint("Valid HTTP/HTTPS URL")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec!["regex:^https?://".to_string()])
                .aliases(vec!["u".to_string()])
                .tags(vec!["web".to_string()])
                .end(),

            // DateTime argument
            ArgumentDefinition::former()
                .name("timestamp")
                .description("A date and time".to_string())
                .kind(Kind::DateTime)
                .hint("ISO 8601 datetime")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![])
                .aliases(vec!["ts".to_string()])
                .tags(vec!["time".to_string()])
                .end(),

            // Enum argument
            ArgumentDefinition::former()
                .name("level")
                .description("A predefined choice".to_string())
                .kind(Kind::Enum(vec!["debug".to_string(), "info".to_string(), "warn".to_string(), "error".to_string()]))
                .hint("Log level choice")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![])
                .aliases(vec!["l".to_string()])
                .tags(vec!["choice".to_string()])
                .end(),

            // Pattern/Regex argument
            ArgumentDefinition::former()
                .name("pattern")
                .description("A regular expression pattern".to_string())
                .kind(Kind::Pattern)
                .hint("Regex pattern string")
                .attributes(ArgumentAttributes::former().optional(true).end())
                .validation_rules(vec![])
                .aliases(vec!["regex".to_string()])
                .tags(vec!["pattern".to_string()])
                .end(),
        ])
        .end();

    let types_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
        println!("Processing arguments:");
        
        for (name, value) in &cmd.arguments {
            match value {
                Value::String(s) => println!("  {}: '{}' (String)", name, s),
                Value::Integer(i) => println!("  {}: {} (Integer)", name, i),
                Value::Float(f) => println!("  {}: {} (Float)", name, f),
                Value::Boolean(b) => println!("  {}: {} (Boolean)", name, b),
                Value::Path(p) => println!("  {}: {:?} (Path)", name, p),
                Value::File(f) => println!("  {}: {:?} (File)", name, f),
                Value::Directory(d) => println!("  {}: {:?} (Directory)", name, d),
                Value::Enum(e) => println!("  {}: '{}' (Enum)", name, e),
                Value::Url(u) => println!("  {}: {} (Url)", name, u),
                Value::DateTime(dt) => println!("  {}: {} (DateTime)", name, dt),
                Value::Pattern(p) => println!("  {}: {} (Pattern)", name, p),
                Value::List(items) => println!("  {}: {:?} (List)", name, items),
                Value::Map(map) => println!("  {}: {:?} (Map)", name, map),
                Value::JsonString(json) => println!("  {}: {} (JsonString)", name, json),
                Value::Object(obj) => println!("  {}: {:?} (Object)", name, obj),
            }
        }
        
        Ok(OutputData {
            content: "Arguments processed successfully".to_string(),
            format: "text".to_string(),
        })
    });

    registry.command_add_runtime(&types_demo, types_routine)?;
    println!("✓ Registered command with various argument types");

    println!("\n=== Supported Argument Types ===");
    println!("• String - Text data");
    println!("• Integer - Whole numbers");
    println!("• Float - Decimal numbers");
    println!("• Boolean - True/false values");
    println!("• Path - File system paths");
    println!("• File - File paths (validated)");
    println!("• Directory - Directory paths (validated)");
    println!("• Url - Web URLs");
    println!("• DateTime - Date/time values");
    println!("• Pattern - Regular expressions");
    println!("• Enum - Predefined choices");
    println!("• List - Collections of items");
    println!("• Map - Key-value pairs");
    println!("• JsonString - JSON text");
    println!("• Object - JSON objects");

    println!("\n=== Example Usage ===");
    println!("cargo run --bin unilang_cli types_demo \\");
    println!("  text::'Hello World' \\");
    println!("  number::42 \\");
    println!("  decimal::3.14 \\");
    println!("  flag::true \\");
    println!("  path::/tmp/test \\");
    println!("  url::https://example.com \\");
    println!("  level::info \\");
    println!("  pattern::'^[a-z]+$'");

    Ok(())
}