//! Manual debug example for string parser functionality.

use strs_tools::string::parser::*;

fn main() {
    let input = "myapp --verbose --output:result.txt input1.txt";
    println!("Input: '{}'", input);
    
    let results: Result<Vec<_>, _> = input.parse_command_line().collect();
    
    match results {
        Ok(tokens) => {
            println!("Parsed {} tokens:", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("{}: {:?}", i, token);
            }
        },
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
    
    // Test individual components
    println!("\nTesting key-value parsing:");
    let kv_test = "--output:result.txt";
    println!("KV test input: '{}'", kv_test);
    if kv_test.starts_with("--") {
        let without_prefix = &kv_test[2..];
        println!("Without prefix: '{}'", without_prefix);
        if without_prefix.contains(":") {
            let parts: Vec<_> = without_prefix.splitn(2, ":").collect();
            println!("Split parts: {:?}", parts);
        }
    }
}