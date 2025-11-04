//! Tests for Row 7: Rust DSL → Dynamic `HashMap` with inline closures.
//!
//! This module tests the `command_with_routine()` builder method that enables
//! inline closure registration for maximum development flexibility.
//!
//! **Test Matrix for Inline Closure Registration**
//!
//! | Test | Category | Validates | Status |
//! |------|----------|-----------|--------|
//! | IC1.1 | Basic Registration | Single inline closure command | ✅ Tested |
//! | IC1.2 | Basic Registration | Command execution with inline closure | ✅ Tested |
//! | IC2.1 | Multiple Commands | Multiple inline closures in chain | ✅ Tested |
//! | IC2.2 | Multiple Commands | Each closure executes independently | ✅ Tested |
//! | IC3.1 | Auto Help | Help command auto-generated for inline | ✅ Tested |
//! | IC3.2 | Auto Help | Help command execution | ✅ Tested |
//! | IC4.1 | Closure Capture | Closure captures external variables | ✅ Tested |
//! | IC4.2 | Closure Capture | Multiple closures with different captures | ✅ Tested |
//! | IC5.1 | Builder Integration | Mix YAML + inline closures | ✅ Tested |
//! | IC5.2 | Builder Integration | Mix JSON + inline closures | ✅ Tested |
//! | IC6.1 | Error Handling | Invalid command name (no dot) | ✅ Tested |
//! | IC6.2 | Error Handling | Builder chain continues after error | ✅ Tested |
//! | IC7.1 | Command Lookup | Inline command found in registry | ✅ Tested |
//! | IC7.2 | Command Lookup | Command metadata correct | ✅ Tested |

use unilang::
{
  registry::CommandRegistry,
  interpreter::ExecutionContext,
  data::OutputData,
  semantic::VerifiedCommand,
};

// ============================================================================
// IC1: Basic Registration Tests
// ============================================================================

#[ test ]
fn test_ic1_1_single_inline_closure_registration()
{
  // Test Matrix Row: IC1.1
  // Register single command with inline closure

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".greet",
      "Greets the user",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Hello, World!".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  // Command should be registered
  let cmd = registry.command( ".greet" );
  assert!( cmd.is_some(), "Command should be registered" );

  let cmd_def = cmd.unwrap();
  assert_eq!( cmd_def.name().to_string(), ".greet" );
  assert_eq!( cmd_def.description().to_string(), "Greets the user" );
  assert!(matches!(cmd_def.status(), unilang::data::CommandStatus::Active));
}

#[ test ]
fn test_ic1_2_inline_closure_execution()
{
  // Test Matrix Row: IC1.2
  // Execute command with inline closure

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".echo",
      "Echoes input",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Echo response".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  let cmd_def = registry.command( ".echo" ).expect( "Command not found" );
  let routine = registry.get_routine( ".echo" ).expect( "Routine not found" );

  // Create mock VerifiedCommand and ExecutionContext for testing
  let verified_cmd = VerifiedCommand
  {
    definition : cmd_def.clone(),
    arguments : std::collections::HashMap::new(),
  };

  let ctx = ExecutionContext::default();

  // Execute the routine
  let result = routine( verified_cmd, ctx );
  assert!( result.is_ok() );

  let output = result.unwrap();
  assert_eq!( output.content, "Echo response" );
  assert_eq!( output.format, "text" );
}

// ============================================================================
// IC2: Multiple Commands Tests
// ============================================================================

#[ test ]
fn test_ic2_1_multiple_inline_closures_chain()
{
  // Test Matrix Row: IC2.1
  // Register multiple inline closures in builder chain

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".cmd1",
      "First command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Output 1".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".cmd2",
      "Second command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Output 2".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".cmd3",
      "Third command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Output 3".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  // All commands should be registered
  assert!( registry.command( ".cmd1" ).is_some() );
  assert!( registry.command( ".cmd2" ).is_some() );
  assert!( registry.command( ".cmd3" ).is_some() );
}

#[ test ]
fn test_ic2_2_each_closure_executes_independently()
{
  // Test Matrix Row: IC2.2
  // Each closure should execute its own logic independently

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".add",
      "Adds numbers",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "42".to_string(),
          format : "number".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".multiply",
      "Multiplies numbers",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "100".to_string(),
          format : "number".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  let ctx = ExecutionContext::default();

  // Execute .add
  let add_routine = registry.get_routine( ".add" ).expect( "Add routine not found" );
  let add_cmd_def = registry.command( ".add" ).expect( "Add command not found" );
  let add_result = add_routine( VerifiedCommand
  {
    definition : add_cmd_def,
    arguments : std::collections::HashMap::new(),
  }, ctx.clone() );
  assert_eq!( add_result.unwrap().content, "42" );

  // Execute .multiply
  let mul_routine = registry.get_routine( ".multiply" ).expect( "Multiply routine not found" );
  let mul_cmd_def = registry.command( ".multiply" ).expect( "Multiply command not found" );
  let mul_result = mul_routine( VerifiedCommand
  {
    definition : mul_cmd_def,
    arguments : std::collections::HashMap::new(),
  }, ctx.clone() );
  assert_eq!( mul_result.unwrap().content, "100" );
}

