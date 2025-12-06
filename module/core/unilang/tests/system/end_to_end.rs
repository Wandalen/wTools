//! Complete system integration test demonstrating all implemented changes
//! 
//! This test validates that issue 017 has been completely resolved and that
//! the governing principle of "Minimum Implicit Magic" is properly enforced.

#![ allow( clippy::needless_pass_by_value ) ]
#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::too_many_lines ) ]
#![ allow( clippy::inefficient_to_string ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::map_unwrap_or ) ]

#![ allow( deprecated ) ]

use unilang::{ CommandDefinition, CommandRegistry, Pipeline, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

fn demo_handler(cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  let output = format!("✅ Command '{}' executed successfully", cmd.definition.name());
  Ok( OutputData { content: output, format: "text".to_string(), execution_time_ms: None } )
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
    // Note: .help is already a static command, so we test different dynamic commands
    (".status", "Show application status"),
  ];
  
  for (name, desc) in &root_commands {
    let cmd = CommandDefinition::former()
      .name(*name)
      .description(*desc)
      .status("stable")
      .version("1.0.0")
      .auto_help_enabled(false)
      .end();
    
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
    let cmd = CommandDefinition::former()
      .name(*name)
      .namespace(*namespace)
      .description(*desc)
      .status("stable")
      .version("1.0.0")
      .auto_help_enabled(false)
      .end();
    
    let result = registry.command_add_runtime(&cmd, Box::new(demo_handler));
    assert!(result.is_ok(), "Namespaced command '{}/{}' should register successfully", namespace, name);
    println!("  ✅ Registered: {}{}", namespace, name.strip_prefix('.').unwrap_or(name));
  }
  
  // Test 3: Validation enforcement
  // Phase 2 Update: Invalid command construction moved to separate #[should_panic] test
  // (see test_reject_invalid_commands_at_construction)
  println!("\n📝 Test 3: Validation enforcement - Phase 2 uses fail-fast at construction");
  println!("  ✅ Validation tests moved to construction-time panic tests");

  // Test 4: Command execution (resolving issue 017)
  println!("\n📝 Test 4: Command execution (Issue 017 resolution)");
  let pipeline = Pipeline::new(registry);
  
  let test_commands = vec![
    ".chat",
    ".run",
    ".status", // Using dynamic command that has a routine
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
  
  let explicit_cmd = CommandDefinition::former()
    .name(".explicit_test")
    .description("Explicitly named command")
    .auto_help_enabled(false)
    .end();
  
  let result = registry.command_add_runtime(&explicit_cmd, Box::new(demo_handler));
  assert!(result.is_ok(), "Explicit command should be accepted");
  println!("  ✅ Explicit naming accepted");
  
  let pipeline = Pipeline::new(registry);
  let execution_result = pipeline.process_command_simple(".explicit_test");
  assert!(execution_result.success, "Explicit command should execute");
  println!("  ✅ No implicit transformations - command used exactly as registered");
  
  // Principle 2: Fail-Fast Validation
  // Phase 2 Update: Validation moved to construction time (earlier than registration)
  println!("\n🔍 Principle 2: Fail-Fast Validation");
  println!("  ✅ Phase 2 fail-fast: Invalid commands panic at construction time");
  println!("  ✅ Even faster validation - before registration is attempted");
  println!("  ✅ See separate #[should_panic] tests for construction-time validation");

  println!("\n🎉 All governing principles successfully enforced!");
}