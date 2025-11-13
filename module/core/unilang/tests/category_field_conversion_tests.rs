//!
//! # Category Field Conversion Tests
//!
//! ## What This Tests
//!
//! This test suite validates that the `category` field is properly preserved during
//! conversion from `StaticCommandDefinition` to `CommandDefinition` via the `From` trait.
//! The conversion is a critical step in the data flow:
//!
//! YAML → Build Script → `StaticCommandDefinition` → **From conversion** → `CommandDefinition`
//!
//! ## Why This Matters
//!
//! Issue-089 root cause: The conversion was hardcoding empty string for category,
//! discarding the value from YAML. This broke command grouping in help output.
//!
//! All fields added to `StaticCommandDefinition` must be explicitly mapped in the
//! `From` conversion, otherwise YAML configuration is silently ignored. These tests prevent:
//! - Category values being lost during conversion
//! - Empty categories becoming unexpected default values
//! - Category being overwritten by other field conversions
//! - Unexpected transformations (trim, lowercase, etc.)
//! - Regression of issue-088 fix (`auto_help_enabled` pattern)
//!
//! ## Failure Interpretation
//!
//! - `from_static_preserves_non_empty_category()` fails: Conversion losing category value (issue-089 root cause)
//! - `from_static_preserves_empty_category()` fails: Empty string not preserved, becoming null or default
//! - `from_static_with_all_fields_preserves_category()` fails: Category overwritten by other field conversions
//! - `conversion_doesnt_modify_category()` fails: Category being transformed unexpectedly
//! - `issue_088_regression_both_fields_preserved()` fails: New fix broke old fix (critical regression)
//!
//! ## Related
//!
//! - Issue-089: Category field conversion fix
//! - Issue-088: Auto help enabled conversion fix (same pattern)
//! - `tests/category_field_unit_tests.rs` - Tests struct and builder
//! - `tests/category_field_backward_compat_tests.rs` - CRITICAL backward compatibility

use unilang::static_data::*;
use unilang::data::CommandDefinition;

//
// Test: from_static preserves non-empty category
//

/// Verifies that conversion preserves non-empty category value.
///
/// This prevents loss of category during conversion (was issue-089 root cause).
// test_kind: bug_reproducer(issue-089)
#[ test ]
fn from_static_preserves_non_empty_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
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

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "git_ops" );
}

//
// Test: from_static preserves empty category
//

/// Verifies that conversion preserves empty category (not null or "uncategorized").
///
/// This prevents empty category becoming unexpected default.
#[ test ]
fn from_static_preserves_empty_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
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

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "" );
}

//
// Test: from_static with all fields preserves category
//

/// Verifies that category is preserved when all struct fields are populated.
///
/// This prevents category being overwritten by other field conversions.
#[ test ]
fn from_static_with_all_fields_preserves_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "namespace",
    description : "Test command",
    hint : "test hint",
    arguments : &[],
    routine_link : Some( "routine" ),
    status : "stable",
    version : "1.0.0",
    tags : &[ "tag1", "tag2" ],
    aliases : &[ "alias1" ],
    permissions : &[ "read" ],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "POST",
    examples : &[ "example" ],
    auto_help_enabled : false,
    category : "test_category",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "test_category" );
  assert!( !dynamic_cmd.auto_help_enabled() );
  assert_eq!( dynamic_cmd.name().as_str(), ".test" );
}

//
// Test: conversion doesn't modify category
//

/// Verifies that category value is unchanged through conversion (no trim, lowercase, etc.).
///
/// This prevents unexpected category transformations.
#[ test ]
fn conversion_doesnt_modify_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
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
    category : "  MiXeD_CaSe_WiTh_SpAcEs  ",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "  MiXeD_CaSe_WiTh_SpAcEs  " );
}

//
// Test: issue-088 regression - both fields preserved
//

/// Verifies that both `auto_help_enabled` AND `category` are preserved in conversion.
///
/// This prevents regression of issue-088 fix when adding issue-089 fix.
// test_kind: bug_reproducer(issue-088)
// test_kind: bug_reproducer(issue-089)
#[ test ]
fn issue_088_regression_both_fields_preserved()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
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
    auto_help_enabled : false,
    category : "test",
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert!( !dynamic_cmd.auto_help_enabled(), "Issue-088 regression: auto_help_enabled not preserved" );
  assert_eq!( dynamic_cmd.category(), "test", "Issue-089: category not preserved" );
}
