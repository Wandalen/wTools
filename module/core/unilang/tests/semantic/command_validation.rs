//! Test explicit command naming validation (FR-REG-6)
//! 
//! Tests that the framework enforces explicit dot prefixes and rejects
//! commands that don't follow the naming requirements.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::print_literal ) ]
#![ allow( clippy::single_char_pattern ) ]

#![ allow( deprecated ) ]

use unilang::{ CommandDefinition, CommandRegistry, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

fn dummy_handler(_cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  Ok( OutputData { content: "test".to_string(), format: "text".to_string(), execution_time_ms: None } )
}

#[test]
fn test_reject_commands_without_dot_prefix()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  // This should be REJECTED - no dot prefix
  let invalid_cmd = CommandDefinition {
    name: "chat".to_string(), // ❌ Missing dot prefix
    namespace: String::new(),
    description: "This should be rejected".to_string(),
    routine_link: None,
    arguments: Vec::new(),
    hint: String::new(),
    status: String::new(),
    version: String::new(),
    tags: Vec::new(),
    aliases: Vec::new(),
    permissions: Vec::new(),
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: Vec::new(),
    auto_help_enabled: false,
  };
  
  #[allow(deprecated)]
    #[allow(deprecated)]
    let result = registry.command_add_runtime(&invalid_cmd, Box::new(dummy_handler));
  
  // Should fail with explicit error message
  assert!(result.is_err(), "Command without dot prefix should be rejected");
  
  let error_msg = format!("{:?}", result.unwrap_err());
  assert!(error_msg.contains("must start with dot prefix"), 
         "Error should mention dot prefix requirement: {}", error_msg);
  assert!(error_msg.contains("minimal implicit transformations"), 
         "Error should reference the principle: {}", error_msg);
  
  println!("✅ Correctly rejected command without dot prefix");
}

#[test] 
fn test_reject_invalid_namespace()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  // This should be REJECTED - namespace without dot prefix
  let invalid_cmd = CommandDefinition {
    name: ".list".to_string(), // ✅ Correct name
    namespace: "session".to_string(), // ❌ Namespace missing dot
    description: "This should be rejected".to_string(),
    routine_link: None,
    arguments: Vec::new(),
    hint: String::new(),
    status: String::new(),
    version: String::new(),
    tags: Vec::new(),
    aliases: Vec::new(),
    permissions: Vec::new(),
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: Vec::new(),
    auto_help_enabled: false,
  };
  
  #[allow(deprecated)]
    #[allow(deprecated)]
    let result = registry.command_add_runtime(&invalid_cmd, Box::new(dummy_handler));
  
  // Should fail with explicit error message
  assert!(result.is_err(), "Namespace without dot prefix should be rejected");
  
  let error_msg = format!("{:?}", result.unwrap_err());
  assert!(error_msg.contains("namespace"), 
         "Error should mention namespace: {}", error_msg);
  assert!(error_msg.contains("must start with dot prefix"), 
         "Error should mention dot prefix requirement: {}", error_msg);
  
  println!("✅ Correctly rejected invalid namespace");
}

#[test]
fn test_accept_correctly_formatted_commands()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  // Root-level command - should be accepted
  let root_cmd = CommandDefinition {
    name: ".test_chat".to_string(), // ✅ Correct dot prefix
    namespace: String::new(), // ✅ Empty namespace for root
    description: "Correctly formatted root command".to_string(),
    routine_link: None,
    arguments: Vec::new(),
    hint: String::new(),
    status: String::new(),
    version: String::new(),
    tags: Vec::new(),
    aliases: Vec::new(),
    permissions: Vec::new(),
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: Vec::new(),
    auto_help_enabled: false,
  };
  
  #[allow(deprecated)]
    #[allow(deprecated)]
    let result = registry.command_add_runtime(&root_cmd, Box::new(dummy_handler));
  assert!(result.is_ok(), "Correctly formatted root command should be accepted");
  println!("✅ Accepted correctly formatted root command");
  
  // Namespaced command - should be accepted
  let namespaced_cmd = CommandDefinition {
    name: ".list".to_string(), // ✅ Correct dot prefix  
    namespace: ".session".to_string(), // ✅ Correct namespace with dot
    description: "Correctly formatted namespaced command".to_string(),
    routine_link: None,
    arguments: Vec::new(),
    hint: String::new(),
    status: String::new(),
    version: String::new(),
    tags: Vec::new(),
    aliases: Vec::new(),
    permissions: Vec::new(),
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: Vec::new(),
    auto_help_enabled: false,
  };
  
  #[allow(deprecated)]
    #[allow(deprecated)]
    let result2 = registry.command_add_runtime(&namespaced_cmd, Box::new(dummy_handler));
  assert!(result2.is_ok(), "Correctly formatted namespaced command should be accepted");
  println!("✅ Accepted correctly formatted namespaced command");
}

#[test]
fn test_principle_minimum_implicit_magic()
{
  println!("\n🎯 TESTING GOVERNING PRINCIPLE: Minimum Implicit Magic");
  println!("   - Commands registered exactly as specified");
  println!("   - No automatic transformations or prefix additions");
  println!("   - Explicit validation with clear error messages");
  println!("   - What you register is exactly what gets executed\n");
  
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  
  // Test cases demonstrating the principle
  let test_cases = vec![
    ("chat", "❌ Should fail - missing dot prefix"),
    (".chat", "✅ Should pass - explicit dot prefix"),
  ];
  
  for (name, _expected) in test_cases {
    let cmd = CommandDefinition {
      name: name.to_string(),
      namespace: String::new(),
      description: format!("Testing name: {}", name),
      routine_link: None,
      arguments: Vec::new(),
      hint: String::new(),
      status: String::new(),
      version: String::new(),
      tags: Vec::new(),
      aliases: Vec::new(),
      permissions: Vec::new(),
      idempotent: false,
      deprecation_message: String::new(),
      http_method_hint: String::new(),
      examples: Vec::new(),
    auto_help_enabled: false,
    };
    
    #[allow(deprecated)]
        let result = registry.command_add_runtime(&cmd, Box::new(dummy_handler));
    
    if name.starts_with('.') {
      assert!(result.is_ok(), "Command '{}' should be accepted", name);
      println!("   {} Command '{}' correctly accepted", "✅", name);
    } else {
      assert!(result.is_err(), "Command '{}' should be rejected", name);
      println!("   {} Command '{}' correctly rejected", "❌", name);
      let error = format!("{:?}", result.unwrap_err());
      println!("      Reason: {}", error.split("Registration(").nth(1).unwrap_or("unknown").trim_end_matches("\")\")").trim_start_matches("\""));
    }
  }
  
  println!("\n🎉 Principle successfully enforced!");
}