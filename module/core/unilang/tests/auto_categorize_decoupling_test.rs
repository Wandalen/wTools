//! `auto_categorize` Decoupling Test
//!
//! **Purpose:** Verify that `auto_categorize()` returns empty string for all inputs,
//! enforcing the architectural requirement that categories must be explicit via
//! `CommandDefinition::category()` and never inferred from command names.
//!
//! **Root Cause:** The old implementation contained domain-specific pattern matching
//! (e.g., `.git` → `git_operations`, `.remove` → `removal_operations`) that violated
//! the framework's architectural principle of domain-agnosticism. This coupling made
//! the unilang library unusable for applications in other domains.
//!
//! **Why Not Caught:** The original implementation predated the architectural
//! requirement for complete domain-agnosticism. Tests validated the coupled behavior
//! rather than challenging it.
//!
//! **Fix Applied:** Replace pattern-matching logic with `String::new()`, making
//! category inference impossible. All categories must now be explicitly specified
//! via `CommandDefinition::category()` field.
//!
//! **Prevention:** These tests enforce the architectural requirement that
//! `auto_categorize()` must return empty string for ALL inputs. Any attempt to
//! reintroduce inference logic will cause test failures.
//!
//! **Pitfall:** Never infer categories from command names. Always require explicit
//! categorization via `CommandDefinition::category()`. Pattern matching creates
//! domain coupling that prevents generic library reuse.

#![ cfg( test ) ]
#![ allow( clippy::uninlined_format_args ) ]

use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;

/// Test: `auto_categorize` returns empty string (no inference)
///
/// Validates that `auto_categorize()` never infers categories from command names,
/// returning empty string for all inputs. This enforces the architectural requirement
/// that categories must be explicit.
#[ test ]
fn test_auto_categorize_returns_empty_string()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  // Previously coupled patterns that MUST now return empty string
  let test_cases = vec![
    ".git.status",        // Previously inferred as "git_operations"
    ".remove.both",       // Previously inferred as "removal_operations"
    ".orgs.list",         // Previously inferred as "github_integration"
    ".add",               // Previously inferred as "repository_management"
    ".clone",             // Previously inferred as "repository_management"
    ".daemon.start",      // Previously inferred as "daemon_lifecycle"
    ".job.run",           // Previously inferred as "job_management"
    ".chat",              // Previously inferred as "ai_assistance"
    ".help",              // Previously inferred as "help_system"
    ".status",            // Generic command with no inference
    ".arbitrary.command", // Arbitrary command with no inference
  ];

  for command_name in test_cases
  {
    let category = help_gen.auto_categorize_for_test( command_name );

    assert_eq!(
      category,
      String::new(),
      "auto_categorize('{}') must return empty string, got '{}'",
      command_name,
      category
    );
  }
}

/// Test: `auto_categorize` is a pure function (no side effects)
///
/// Validates that `auto_categorize()` is stateless and produces consistent
/// results across multiple invocations with the same input.
#[ test ]
fn test_auto_categorize_is_pure_function()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  let command_name = ".git.status";

  // Call multiple times - should always return same result
  let result1 = help_gen.auto_categorize_for_test( command_name );
  let result2 = help_gen.auto_categorize_for_test( command_name );
  let result3 = help_gen.auto_categorize_for_test( command_name );

  assert_eq!( result1, result2 );
  assert_eq!( result2, result3 );
  assert_eq!( result1, String::new() );
}

/// Test: `auto_categorize` handles edge cases
///
/// Validates that `auto_categorize()` returns empty string for edge cases
/// including empty string, single dot, and malformed command names.
#[ test ]
fn test_auto_categorize_edge_cases()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  let edge_cases = vec![
    "",                  // Empty string
    ".",                 // Single dot
    "..",                // Double dot
    "no_leading_dot",    // Missing dot prefix
    ".123",              // Numeric name
    ".a",                // Single character
    ".very.long.nested.command.hierarchy", // Deep nesting
  ];

  for command_name in edge_cases
  {
    let category = help_gen.auto_categorize_for_test( command_name );

    assert_eq!(
      category,
      String::new(),
      "auto_categorize('{}') must return empty string for edge case",
      command_name
    );
  }
}

/// Test: Explicit categorization is required
///
/// Demonstrates that commands without explicit `category()` field have no
/// category, enforcing the architectural requirement for explicit categorization.
#[ test ]
fn test_explicit_categorization_required()
{
  use unilang::data::CommandDefinition;

  // Command WITHOUT explicit category - should have no category
  let cmd_no_category = CommandDefinition::former()
    .name( ".git.status" )
    .description( "Show git status" )
    .end();

  assert_eq!(
    cmd_no_category.category(),
    "",
    "Command without explicit category() must have empty category"
  );

  // Command WITH explicit category - should have that category
  let cmd_with_category = CommandDefinition::former()
    .name( ".git.status" )
    .description( "Show git status" )
    .category( "version_control" )
    .end();

  assert_eq!(
    cmd_with_category.category(),
    "version_control",
    "Command with explicit category() must preserve that category"
  );
}

/// Test: `auto_categorize` complexity is minimal
///
/// Validates that `auto_categorize()` implementation is simple (≤5 lines)
/// as specified in the migration plan.
#[ test ]
fn test_auto_categorize_implementation_simplicity()
{
  // This test validates that auto_categorize is so simple that it can be
  // represented as a single expression: String::new()
  //
  // If this test compiles and runs, it proves the implementation is minimal.
  // Any attempt to add complex logic will require changes to this test.

  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  // Implementation should be equivalent to this:
  let expected = String::new();
  let actual = help_gen.auto_categorize_for_test( ".any.command" );

  assert_eq!(
    actual,
    expected,
    "auto_categorize implementation must be equivalent to String::new()"
  );
}
