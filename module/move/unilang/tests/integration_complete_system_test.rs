//! Complete system integration test demonstrating all implemented changes
//! 
//! This test validates that issue 017 has been completely resolved and that
//! the governing principle of "Minimum Implicit Magic" is properly enforced.

use unilang::{ CommandDefinition, CommandRegistry, Pipeline, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

fn demo_handler(cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  let output = format!("✅ Command '{}' executed successfully", cmd.definition.name);
  Ok( OutputData { content: output, format: "text".to_string() } )
}

#[test]
fn test_complete_system_integration()
{
  println!("\n🚀 COMPLETE SYSTEM INTEGRATION TEST");
  println!("Validating issue 017 resolution and governing principles\n");
  
  let mut registry = CommandRegistry::new();
  
  // Test 1: Root-level commands with explicit dot prefixes
  println!("📝 Test 1: Root-level commands");
  let root_commands = vec![
    (".chat", "Multi-agent chat system"),
    (".run", "Execute commands with prompts"),
    (".help", "Show help information"),
  ];
  
  for (name, desc) in &root_commands {
    let cmd = CommandDefinition {
      name: name.to_string(),
      namespace: String::new(),
      description: desc.to_string(),
      routine_link: None,
      arguments: Vec::new(),
      hint: String::new(),
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: Vec::new(),
      aliases: Vec::new(),
      permissions: Vec::new(),
      idempotent: false,
      deprecation_message: String::new(),
      http_method_hint: String::new(),
      examples: Vec::new(),
    };
    
    let result = registry.command_add_runtime(&cmd, Box::new(demo_handler));
    assert!(result.is_ok(), "Root command '{}' should register successfully", name);
    println!("  ✅ Registered: {}", name);
  }
  
  // Test 2: Namespaced commands
  println!("\n📝 Test 2: Namespaced commands");
  let namespaced_commands = vec![
    (".list", ".session", "List all sessions"),
    (".create", ".session", "Create new session"),
    (".add", ".math", "Add two numbers"),
  ];
  
  for (name, namespace, desc) in &namespaced_commands {
    let cmd = CommandDefinition {
      name: name.to_string(),
      namespace: namespace.to_string(),
      description: desc.to_string(),
      routine_link: None,
      arguments: Vec::new(),
      hint: String::new(),
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: Vec::new(),
      aliases: Vec::new(),
      permissions: Vec::new(),
      idempotent: false,
      deprecation_message: String::new(),
      http_method_hint: String::new(),
      examples: Vec::new(),
    };
    
    let result = registry.command_add_runtime(&cmd, Box::new(demo_handler));
    assert!(result.is_ok(), "Namespaced command '{}/{}' should register successfully", namespace, name);
    println!("  ✅ Registered: {}{}", namespace, name.strip_prefix('.').unwrap_or(name));
  }
  
  // Test 3: Validation rejects invalid commands
  println!("\n📝 Test 3: Validation enforcement");
  let invalid_commands = vec![
    ("chat", "Missing dot prefix"),
    ("run", "Missing dot prefix"), 
  ];
  
  for (invalid_name, reason) in &invalid_commands {
    let invalid_cmd = CommandDefinition {
      name: invalid_name.to_string(),
      namespace: String::new(),
      description: "This should fail".to_string(),
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
    };
    
    let result = registry.command_add_runtime(&invalid_cmd, Box::new(demo_handler));
    assert!(result.is_err(), "Command '{}' should be rejected: {}", invalid_name, reason);
    println!("  ❌ Correctly rejected: '{}' ({})", invalid_name, reason);
  }
  
  // Test 4: Command execution (resolving issue 017)
  println!("\n📝 Test 4: Command execution (Issue 017 resolution)");
  let pipeline = Pipeline::new(registry);
  
  let test_commands = vec![
    ".chat",
    ".run", 
    ".help",
    ".session.list",
    ".session.create",
    ".math.add",
  ];
  
  for cmd_name in &test_commands {
    let result = pipeline.process_command_simple(cmd_name);
    
    assert!(result.success, 
           "Command '{}' should execute successfully (Issue 017 was: commands registered but failed at runtime)", 
           cmd_name);
    
    assert!(!result.outputs.is_empty(), 
           "Command '{}' should produce output", cmd_name);
           
    let output_contains_success = result.outputs.iter()
      .any(|output| output.content.contains("executed successfully"));
    assert!(output_contains_success, 
           "Command '{}' should show successful execution", cmd_name);
           
    println!("  ✅ Executed: {} → {}", cmd_name, 
             result.outputs.first().map(|o| &o.content).unwrap_or(&"no output".to_string()));
  }
  
  println!("\n🎉 INTEGRATION TEST COMPLETE");
  println!("✅ Issue 017 resolved: Commands register and execute correctly");
  println!("✅ Governing principle enforced: Minimum Implicit Magic"); 
  println!("✅ Validation working: Invalid commands rejected with clear messages");
  println!("✅ Both root-level and namespaced commands function properly");
}

#[test]
fn test_governing_principles_compliance()
{
  println!("\n🎯 GOVERNING PRINCIPLES COMPLIANCE TEST\n");
  
  // Principle 1: Minimum Implicit Magic
  println!("🔍 Principle 1: Minimum Implicit Magic");
  let mut registry = CommandRegistry::new();
  
  let explicit_cmd = CommandDefinition {
    name: ".explicit_test".to_string(),
    namespace: String::new(),
    description: "Explicitly named command".to_string(),
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
  };
  
  let result = registry.command_add_runtime(&explicit_cmd, Box::new(demo_handler));
  assert!(result.is_ok(), "Explicit command should be accepted");
  println!("  ✅ Explicit naming accepted");
  
  let pipeline = Pipeline::new(registry);
  let execution_result = pipeline.process_command_simple(".explicit_test");
  assert!(execution_result.success, "Explicit command should execute");
  println!("  ✅ No implicit transformations - command used exactly as registered");
  
  // Principle 2: Fail-Fast Validation
  println!("\n🔍 Principle 2: Fail-Fast Validation");
  let mut registry2 = CommandRegistry::new();
  
  let invalid_cmd = CommandDefinition {
    name: "implicit_test".to_string(), // Missing dot
    namespace: String::new(),
    description: "Should fail validation".to_string(),
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
  };
  
  let result = registry2.command_add_runtime(&invalid_cmd, Box::new(demo_handler));
  assert!(result.is_err(), "Invalid command should be rejected at registration time");
  
  let error_msg = format!("{:?}", result.unwrap_err());
  assert!(error_msg.contains("must start with dot prefix"), 
         "Error should provide clear guidance");
  println!("  ✅ Fail-fast validation with clear error message");
  println!("  ✅ Registration-time validation prevents runtime issues");
  
  println!("\n🎉 All governing principles successfully enforced!");
}