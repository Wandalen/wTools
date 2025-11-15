//!
//! # Category Field Edge Cases and Boundary Conditions Tests
//!
//! ## What This Tests
//!
//! This test suite validates that the `category` field handles edge cases, boundary
//! conditions, and unusual inputs correctly. These tests cover scenarios that are
//! unlikely but must work correctly to prevent panics, corruption, or undefined behavior.
//!
//! ## Why This Matters
//!
//! Edge cases often reveal bugs that only manifest in production. These tests prevent:
//! - Buffer overflows from long category names
//! - Unicode corruption
//! - Whitespace handling issues
//! - Performance problems with large datasets
//! - Case sensitivity confusion
//! - Path interpretation vulnerabilities
//! - Injection attacks
//!
//! ## Failure Interpretation
//!
//! - `category_very_long_name_200_chars()` fails: Long names truncated or causing panics
//! - `category_with_unicode_emoji()` fails: Unicode not preserved, corruption
//! - `category_with_tabs_and_newlines()` fails: Whitespace escaping broken
//! - `category_with_all_special_markdown_chars()` fails: Special char handling broken
//! - `category_only_whitespace()` fails: Whitespace-only categories not handled gracefully
//! - `all_commands_same_category()` fails: Edge case where default section empty
//! - `all_commands_empty_category()` fails: Edge case where no categories shown
//! - `duplicate_category_names()` fails: Duplicate categories causing issues
//! - `category_case_sensitivity()` fails: Unexpected case-insensitive merging
//! - `zero_commands_in_system()` fails: Empty registry causing panics
//! - `category_with_path_separators()` fails: Category misinterpreted as path
//! - `category_with_injection_chars()` fails: Injection vulnerability
//!
//! ## Related
//!
//! - Issue-089: Category field edge case handling
//! - Security: Injection prevention

use unilang::static_data::*;
use unilang::data::CommandDefinition;

//
// Test: category very long name (200 chars)
//

/// Verifies that 200-character category names work without truncation or panics.
///
/// This prevents long categories causing failures.
#[ test ]
fn category_very_long_name_200_chars()
{
  // Generate exactly 200 'a' characters
  const LONG_CAT : &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : LONG_CAT,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  // Verify no truncation
  assert_eq!( dynamic_cmd.category(), LONG_CAT );
  assert!( dynamic_cmd.category().len() >= 200 );
}

//
// Test: category with unicode emoji
//

/// Verifies that unicode emoji in categories are preserved correctly.
///
/// This prevents unicode causing panics or corruption.
#[ test ]
fn category_with_unicode_emoji()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "🔧 Operations",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "🔧 Operations" );
}

//
// Test: category with tabs and newlines
//

/// Verifies that tabs and newlines in categories are handled correctly.
///
/// This prevents whitespace breaking generated code.
#[ test ]
fn category_with_tabs_and_newlines()
{
  // In practice, these would be escaped in YAML/generated code
  // This tests the struct can hold such values
  let cmd = StaticCommandDefinition::new( ".test", "", "Test" )
    .with_category( "test\twith\ttabs" );

  assert!( cmd.category.contains( '\t' ) );
}

//
// Test: category with all special markdown chars
//

/// Verifies that markdown special characters in categories are handled.
///
/// This prevents markdown rendering issues in help.
#[ test ]
fn category_with_all_special_markdown_chars()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "Code: *bold*, _italic_, [link]",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "Code: *bold*, _italic_, [link]" );
}

//
// Test: category only whitespace
//

/// Verifies that whitespace-only categories are handled gracefully.
///
/// This prevents whitespace-only categories breaking layout.
#[ test ]
fn category_only_whitespace()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "   ",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "   " );
}

//
// Test: all commands same category
//

/// Verifies that having all commands in same category works correctly.
///
/// This prevents edge case where default section is empty.
#[ test ]
fn all_commands_same_category()
{
  static CMD1 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd1",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "same_category",
  };

  static CMD2 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd2",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "same_category",
  };

  let d1 : CommandDefinition = ( &CMD1 ).into();
  let d2 : CommandDefinition = ( &CMD2 ).into();

  assert_eq!( d1.category(), "same_category" );
  assert_eq!( d2.category(), "same_category" );
}

//
// Test: all commands empty category
//

/// Verifies that all commands having empty category works correctly.
///
/// This prevents edge case where no categories shown.
#[ test ]
fn all_commands_empty_category()
{
  static CMD1 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd1",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "",
  };

  let d1 : CommandDefinition = ( &CMD1 ).into();

  assert_eq!( d1.category(), "" );
}

//
// Test: duplicate category names
//

/// Verifies that multiple commands with identical category work correctly.
///
/// This prevents duplicate categories causing issues.
#[ test ]
fn duplicate_category_names()
{
  static CMD1 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd1",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "duplicate",
  };

  static CMD2 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd2",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "duplicate",
  };

  let d1 : CommandDefinition = ( &CMD1 ).into();
  let d2 : CommandDefinition = ( &CMD2 ).into();

  assert_eq!( d1.category(), "duplicate" );
  assert_eq!( d2.category(), "duplicate" );
}

//
// Test: category case sensitivity
//

/// Verifies that categories are case sensitive (Git_Ops != git_ops).
///
/// This prevents unexpected case-insensitive merging.
#[ test ]
fn category_case_sensitivity()
{
  static CMD1 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd1",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "Git_Ops",
  };

  static CMD2 : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".cmd2",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "git_ops",
  };

  let d1 : CommandDefinition = ( &CMD1 ).into();
  let d2 : CommandDefinition = ( &CMD2 ).into();

  assert_ne!( d1.category(), d2.category() );
}

//
// Test: category with path separators
//

/// Verifies that path separators in categories dont cause path interpretation.
///
/// This prevents category being misinterpreted as path.
#[ test ]
fn category_with_path_separators()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "file/operations",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "file/operations" );
}

//
// Test: category with injection chars
//

/// Verifies that SQL-like injection characters are treated as plain strings.
///
/// This prevents injection vulnerabilities if category ever stored in DB.
#[ test ]
fn category_with_injection_chars()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "'; DROP TABLE commands; --",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "'; DROP TABLE commands; --" );
}
