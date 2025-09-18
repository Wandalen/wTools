#![allow(clippy::all)]
//! # WebAssembly REPL for Unilang
//!
//! This example demonstrates how to use the unilang command framework in a WebAssembly environment.
//! It provides a web-based REPL (Read-Eval-Print Loop) interface for interacting with unilang commands.

use wasm_bindgen::prelude::*;
use web_sys::console;

// Import the `wee_alloc` global allocator for small WASM binary size
#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Set up panic hook for better error messages in development
#[wasm_bindgen(start)]
pub fn main() {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

/// Demo handler for WebAssembly commands
fn demo_handler(cmd: unilang::VerifiedCommand, _ctx: unilang::ExecutionContext) -> Result<unilang::OutputData, unilang::ErrorData> {
  let output = format!("✅ Successfully executed command: {}", cmd.definition.name);
  Ok(unilang::OutputData {
    content: output,
    format: "text".to_string()
  })
}

/// WebAssembly REPL interface for unilang commands
#[wasm_bindgen]
pub struct UniLangWasmRepl {
  pipeline: unilang::Pipeline,
}

#[wasm_bindgen]
impl UniLangWasmRepl {
  /// Create a new WebAssembly REPL instance
  #[wasm_bindgen(constructor)]
  pub fn new() -> UniLangWasmRepl {
    // Create a command registry
    #[allow(deprecated)]
    let mut registry = unilang::CommandRegistry::new();
    
    // Register basic commands suitable for WebAssembly environment
    Self::register_basic_commands(&mut registry);
    
    // Create pipeline with the registry
    let pipeline = unilang::Pipeline::new(registry);
    
    UniLangWasmRepl { pipeline }
  }
  
  /// Process a command input and return the result
  #[wasm_bindgen]
  pub fn execute_command(&self, input: &str) -> String {
    let result = self.pipeline.process_command_simple(input);
    
    if result.success {
      if result.outputs.is_empty() {
        "✅ Command executed successfully".to_string()
      } else {
        result.outputs.iter()
          .map(|output| output.content.clone())
          .collect::<Vec<_>>()
          .join("\n")
      }
    } else {
      match &result.error {
        Some(error) => format!("❌ Error: {}", error),
        None => "❌ Unknown error occurred".to_string()
      }
    }
  }
  
  /// Get help information for available commands
  #[wasm_bindgen]
  pub fn get_help(&self) -> String {
    let result = self.pipeline.process_command_simple(".help");
    
    if result.success {
      result.outputs.iter()
        .map(|output| output.content.clone())
        .collect::<Vec<_>>()
        .join("\n")
    } else {
      "Help not available".to_string()
    }
  }
  
  /// Load command definitions from JSON (simplified for demo)
  #[wasm_bindgen]
  pub fn load_commands_json(&self, _json: &str) -> String {
    // In a full implementation, this would parse JSON and register commands
    // For this demo, we'll just return a success message
    "✅ JSON command loading not implemented in this demo".to_string()
  }
  
  /// Register basic commands for demonstration
  fn register_basic_commands(registry: &mut unilang::CommandRegistry) {
    // Register a simple echo command
    let echo_cmd = unilang::CommandDefinition {
      name: "echo".to_string(),
      description: "Simple echo command for WebAssembly demo".to_string(),
      namespace: "demo".to_string(),
      hint: "Echo the input text".to_string(),
      arguments: vec![
        unilang::ArgumentDefinition {
          name: "text".to_string(),
          kind: unilang::Kind::String,
          attributes: unilang::ArgumentAttributes::default(),
          hint: "Text to echo".to_string(),
          description: "The text that will be echoed back".to_string(),
          validation_rules: Vec::new(),
          aliases: Vec::new(),
          tags: Vec::new(),
        }
      ],
      routine_link: None,
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: Vec::new(),
      aliases: Vec::new(),
      examples: Vec::new(),
    auto_help_enabled: false,
      permissions: Vec::new(),
      idempotent: true,
      deprecation_message: String::new(),
      http_method_hint: "GET".to_string(),
    };
    
    #[allow(deprecated)]
    if let Err(e) = registry.command_add_runtime(&echo_cmd, Box::new(demo_handler)) {
      console::error_1(&format!("Failed to register echo command: {}", e).into());
    }
    
    // Register a simple calculator command
    let calc_cmd = unilang::CommandDefinition {
      name: "add".to_string(),
      description: "Simple addition calculator for WebAssembly demo".to_string(),
      namespace: "calc".to_string(),
      hint: "Add two numbers".to_string(),
      arguments: vec![
        unilang::ArgumentDefinition {
          name: "a".to_string(),
          kind: unilang::Kind::Integer,
          attributes: unilang::ArgumentAttributes::default(),
          hint: "First number".to_string(),
          description: "The first number to add".to_string(),
          validation_rules: Vec::new(),
          aliases: Vec::new(),
          tags: Vec::new(),
        },
        unilang::ArgumentDefinition {
          name: "b".to_string(),
          kind: unilang::Kind::Integer,
          attributes: unilang::ArgumentAttributes::default(),
          hint: "Second number".to_string(),
          description: "The second number to add".to_string(),
          validation_rules: Vec::new(),
          aliases: Vec::new(),
          tags: Vec::new(),
        }
      ],
      routine_link: None,
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: Vec::new(),
      aliases: Vec::new(),
      examples: Vec::new(),
    auto_help_enabled: false,
      permissions: Vec::new(),
      idempotent: true,
      deprecation_message: String::new(),
      http_method_hint: "GET".to_string(),
    };
    
    #[allow(deprecated)]
    if let Err(e) = registry.command_add_runtime(&calc_cmd, Box::new(demo_handler)) {
      console::error_1(&format!("Failed to register add command: {}", e).into());
    }
  }
}

/// Utility function to log messages from WASM to browser console
#[wasm_bindgen]
pub fn log(s: &str) {
  console::log_1(&s.into());
}