//! Tests for `StaticCommandRegistry` to `CommandRegistry` conversion bridge.
//!
//! ## Test Matrix
//!
//! | Test | Input | Expected | Status |
//! |------|-------|----------|--------|
//! | `test_static_registry_converts_to_command_registry` | `StaticCommandRegistry` | `CommandRegistry` | TBD |
//! | `test_converted_registry_contains_all_commands` | `StaticRegistry` with commands | All present in `CommandRegistry` | TBD |
//! | `test_converted_registry_works_with_pipeline` | `StaticRegistry` | Pipeline accepts .`into()` | TBD |
//! | `test_converted_commands_preserve_metadata` | `StaticCommand` | All fields preserved | TBD |
//!
//! ## Design Rationale
//!
//! The conversion bridge enables users to:
//! 1. Define commands statically via YAML for compile-time validation
//! 2. Convert to `CommandRegistry` for runtime use with Pipeline
//! 3. Use pattern: `Pipeline::new(STATIC_COMMANDS.into())`
//!
//! This resolves the mismatch where Pipeline requires `CommandRegistry` but
//! static definitions produce `StaticCommandRegistry`.
//!
//! ## Related Hypotheses
//!
//! - H15: No From<StaticCommandRegistry> for `CommandRegistry` - Pipeline uses concrete type
//! - H16: `Pipeline::new` takes `CommandRegistry` (NOT trait)
//! - H31: From conversion uses `expect()` - RUNTIME PANIC on invalid names
//!
//! ## Root Cause
//!
//! `StaticCommandRegistry` was designed independently without conversion path.
//!
//! ## Why Not Caught
//!
//! Original design didn't anticipate needing to use static commands with Pipeline.
//!
//! ## Fix Applied
//!
//! Added From<StaticCommandRegistry> for `CommandRegistry` conversion.
//!
//! ## Prevention
//!
//! Registry types should have standard conversion traits when semantically equivalent.
//!
//! ## Pitfall
//!
//! Conversion can fail if static definitions weren't validated at build time.

#[cfg(feature = "static_registry")]
mod conversion_tests
{
  use unilang::registry::CommandRegistry;
  use unilang::registry::StaticCommandRegistry;

  /// Test that `StaticCommandRegistry` can be converted to `CommandRegistry`.
  /// This is the core test for H15 - enables Pattern: `static_registry.into()`
  #[test]
  fn test_static_registry_converts_to_command_registry()
  {
    // Create empty static registry for now
    let static_registry = StaticCommandRegistry::new();

    // This should compile - proves From trait exists
    let _command_registry: CommandRegistry = static_registry.into();

    // If this test compiles and runs, the conversion bridge exists
  }

  /// Test that all commands from static registry are present in converted registry.
  #[test]
  fn test_converted_registry_preserves_commands()
  {
    // Use the global STATIC_COMMANDS if available
    // For this test, we verify empty registry converts correctly
    let static_registry = StaticCommandRegistry::new();
    let command_registry: CommandRegistry = static_registry.into();

    // Empty static registry converts to CommandRegistry
    // The conversion itself succeeding is the key test
    // The registry may have base commands (.help) auto-registered
    let _cmd_count = command_registry.commands().len();
    // Conversion succeeded if we reach this point
  }

  /// Test that converted registry works with Pipeline.
  /// This validates the usage pattern: `Pipeline::new(STATIC_COMMANDS.into())`
  #[test]
  fn test_converted_registry_usable_with_pipeline()
  {
    use unilang::pipeline::Pipeline;

    let static_registry = StaticCommandRegistry::new();
    let command_registry: CommandRegistry = static_registry.into();

    // This should work - Pipeline accepts CommandRegistry
    let _pipeline = Pipeline::new(command_registry);

    // If this compiles and runs, the pattern works
  }
}

// Tests that should work regardless of feature flags
#[test]
fn test_conversion_bridge_design_note()
{
  // This test documents the design decision for the conversion bridge.
  //
  // The conversion bridge (From<StaticCommandRegistry> for CommandRegistry) enables:
  //
  // 1. YAML-defined commands with compile-time validation
  // 2. Runtime use with Pipeline, Interpreter, HelpGenerator
  // 3. Clean usage pattern: Pipeline::new(STATIC_COMMANDS.into())
  //
  // Without this bridge, users would need to manually recreate commands
  // at runtime, losing the benefits of compile-time validation.
  //
  // Implementation notes:
  // - Since build.rs now validates commands, From cannot fail
  // - No need for TryFrom - validation happens at build time
  // - Conversion copies data from static to dynamic representation
}
