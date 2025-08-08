//! Manual testing program for parser integration functionality
//!
//! This program demonstrates and tests various parser integration features
//! through interactive examples and validates functionality manually.

use strs_tools::string::parser::*;
use std::time::Instant;

fn main() {
    println!("=== Parser Integration Manual Testing ===\n");
    
    test_basic_single_pass_parsing();
    test_command_line_parsing_scenarios();
    test_validation_functionality();
    test_error_handling();
    test_performance_comparison();
    test_real_world_scenarios();
    
    println!("=== All Manual Tests Completed Successfully ===");
}

fn test_basic_single_pass_parsing() {
    println!("📋 Testing Basic Single-Pass Parsing");
    println!("────────────────────────────────────────");
    
    // Test 1: Parse integers
    let input = "1,2,3,4,5";
    println!("Input: '{}'", input);
    
    let results: Result<Vec<i32>, _> = input
        .split_and_parse(&[","], |token| {
            token.parse().map_err(|_| ParseError::InvalidToken {
                token: token.to_string(),
                position: 0,
                expected: "integer".to_string(),
            })
        })
        .collect();
    
    match results {
        Ok(numbers) => println!("✅ Parsed integers: {:?}", numbers),
        Err(e) => println!("❌ Error: {:?}", e),
    }
    
    // Test 2: Parse with mixed types
    let input = "apple,123,banana,456";
    println!("\nInput: '{}'", input);
    println!("Attempting to parse as integers (should have errors):");
    
    let results: Vec<_> = input
        .split_and_parse(&[","], |token| {
            token.parse::<i32>().map_err(|_| ParseError::InvalidToken {
                token: token.to_string(),
                position: 0,
                expected: "integer".to_string(),
            })
        })
        .collect();
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(num) => println!("  Token {}: ✅ {}", i, num),
            Err(e) => println!("  Token {}: ❌ {:?}", i, e),
        }
    }
    
    println!();
}

fn test_command_line_parsing_scenarios() {
    println!("⚡ Testing Command-Line Parsing Scenarios");
    println!("─────────────────────────────────────────────");
    
    let test_cases = vec![
        "simple_app",
        "app --verbose",
        "app --output:result.txt input.txt",
        "server --port:8080 --host:localhost --ssl debug.log",
        "compile --target:x86_64 --release --jobs:4 src/",
        "git commit --message:\"Fix parser\" --author:\"user@example.com\"",
    ];
    
    for (i, input) in test_cases.iter().enumerate() {
        println!("\nTest Case {}: '{}'", i + 1, input);
        
        let results: Result<Vec<_>, _> = input.parse_command_line().collect();
        match results {
            Ok(tokens) => {
                println!("  ✅ Parsed {} tokens:", tokens.len());
                for (j, token) in tokens.iter().enumerate() {
                    match token {
                        ParsedToken::Command(cmd) => println!("    {}: Command({})", j, cmd),
                        ParsedToken::Flag(flag) => println!("    {}: Flag({})", j, flag),
                        ParsedToken::KeyValue { key, value } => println!("    {}: KeyValue({}={})", j, key, value),
                        ParsedToken::Positional(arg) => println!("    {}: Positional({})", j, arg),
                    }
                }
            },
            Err(e) => println!("  ❌ Error: {:?}", e),
        }
    }
    
    println!();
}

fn test_validation_functionality() {
    println!("🔍 Testing Validation Functionality");
    println!("────────────────────────────────────");
    
    // Test 1: Alphabetic validation
    let input = "apple,123,banana,456,cherry";
    println!("Input: '{}'", input);
    println!("Validating alphabetic tokens only:");
    
    let results: Vec<_> = input
        .split_with_validation(&[","], |token| {
            token.chars().all(|c| c.is_alphabetic())
        })
        .collect();
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(token) => println!("  Token {}: ✅ '{}'", i, token),
            Err(e) => println!("  Token {}: ❌ {:?}", i, e),
        }
    }
    
    // Test 2: Token counting
    let alpha_count = input.count_valid_tokens(&[","], |token| {
        token.chars().all(|c| c.is_alphabetic())
    });
    let numeric_count = input.count_valid_tokens(&[","], |token| {
        token.chars().all(|c| c.is_numeric())
    });
    
    println!("  📊 Alphabetic tokens: {}", alpha_count);
    println!("  📊 Numeric tokens: {}", numeric_count);
    
    println!();
}

fn test_error_handling() {
    println!("🚨 Testing Error Handling");
    println!("─────────────────────────");
    
    // Test 1: Invalid key-value pairs
    let invalid_kvs = vec!["--key:", ":value", "--:", "key:"];
    
    for kv in invalid_kvs {
        println!("\nTesting invalid key-value: '{}'", kv);
        let results: Result<Vec<_>, _> = kv.parse_command_line().collect();
        match results {
            Ok(tokens) => println!("  ✅ Parsed: {:?}", tokens),
            Err(e) => println!("  ❌ Error (expected): {:?}", e),
        }
    }
    
    // Test 2: Empty inputs
    let empty_inputs = vec!["", "   ", "\t\t", "   \n  "];
    
    for input in empty_inputs {
        println!("\nTesting empty input: '{:?}'", input);
        let results: Result<Vec<_>, _> = input.parse_command_line().collect();
        match results {
            Ok(tokens) => println!("  ✅ Parsed {} tokens", tokens.len()),
            Err(e) => println!("  ❌ Error: {:?}", e),
        }
    }
    
    println!();
}

