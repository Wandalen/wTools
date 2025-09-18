//! Issue 017 Solution Documentation and Demonstration
//!

#![ allow( clippy::uninlined_format_args ) ]
//!
//! **Problem**: Commands registered with dot prefixes (e.g., ".chat") failed at runtime 
//! with "No executable routine found" errors, despite successful registration.
//!
//! **Root Cause**: The interpreter was adding dot prefixes to ALL commands during lookup,
//! causing ".chat" to become "..chat", which didn't match the registered key.
//!
//! **Solution**: Implemented "Minimum Implicit Magic" governing principle:
//! 1. **Explicit Validation**: All commands MUST start with dot prefix (e.g., ".chat")
//! 2. **No Transformations**: Interpreter uses command names exactly as registered
//! 3. **Fail-Fast**: Invalid commands are rejected at registration time with clear errors
//!
//! **Impact**: Eliminates all implicit transformations that caused the double-dot bug.
//! Commands work exactly as registered with predictable, explicit behavior.

use unilang::{ CommandDefinition, CommandRegistry, Pipeline, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

/// Demonstration command handler
#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
fn demo_handler(cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  let output = format!("✅ Successfully executed command: {}", cmd.definition.name);
  Ok( OutputData { content: output, format: "text".to_string() } )
}

#[test]
#[allow(clippy::too_many_lines)]
fn demonstrate_issue_017_solution()
{
  println!("\n🔍 Issue 017: Command Runtime Registration Failure - ACTUAL SOLUTION\n");

  // ❌ OLD PROBLEM: Interpreter double-prefixing caused failures
  println!("❌ ORIGINAL Issue 017 Problem:");
  println!("   Registration: name=\".chat\" → stored as \".chat\"");
  println!("   Lookup: interpreter added dot → looked for \"..chat\"");
  println!("   Result: \"No executable routine found\" ❌\n");
  
  // ✅ ACTUAL SOLUTION: Explicit naming with validation
  println!("✅ IMPLEMENTED Solution (Minimum Implicit Magic):");
  println!("   1. Validation: ALL commands must start with dot prefix");
  println!("   2. No transformations: Use names exactly as registered");
  println!("   3. Fail-fast: Invalid commands rejected at registration\n");
  
  let mut registry = CommandRegistry::new();
  
  // Demonstrate the current working approach
  let working_commands = vec![
    (".chat", "Start multi-agent chat session"),
    (".run", "Execute commands with prompts"),
    // Note: .help is already a static command, so we test a different dynamic command
    (".info", "Show application information"),
  ];
  
  println!("📝 Registering commands with EXPLICIT DOT PREFIXES...");
  for (name, description) in &working_commands {
    let cmd = CommandDefinition {
      name: (*name).to_string(), // ← Explicit dot prefix required
      namespace: String::new(),
      description: (*description).to_string(),
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
    auto_help_enabled: false,
    };
    
    let result = registry.command_add_runtime(&cmd, Box::new(demo_handler));
    assert!(result.is_ok(), "Failed to register {name}");
    println!("   ✅ {name} → registered with explicit naming");
  }
  
  // Demonstrate validation prevents invalid commands
  println!("\n🛡️  Testing validation (should reject commands without dot prefix):");
  let invalid_cmd = CommandDefinition {
    name: "invalid_no_dot".to_string(), // No dot prefix
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
  
  let result = registry.command_add_runtime(&invalid_cmd, Box::new(demo_handler));
  assert!(result.is_err(), "Should reject command without dot prefix");
  println!("   ✅ Validation correctly rejected command without dot prefix");
  
  // Also demonstrate namespaced commands work correctly
  let namespaced_cmd = CommandDefinition {
    name: ".list".to_string(), // Explicit dot prefix
    namespace: ".session".to_string(), // Namespace with dot
    description: "List all available sessions".to_string(),
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
    auto_help_enabled: false,
  };
  
  let result = registry.command_add_runtime(&namespaced_cmd, Box::new(demo_handler));
  assert!(result.is_ok(), "Failed to register namespaced command");
  println!("   ✅ .list (namespace: .session) → accessible as .session.list\n");
  
  let pipeline = Pipeline::new(registry);
  
  println!("🧪 Testing command execution...");
  
  // Test all the registered commands work perfectly
  let test_commands = vec![".chat", ".run", ".info", ".session.list"];
  
  for cmd_name in &test_commands {
    let result = pipeline.process_command_simple(cmd_name);
    
    if result.success {
      println!("   ✅ {} executed successfully", cmd_name);
      for output in &result.outputs {
        println!("      {}", output.content);
      }
    } else {
      panic!("❌ Command {cmd_name} failed: {}", 
             result.error.as_ref().unwrap_or(&"unknown".to_string()));
    }
  }
  
  println!("\n🎉 Issue 017 completely resolved!\n");
  
  println!("📋 FINAL SOLUTION SUMMARY:");
  println!("   ✅ Governing Principle: 'Minimum Implicit Magic'");
  println!("   ✅ Explicit Validation: All commands must start with dot");  
  println!("   ✅ No Transformations: Names used exactly as registered");
  println!("   ✅ Fail-Fast: Invalid commands rejected with clear errors");
  println!("   ✅ Predictable Behavior: No hidden magic or transformations\n");
  
  println!("🔧 Developer Guidelines:");
  println!("   • Root command:    name=\".chat\", namespace=\"\" → accessible as .chat");
  println!("   • Namespaced:      name=\".list\", namespace=\".session\" → accessible as .session.list");
  println!("   • NEVER use:       name=\"chat\" (will be rejected by validation)");
  println!("   • Result: Reliable, predictable command behavior\n");
}

/// Verify the fix handles the original issue scenarios perfectly
#[test]
fn verify_issue_017_completely_resolved()
{
  // This test verifies that the exact commands that were failing now work perfectly
  let mut registry = CommandRegistry::new();
  
  // Register the problematic commands using the correct explicit dot prefix approach
  let original_failing_commands = vec![
    (".chat", "Start a multi-agent chat session with Initiative-based turn-taking"),
    (".run", "Execute commands with specified prompts"),
  ];
  
  for (name, description) in &original_failing_commands {
    let cmd = CommandDefinition {
      name: (*name).to_string(), // Explicit dot prefix
      namespace: String::new(), // Empty namespace
      description: (*description).to_string(),
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
    
    let result = registry.command_add_runtime(&cmd, Box::new(demo_handler));
    assert!(result.is_ok(), "Registration should succeed for {name}");
  }
  
  let pipeline = Pipeline::new(registry);
  
  // Test the exact commands that were failing in the original issue
  let previously_failing_commands = vec![".chat", ".run"];
  
  for cmd in &previously_failing_commands {
    let result = pipeline.process_command_simple(cmd);
    
    // These should ALL work perfectly now with our solution
    assert!(result.success, 
           "REGRESSION: Command {cmd} still failing after fix: {}", 
           result.error.as_ref().unwrap_or(&"unknown".to_string()));
           
    // Verify we get the expected success output
    assert!(!result.outputs.is_empty(), "Command {cmd} should produce output");
    
    let output_contains_success = result.outputs.iter()
      .any(|output| output.content.contains("Successfully executed"));
    assert!(output_contains_success, 
           "Command {cmd} should show successful execution");
  }
  
  println!("✅ Issue 017 verification PASSED - all previously failing commands now work flawlessly!");
  println!("✅ Solution: Explicit dot prefix validation with 'Minimum Implicit Magic' principle");
}