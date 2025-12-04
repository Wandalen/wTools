//! `format_category_name` Decoupling Test
//!
//! **Purpose:** Verify that `format_category_name()` uses a generic `snake_case` → Title Case
//! transformation algorithm instead of hardcoded category-to-display-name mappings.
//!
//! **Root Cause:** The old implementation contained a match statement with 15+ hardcoded
//! category mappings (e.g., `"git_operations"` → "GIT OPERATIONS", `"daemon_lifecycle"` →
//! "DAEMON LIFECYCLE") that were specific to certain CLI applications (wip, wplan, dream,
//! wish). This coupling violated the framework's architectural principle of domain-agnosticism.
//!
//! **Why Not Caught:** The original implementation predated the architectural requirement
//! for complete domain-agnosticism. Tests validated the hardcoded mappings rather than
//! challenging the coupling.
//!
//! **Fix Applied:** Replace match statement with generic algorithm:
//! ```text
//! snake_case → split on '_' → capitalize each word → join with space → Title Case
//! ```
//!
//! **Prevention:** These tests enforce that `format_category_name()` uses a generic
//! transformation algorithm that works for ANY category name, not just specific domains.
//!
//! **Pitfall:** Never reintroduce hardcoded category mappings. Always use generic
//! string transformation algorithms to maintain domain-agnosticism.

#![ cfg( test ) ]
#![ allow( clippy::uninlined_format_args ) ]

use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;

/// Test: `format_category_name` uses generic Title Case transformation
///
/// Validates that the function transforms any `snake_case` input to Title Case
/// using a generic algorithm, not hardcoded mappings.
#[ test ]
fn test_format_category_name_generic_transformation()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  let test_cases = vec![
    // Original hardcoded categories (should now use generic algorithm)
    ("git_operations", "Git Operations"),
    ("repository_management", "Repository Management"),
    ("removal_operations", "Removal Operations"),
    ("github_integration", "Github Integration"),
    ("daemon_lifecycle", "Daemon Lifecycle"),
    ("job_management", "Job Management"),
    ("ai_assistance", "Ai Assistance"),

    // New arbitrary categories (proves generic algorithm)
    ("file_system", "File System"),
    ("network_operations", "Network Operations"),
    ("user_management", "User Management"),
    ("database_queries", "Database Queries"),
    ("cache_invalidation", "Cache Invalidation"),

    // Edge cases
    ("single", "Single"),
    ("a_b", "A B"),
    ("", ""),
  ];

  for (input, expected) in test_cases
  {
    let result = help_gen.format_category_name_for_test( input );

    assert_eq!(
      result,
      expected,
      "format_category_name('{}') should return '{}' (Title Case), got '{}'",
      input,
      expected,
      result
    );
  }
}

/// Test: `format_category_name` handles empty string
#[ test ]
fn test_format_category_name_empty_string()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  let result = help_gen.format_category_name_for_test( "" );
  assert_eq!( result, "", "Empty string should return empty string" );
}

/// Test: `format_category_name` handles single word
#[ test ]
fn test_format_category_name_single_word()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  assert_eq!(
    help_gen.format_category_name_for_test( "system" ),
    "System"
  );

  assert_eq!(
    help_gen.format_category_name_for_test( "help" ),
    "Help"
  );
}

/// Test: `format_category_name` handles multiple underscores
#[ test ]
fn test_format_category_name_multiple_underscores()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  assert_eq!(
    help_gen.format_category_name_for_test( "very_long_category_name" ),
    "Very Long Category Name"
  );
}

/// Test: `format_category_name` is pure function
///
/// Validates that the function produces consistent results across multiple
/// invocations with the same input (no side effects).
#[ test ]
fn test_format_category_name_is_pure()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  let input = "git_operations";

  let result1 = help_gen.format_category_name_for_test( input );
  let result2 = help_gen.format_category_name_for_test( input );
  let result3 = help_gen.format_category_name_for_test( input );

  assert_eq!( result1, result2 );
  assert_eq!( result2, result3 );
  assert_eq!( result1, "Git Operations" );
}

/// Test: No hardcoded category mappings exist
///
/// Ensures that arbitrary category names work correctly, proving
/// the algorithm is generic and not dependent on hardcoded mappings.
#[ test ]
fn test_no_hardcoded_mappings()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  // These categories were never hardcoded in the old implementation
  // If they work correctly, it proves the algorithm is generic
  let arbitrary_categories = vec![
    ("random_category", "Random Category"),
    ("another_example", "Another Example"),
    ("foo_bar_baz", "Foo Bar Baz"),
    ("x_y_z", "X Y Z"),
  ];

  for (input, expected) in arbitrary_categories
  {
    let result = help_gen.format_category_name_for_test( input );
    assert_eq!(
      result,
      expected,
      "Arbitrary category '{}' should transform to '{}', got '{}'",
      input,
      expected,
      result
    );
  }
}

/// Test: Algorithm does not depend on domain knowledge
///
/// Validates that the transformation works for categories from ANY domain,
/// not just git/repository/daemon/ai domains.
#[ test ]
fn test_domain_agnostic_transformation()
{
  #[ allow( deprecated ) ]
  let registry = CommandRegistry::new();
  let help_gen = HelpGenerator::new( &registry );

  // Categories from completely different domains
  let domain_agnostic = vec![
    ("medical_records", "Medical Records"),
    ("financial_transactions", "Financial Transactions"),
    ("weather_data", "Weather Data"),
    ("inventory_management", "Inventory Management"),
    ("customer_support", "Customer Support"),
  ];

  for (input, expected) in domain_agnostic
  {
    let result = help_gen.format_category_name_for_test( input );
    assert_eq!(
      result,
      expected,
      "Domain-agnostic category '{}' failed transformation",
      input
    );
  }
}