fn test_performance_comparison() {
    println!("⏱️  Testing Performance Comparison");
    println!("──────────────────────────────────");
    
    let test_data = "word1,word2,word3,word4,word5,word6,word7,word8,word9,word10";
    let iterations = 1000;
    
    // Traditional multi-pass approach
    let start = Instant::now();
    for _ in 0..iterations {
        let tokens: Vec<&str> = test_data.split(',').collect();
        let _results: Vec<String> = tokens.iter().map(|s| s.to_uppercase()).collect();
    }
    let traditional_time = start.elapsed();
    
    // Single-pass parser approach
    let start = Instant::now();
    for _ in 0..iterations {
        let _results: Result<Vec<String>, _> = test_data
            .split_and_parse(&[","], |token| {
                Ok(token.to_uppercase())
            })
            .collect();
    }
    let parser_time = start.elapsed();
    
    println!("Performance comparison ({} iterations):", iterations);
    println!("  Traditional approach: {:?}", traditional_time);
    println!("  Parser integration:   {:?}", parser_time);
    
    let improvement = if parser_time.as_nanos() > 0 {
        traditional_time.as_nanos() as f64 / parser_time.as_nanos() as f64
    } else {
        1.0
    };
    
    println!("  Performance ratio:    {:.2}x", improvement);
    
    println!();
}

fn test_real_world_scenarios() {
    println!("🌍 Testing Real-World Scenarios");
    println!("───────────────────────────────");
    
    // Scenario 1: Configuration parsing
    println!("Scenario 1: Configuration file parsing");
    let config = "timeout:30,retries:3,host:localhost,port:8080,ssl:true";
    
    #[derive(Debug)]
    struct Config {
        timeout: u32,
        retries: u32,
        host: String,
        port: u16,
        ssl: bool,
    }
    
    let mut config_values = Config {
        timeout: 10,
        retries: 1,
        host: "127.0.0.1".to_string(),
        port: 80,
        ssl: false,
    };
    
    let results: Result<Vec<_>, _> = config
        .split_and_parse(&[","], |token| {
            if let Some(colon_pos) = token.find(':') {
                let key = &token[..colon_pos];
                let value = &token[colon_pos + 1..];
                Ok((key.to_string(), value.to_string()))
            } else {
                Err(ParseError::InvalidKeyValuePair(token.to_string()))
            }
        })
        .collect();
    
    match results {
        Ok(pairs) => {
            println!("  ✅ Parsed {} configuration pairs:", pairs.len());
            for (key, value) in pairs {
                match key.as_str() {
                    "timeout" => {
                        config_values.timeout = value.parse().unwrap_or(config_values.timeout);
                        println!("    timeout = {}", config_values.timeout);
                    },
                    "retries" => {
                        config_values.retries = value.parse().unwrap_or(config_values.retries);
                        println!("    retries = {}", config_values.retries);
                    },
                    "host" => {
                        config_values.host = value;
                        println!("    host = {}", config_values.host);
                    },
                    "port" => {
                        config_values.port = value.parse().unwrap_or(config_values.port);
                        println!("    port = {}", config_values.port);
                    },
                    "ssl" => {
                        config_values.ssl = value == "true";
                        println!("    ssl = {}", config_values.ssl);
                    },
                    _ => println!("    unknown key: {}", key),
                }
            }
            println!("  Final config: {:?}", config_values);
        },
        Err(e) => println!("  ❌ Configuration parsing error: {:?}", e),
    }
    
    // Scenario 2: Log parsing
    println!("\nScenario 2: Log entry parsing");
    let log_entry = "app --level:info --module:parser --message:\"Processing complete\" --timestamp:1234567890";
    
    let results: Result<Vec<_>, _> = log_entry.parse_command_line().collect();
    match results {
        Ok(tokens) => {
            println!("  ✅ Parsed log entry with {} tokens:", tokens.len());
            for token in tokens {
                match token {
                    ParsedToken::Command(app) => println!("    Application: {}", app),
                    ParsedToken::KeyValue { key: "level", value } => println!("    Log Level: {}", value),
                    ParsedToken::KeyValue { key: "module", value } => println!("    Module: {}", value),
                    ParsedToken::KeyValue { key: "message", value } => println!("    Message: {}", value),
                    ParsedToken::KeyValue { key: "timestamp", value } => {
                        if let Ok(ts) = value.parse::<u64>() {
                            println!("    Timestamp: {} ({})", ts, value);
                        } else {
                            println!("    Timestamp: {}", value);
                        }
                    },
                    ParsedToken::KeyValue { key, value } => println!("    {}: {}", key, value),
                    ParsedToken::Flag(flag) => println!("    Flag: {}", flag),
                    ParsedToken::Positional(arg) => println!("    Argument: {}", arg),
                }
            }
        },
        Err(e) => println!("  ❌ Log parsing error: {:?}", e),
    }
    
    println!();
}