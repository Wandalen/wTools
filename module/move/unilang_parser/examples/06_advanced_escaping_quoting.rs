//! Advanced Escaping and Quoting Example
//!
//! This example demonstrates:
//! - Complex escape sequences (\n, \t, \\, \", \')
//! - Regex patterns with escaping
//! - Mixed quote types and special characters

use unilang_parser::{Parser, UnilangParserOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new(UnilangParserOptions::default());

    // Complex escaping scenarios
    println!("=== Complex Escape Sequences ===");
    let cmd = parser.parse_single_instruction(
        r#"log.message text::"Line 1\nLine 2\tTabbed" pattern::"\\d+\\.\\d+""#
    )?;

    println!("Command: {:?}", cmd.command_path_slices);

    // The parser handles escape sequences
    if let Some(text) = cmd.named_arguments.get("text") {
        println!("Text with escapes: {:?}", text);
        println!("Text displayed: {:?}", text);
    }

    if let Some(pattern) = cmd.named_arguments.get("pattern") {
        println!("Regex pattern: {:?}", pattern);
        println!("Pattern displayed: {:?}", pattern);
    }

    // JSON-like content with escaping
    println!("\n=== JSON Content with Escaping ===");
    let cmd2 = parser.parse_single_instruction(
        r#"api.send payload::"{\"name\": \"John Doe\", \"age\": 30, \"city\": \"New\\York\"}" content_type::"application/json""#
    )?;

    if let Some(payload) = cmd2.named_arguments.get("payload") {
        println!("JSON payload: {:?}", payload);
    }

    // File paths with spaces and special characters
    println!("\n=== File Paths with Special Characters ===");
    let cmd3 = parser.parse_single_instruction(
        r#"file.process input::"/path/with spaces/file(1).txt" output::"/backup/file_copy.txt""#
    )?;

    println!("Input file: {:?}", cmd3.named_arguments.get("input").unwrap());
    println!("Output file: {:?}", cmd3.named_arguments.get("output").unwrap());

    // Mixed single and double quotes
    println!("\n=== Mixed Quote Types ===");
    let cmd4 = parser.parse_single_instruction(
        r#"script.run command::'echo "Hello World"' timeout::30"#
    )?;

    println!("Script command: {:?}", cmd4.named_arguments.get("command").unwrap());

    // SQL with complex escaping
    println!("\n=== SQL with Complex Escaping ===");
    let cmd5 = parser.parse_single_instruction(
        r#"db.query sql::"SELECT * FROM users WHERE name LIKE '%O\'Reilly%' AND status = \"active\"" limit::100"#
    )?;

    if let Some(sql) = cmd5.named_arguments.get("sql") {
        println!("SQL query: {:?}", sql);
    }

    println!("\n✓ Advanced escaping and quoting parsing successful!");
    Ok(())
}