// ============================================================================
// IC3: Auto Help Generation Tests
// ============================================================================

#[ test ]
fn test_ic3_1_auto_help_generated_for_inline()
{
  // Test Matrix Row: IC3.1
  // Help commands should be auto-generated for inline closures

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".test",
      "Test command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Test output".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  // Base command should exist
  assert!( registry.command( ".test" ).is_some() );

  // Help command should be auto-generated
  let help_cmd = registry.command( ".test.help" );
  assert!( help_cmd.is_some(), "Help command should be auto-generated" );

  let help_def = help_cmd.unwrap();
  // The full name is .test.help (namespace + name)
  assert_eq!( help_def.full_name(), ".test.help" );
}

#[ test ]
fn test_ic3_2_help_command_execution()
{
  // Test Matrix Row: IC3.2
  // Help command should execute and return help text

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".example",
      "Example command for help test",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Example output".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  let help_routine = registry.get_routine( ".example.help" ).expect( "Help routine not found" );
  let help_cmd_def = registry.command( ".example.help" ).expect( "Help command not found" );

  let ctx = ExecutionContext::default();

  let help_result = help_routine( VerifiedCommand
  {
    definition : help_cmd_def,
    arguments : std::collections::HashMap::new(),
  }, ctx );

  assert!( help_result.is_ok() );
  let help_output = help_result.unwrap();

  // Help output should contain command description
  assert!( help_output.content.contains( "Example command for help test" ) );
}

// ============================================================================
// IC4: Closure Capture Tests
// ============================================================================

