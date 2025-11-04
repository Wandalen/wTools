//! Basic integration test for registry integration functionality
//!
//! This test validates that the `CommandRegistry::from_static_commands()` method
//! works correctly and that both `CommandRegistry` and `StaticCommandRegistry`
//! implement the `CommandRegistryTrait` properly.

#![ allow( deprecated ) ]

use unilang::registry::{ CommandRegistry, CommandRegistryTrait };
use unilang::static_data::{ StaticCommandDefinition, StaticCommandMap };
use phf::phf_map;

// Create a test static command map for testing (internal implementation)
const TEST_STATIC_COMMANDS_INTERNAL: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {
  ".test_static" => &StaticCommandDefinition {
    name: ".test_static",
    namespace: "",
    description: "Test static command",
    hint: "A test command",
    arguments: &[],
    routine_link: None,
    status: "active",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: false,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[],
  },
};

/// Public wrapper for test static commands
static TEST_STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&TEST_STATIC_COMMANDS_INTERNAL);

#[test]
fn test_command_registry_from_static_commands_basic() {
  // Test that CommandRegistry can be created from static commands
  let registry = CommandRegistry::from_static_commands(&TEST_STATIC_COMMANDS);

  // The registry should contain the test static command
  let all_commands = registry.commands();

  // Should have our test command (at least 1)
  assert!(!all_commands.is_empty(), "Registry should contain static commands");

  // Verify that the static command can be retrieved
  let cmd = registry.command(".test_static").expect("Test static command should exist");
  assert_eq!(cmd.name().to_string(), ".test_static", "Command name should match");
  assert_eq!(cmd.description().to_string(), "Test static command", "Command description should match");
}

#[test]
fn test_command_registry_trait_implementation() {
  let registry = CommandRegistry::new();

  // Test that CommandRegistry implements CommandRegistryTrait
  let _: &dyn CommandRegistryTrait = &registry;

  // Test trait methods work
  let _commands = registry.commands();
  // Note: CommandRegistry may include global static commands, so we can't assume it's empty

  // Test that non-existent command returns None
  assert!(registry.command("nonexistent").is_none());
  assert!(registry.get_help_for_command("nonexistent").is_none());
}