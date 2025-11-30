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

/// Test that construction rejects commands without dot prefix (Phase 2 fail-fast)
///
/// **Phase 2 Update:** Validation moved from registration to construction time.
/// Invalid names now panic during `CommandDefinition::former().name()` call.
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_reject_commands_without_dot_prefix()
{
  // Phase 2: This panics at construction time, before registration
  let _invalid_cmd = CommandDefinition::former()
    .name( "chat" ) // ❌ Missing dot prefix - panics here
    .description( "This should be rejected" )
    .end();
}

#[test] 
fn test_reject_invalid_namespace()
{
    let mut registry = CommandRegistry::new();
  
  // This should be REJECTED - namespace without dot prefix
  let mut invalid_cmd = CommandDefinition::former()
    .name( ".list" ) // ✅ Correct name
    .description( "This should be rejected" )
    .end();

  // Manually set invalid namespace after creation
  invalid_cmd.namespace = "session".to_string(); // ❌ Namespace missing dot
  
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
    let mut registry = CommandRegistry::new();
  
  // Root-level command - should be accepted
  let root_cmd = CommandDefinition::former()
    .name( ".test_chat" ) // ✅ Correct dot prefix
    .description( "Correctly formatted root command" )
    .end(); // ✅ Empty namespace for root
  
    let result = registry.command_add_runtime(&root_cmd, Box::new(dummy_handler));
  assert!(result.is_ok(), "Correctly formatted root command should be accepted");
  println!("✅ Accepted correctly formatted root command");
  
  // Namespaced command - should be accepted
  let mut namespaced_cmd = CommandDefinition::former()
    .name( ".list" ) // ✅ Correct dot prefix
    .description( "Correctly formatted namespaced command" )
    .end();

  // Set valid namespace
  namespaced_cmd.namespace = ".session".to_string(); // ✅ Correct namespace with dot
  
    let result2 = registry.command_add_runtime(&namespaced_cmd, Box::new(dummy_handler));
  assert!(result2.is_ok(), "Correctly formatted namespaced command should be accepted");
  println!("✅ Accepted correctly formatted namespaced command");
}

/// Test that valid commands are accepted (Minimum Implicit Magic principle)
///
/// **Principle:** Commands are registered exactly as specified,
/// with no automatic transformations or prefix additions.
#[test]
fn test_principle_minimum_implicit_magic()
{
  println!("\n🎯 TESTING GOVERNING PRINCIPLE: Minimum Implicit Magic");
  println!("   - Commands registered exactly as specified");
  println!("   - No automatic transformations or prefix additions");
  println!("   - Explicit validation with clear error messages");
  println!("   - What you register is exactly what gets executed\n");

    let mut registry = CommandRegistry::new();

  // Test valid command with explicit dot prefix
  let cmd = CommandDefinition::former()
    .name( ".chat" )
    .description( "Testing name: .chat" )
    .end();

    let result = registry.command_add_runtime(&cmd, Box::new(dummy_handler));

  assert!(result.is_ok(), "Command '.chat' should be accepted");
  println!("   {} Command '.chat' correctly accepted", "✅");

  println!("\n🎉 Principle successfully enforced!");
}

/// Test that invalid commands are rejected at construction (Minimum Implicit Magic principle)
///
/// **Phase 2 Update:** Validation moved to construction time.
/// Invalid names panic during `CommandDefinition::former().name()` call.
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_principle_minimum_implicit_magic_rejects_invalid()
{
  // Phase 2: This panics at construction time
  let _invalid_cmd = CommandDefinition::former()
    .name( "chat" ) // ❌ Missing dot prefix - panics here
    .description( "Testing name: chat" )
    .end();
}