#[ test ]
fn test_ic4_1_closure_captures_external_variables()
{
  // Test Matrix Row: IC4.1
  // Closures should be able to capture external variables

  let message = "Captured message".to_string();

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".capture",
      "Captures external variable",
      move |_cmd, _ctx| {
        Ok( OutputData
        {
          content : message.clone(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  let routine = registry.get_routine( ".capture" ).expect( "Routine not found" );
  let cmd_def = registry.command( ".capture" ).expect( "Command not found" );

  let ctx = ExecutionContext::default();

  let result = routine( VerifiedCommand
  {
    definition : cmd_def,
    arguments : std::collections::HashMap::new(),
  }, ctx );

  assert_eq!( result.unwrap().content, "Captured message" );
}

#[ test ]
fn test_ic4_2_multiple_closures_different_captures()
{
  // Test Matrix Row: IC4.2
  // Each closure can capture different variables

  let msg1 = "First".to_string();
  let msg2 = "Second".to_string();

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".first",
      "First captured",
      move |_cmd, _ctx| {
        Ok( OutputData
        {
          content : msg1.clone(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".second",
      "Second captured",
      move |_cmd, _ctx| {
        Ok( OutputData
        {
          content : msg2.clone(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  let ctx = ExecutionContext::default();

  let first_routine = registry.get_routine( ".first" ).unwrap();
  let first_cmd_def = registry.command( ".first" ).unwrap();
  let first_result = first_routine( VerifiedCommand
  {
    definition : first_cmd_def,
    arguments : std::collections::HashMap::new(),
  }, ctx.clone() );
  assert_eq!( first_result.unwrap().content, "First" );

  let second_routine = registry.get_routine( ".second" ).unwrap();
  let second_cmd_def = registry.command( ".second" ).unwrap();
  let second_result = second_routine( VerifiedCommand
  {
    definition : second_cmd_def,
    arguments : std::collections::HashMap::new(),
  }, ctx.clone() );
  assert_eq!( second_result.unwrap().content, "Second" );
}

// ============================================================================
// IC5: Builder Integration Tests
// ============================================================================

#[ test ]
fn test_ic5_1_mix_yaml_and_inline_closures()
{
  // Test Matrix Row: IC5.1
  // Should be able to mix YAML commands and inline closures

  let yaml_content = r#"
- name: ".yaml_cmd"
  namespace: ""
  description: "YAML command"
  hint: "From YAML"
  status: "stable"
  version: "1.0.0"
  arguments: []
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
  auto_help_enabled: true
"#;

  let registry = CommandRegistry::builder()
    .load_from_yaml_str( yaml_content )
    .expect( "YAML parsing failed" )
    .command_with_routine(
      ".inline_cmd",
      "Inline command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Inline output".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  // Both commands should exist
  assert!( registry.command( ".yaml_cmd" ).is_some() );
  assert!( registry.command( ".inline_cmd" ).is_some() );

  // Only inline command should have routine
  assert!( registry.get_routine( ".yaml_cmd" ).is_none() );
  assert!( registry.get_routine( ".inline_cmd" ).is_some() );
}

#[ test ]
fn test_ic5_2_mix_json_and_inline_closures()
{
  // Test Matrix Row: IC5.2
  // Should be able to mix JSON commands and inline closures

  let json_content = r#"
[
  {
    "name": ".json_cmd",
    "namespace": "",
    "description": "JSON command",
    "hint": "From JSON",
    "status": "stable",
    "version": "1.0.0",
    "arguments": [],
    "tags": [],
    "aliases": [],
    "permissions": [],
    "idempotent": true,
    "deprecation_message": "",
    "http_method_hint": "GET",
    "examples": [],
    "auto_help_enabled": true
  }
]
"#;

  let registry = CommandRegistry::builder()
    .load_from_json_str( json_content )
    .expect( "JSON parsing failed" )
    .command_with_routine(
      ".inline_cmd",
      "Inline command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Inline output".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  // Both commands should exist
  assert!( registry.command( ".json_cmd" ).is_some() );
  assert!( registry.command( ".inline_cmd" ).is_some() );

  // Only inline command should have routine
  assert!( registry.get_routine( ".json_cmd" ).is_none() );
  assert!( registry.get_routine( ".inline_cmd" ).is_some() );
}

// ============================================================================
// IC6: Error Handling Tests
// ============================================================================

/// Test that invalid command names are rejected during builder registration (Phase 2 fail-fast)
///
/// **Phase 2 Update:** Validation moved to construction time.
/// Invalid names panic in `command_with_routine()` when creating internal `CommandDefinition`.
#[ test ]
#[should_panic(expected = "MissingDotPrefix")]
fn test_ic6_1_invalid_command_name_logs_error()
{
  // Test Matrix Row: IC6.1
  // Phase 2: This panics during builder call

  let _registry = CommandRegistry::builder()
    .command_with_routine(
      "invalid", // ❌ Missing dot prefix - panics here
      "Invalid command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Should not execute".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();
}

/// Test that invalid commands panic during builder chain (Phase 2 fail-fast)
///
/// **Phase 2 Update:** Builder chain doesn't continue after validation errors.
/// Invalid names panic immediately in first `command_with_routine()` call.
#[ test ]
#[should_panic(expected = "MissingDotPrefix")]
fn test_ic6_2_builder_chain_continues_after_error()
{
  // Test Matrix Row: IC6.2
  // Phase 2: This panics on first invalid command

  let _registry = CommandRegistry::builder()
    .command_with_routine(
      "invalid1", // ❌ Missing dot prefix - panics here
      "Invalid 1",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Bad 1".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .command_with_routine(
      ".valid",
      "Valid command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Good".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();
}

// ============================================================================
// IC7: Command Lookup Tests
// ============================================================================

#[ test ]
fn test_ic7_1_inline_command_found_in_registry()
{
  // Test Matrix Row: IC7.1
  // Inline commands should be findable via standard lookup

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".lookup_test",
      "Lookup test command",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Found".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  // Should be found by full name
  let cmd = registry.command( ".lookup_test" );
  assert!( cmd.is_some() );

  // Should also be in command list
  let all_commands = registry.commands();
  assert!( all_commands.values().any( |c| c.name().as_str() == ".lookup_test" ) );
}

#[ test ]
fn test_ic7_2_command_metadata_correct()
{
  // Test Matrix Row: IC7.2
  // Command metadata should reflect registration parameters

  let registry = CommandRegistry::builder()
    .command_with_routine(
      ".metadata_test",
      "This is the description",
      |_cmd, _ctx| {
        Ok( OutputData
        {
          content : "Output".to_string(),
          format : "text".to_string(),
      execution_time_ms : None,
        })
      }
    )
    .build();

  let cmd = registry.command( ".metadata_test" ).unwrap();

  // Check all metadata fields
  assert_eq!( cmd.name().to_string(), ".metadata_test" );
  assert_eq!( cmd.namespace(), "" ); // Empty by default
  assert_eq!( cmd.description().to_string(), "This is the description" );
  assert!(matches!(cmd.status(), unilang::data::CommandStatus::Active));
  assert_eq!( cmd.version().to_string(), "1.0.0" );
  assert!( cmd.auto_help_enabled() ); // Should be true by default
  assert!( cmd.idempotent() ); // Should be true by default
  assert_eq!( cmd.http_method_hint(), "GET" ); // GET is the default
}
