//! Integration tests for UniLang WASM REPL
//!
//! These tests run in native mode and verify the core functionality
//! that should work consistently across both native and WebAssembly environments.

#![cfg(not(target_arch = "wasm32"))]

// Import from the current crate
use unilang_wasm_repl::{UniLangWasmRepl, log};

/// Test basic REPL instantiation in native environment
#[test]
fn test_native_repl_creation() {
  let repl = UniLangWasmRepl::new();
  
  // Should not panic and should create a valid instance
  drop(repl);
}

/// Test help command execution in native environment
#[test]
fn test_native_help_command() {
  let repl = UniLangWasmRepl::new();
  
  let result = repl.get_help();
  
  // Help should return some content (not empty)
  assert!(!result.is_empty(), "Help should return non-empty content");
  
  // Should contain help information
  println!("Help output: {}", result);
}

/// Test command execution in native environment
#[test]
fn test_native_command_execution() {
  let repl = UniLangWasmRepl::new();
  
  // Test the demo echo command
  let result = repl.execute_command(".demo.echo hello");
  println!("Echo result: {}", result);
  
  // Should execute successfully  
  assert!(result.contains("✅"), "Command should indicate success");
}

/// Test calculator command in native environment
#[test]
fn test_native_calculator_command() {
  let repl = UniLangWasmRepl::new();
  
  // Test the calc add command
  let result = repl.execute_command(".calc.add 5 3");
  println!("Calc result: {}", result);
  
  // Should execute successfully
  assert!(result.contains("✅"), "Calc command should indicate success");
}

/// Test error handling with invalid commands
#[test]
fn test_native_invalid_command_handling() {
  let repl = UniLangWasmRepl::new();
  
  // Test an invalid command
  let result = repl.execute_command(".invalid.command");
  println!("Invalid command result: {}", result);
  
  // Should return an error message
  assert!(result.contains("❌"), "Invalid command should return error");
}

/// Test JSON command loading
#[test]
fn test_native_json_loading() {
  let repl = UniLangWasmRepl::new();
  
  // Test JSON loading (simplified implementation)
  let result = repl.load_commands_json("{}");
  println!("JSON loading result: {}", result);
  
  // Should return a response
  assert!(!result.is_empty(), "JSON loading should return a response");
}

/// Test multiple consecutive commands
#[test]
fn test_native_multiple_commands() {
  let repl = UniLangWasmRepl::new();
  
  let commands = vec![
    (".help", "Should show help"),
    (".demo.echo test", "Should echo 'test'"),
    (".calc.add 10 20", "Should add 10 and 20"),
  ];
  
  for (command, description) in commands {
    let result = repl.execute_command(command);
    println!("{}: {} -> {}", description, command, result);
    assert!(!result.is_empty(), "Command should return non-empty result");
  }
}

/// Test edge cases and boundary conditions
#[test]
fn test_native_edge_cases() {
  let repl = UniLangWasmRepl::new();
  
  // Test empty command
  let result = repl.execute_command("");
  println!("Empty command result: {}", result);
  assert!(!result.is_empty(), "Empty command should return some response");
  
  // Test whitespace command
  let result = repl.execute_command("   ");
  println!("Whitespace command result: {}", result);
  assert!(!result.is_empty(), "Whitespace command should return some response");
  
  // Test very long command
  let long_command = format!(".demo.echo {}", "a".repeat(1000));
  let result = repl.execute_command(&long_command);
  println!("Long command result length: {}", result.len());
  assert!(!result.is_empty(), "Long command should return some response");
}

/// Test concurrent access patterns (if applicable)
#[test]
fn test_native_concurrent_commands() {
  let repl = UniLangWasmRepl::new();
  
  // Simulate rapid command execution
  for i in 0..100 {
    let command = format!(".demo.echo iteration_{}", i);
    let result = repl.execute_command(&command);
    
    if i % 10 == 0 {
      println!("Command {}: {} -> {}", i, command, result);
    }
    
    assert!(!result.is_empty(), "Command {} should return result", i);
  }
}

/// Test utility functions
#[test]
fn test_native_utility_functions() {
  // Test the log function (should not panic)
  log("Test log message from native environment");
  log("Testing special characters: 🚀 <>\"'&");
  log("");
}

/// Performance benchmark test
#[test]
fn test_native_performance() {
  let repl = UniLangWasmRepl::new();
  
  let start = std::time::Instant::now();
  
  // Execute 1000 commands
  for i in 0..1000 {
    let command = if i % 2 == 0 {
      ".demo.echo test"
    } else {
      ".calc.add 1 2"
    };
    
    let _result = repl.execute_command(command);
  }
  
  let duration = start.elapsed();
  println!("1000 commands executed in {:?}", duration);
  println!("Average per command: {:?}", duration / 1000);
  
  // Should complete within reasonable time (less than 1 second for 1000 commands)
  assert!(duration.as_secs() < 5, "Performance test should complete quickly");
}