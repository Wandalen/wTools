//!
//! # Category Field Unit Tests
//!
//! ## What This Tests
//!
//! This test suite validates the basic struct field and builder pattern functionality
//! for the `category` field in `StaticCommandDefinition`. It ensures the field can be:
//! - Properly initialized with default values
//! - Set via the builder pattern
//! - Chained with other builder methods
//! - Used in const contexts
//! - Accessed directly
//!
//! ## Why This Matters
//!
//! The `category` field enables command grouping in CLI help output. If these basic
//! operations don't work correctly, the entire categorization feature fails. These tests
//! prevent:
//! - Uninitialized category values causing undefined behavior
//! - Builder pattern breaking (unable to set category)
//! - Builder chaining failures (incompatible with other methods)
//! - Const compatibility issues (breaking compile-time PHF generation)
//! - Category value corruption (unexpected transformations)
//!
//! ## Failure Interpretation
//!
//! - `default_category_is_empty_string()` fails: Default initialization broken, may cause random category values
//! - `with_category_sets_value()` fails: Builder method not working, cannot set categories
//! - `with_category_is_chainable()` fails: Builder pattern broken, method signature incompatible
//! - `with_category_multiple_calls_last_wins()` fails: Builder semantics unclear, potential concatenation bug
//! - `category_field_is_const_fn_compatible()` fails: Const compatibility broken, PHF generation will fail
//! - `empty_category_string_allowed()` fails: Valid empty category rejected, breaks uncategorized commands
//! - `category_with_spaces_allowed()` fails: Valid category names rejected, limits categorization flexibility
//! - `category_preserves_exact_value()` fails: Category values being transformed, breaks user expectations
//! - `category_field_accessible_directly()` fails: Field visibility issues, cannot read category
//! - `struct_with_all_fields_including_category()` fails: Field ordering or initialization broken
//!
//! ## Related
//!
//! - Issue-089: Category field addition
//! - Issue-088: Auto help enabled (same pattern)
//! - `tests/category_field_conversion_tests.rs` - Tests conversion preserving category
//! - `tests/category_field_backward_compat_tests.rs` - CRITICAL backward compatibility

use unilang::static_data::*;

//
// Test: Default category is empty string
//

/// Verifies that `StaticCommandDefinition::new()` initializes `category` to empty string.
///
/// This prevents uninitialized or random category values.
#[ test ]
fn default_category_is_empty_string()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" );
  assert_eq!( cmd.category, "" );
}

//
// Test: with_category sets value
//

/// Verifies that `with_category()` correctly assigns the category value.
///
/// This prevents the builder method from not working.
#[ test ]
fn with_category_sets_value()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "git_operations" );

  assert_eq!( cmd.category, "git_operations" );
}

//
// Test: with_category is chainable
//

/// Verifies that `with_category()` can be chained with other builder methods.
///
/// This prevents the builder pattern from breaking.
#[ test ]
fn with_category_is_chainable()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "utilities" )
    .with_status( "experimental" )
    .with_version( "2.0.0" );

  assert_eq!( cmd.category, "utilities" );
  assert_eq!( cmd.status, "experimental" );
  assert_eq!( cmd.version, "2.0.0" );
}

//
// Test: with_category multiple calls, last wins
//

/// Verifies that calling `with_category()` multiple times uses the last value.
///
/// This prevents unexpected category concatenation or errors.
#[ test ]
fn with_category_multiple_calls_last_wins()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "first_category" )
    .with_category( "second_category" )
    .with_category( "final_category" );

  assert_eq!( cmd.category, "final_category" );
}

//
// Test: category field is const fn compatible
//

/// Verifies that category can be used in `const` context.
///
/// This prevents breaking const-initialization use cases required for PHF generation.
#[ test ]
fn category_field_is_const_fn_compatible()
{
  const CMD : StaticCommandDefinition = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "const_category" );

  assert_eq!( CMD.category, "const_category" );
}

//
// Test: empty category string allowed
//

/// Verifies that `with_category( "" )` is valid (means uncategorized).
///
/// This prevents rejecting valid empty category.
#[ test ]
fn empty_category_string_allowed()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "" );

  assert_eq!( cmd.category, "" );
}

//
// Test: category with spaces allowed
//

/// Verifies that category names with spaces are allowed.
///
/// This prevents rejecting valid category names with spaces.
#[ test ]
fn category_with_spaces_allowed()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "My Category" );

  assert_eq!( cmd.category, "My Category" );
}

//
// Test: category preserves exact value
//

/// Verifies that category value is not transformed (lowercase, trim, etc.).
///
/// This prevents unexpected category mutations.
#[ test ]
fn category_preserves_exact_value()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "  MiXeD_CaSe_WiTh_SpAcEs  " );

  assert_eq!( cmd.category, "  MiXeD_CaSe_WiTh_SpAcEs  " );
}

//
// Test: category field accessible directly
//

/// Verifies that `cmd.category` can be read directly.
///
/// This prevents field visibility issues.
#[ test ]
fn category_field_accessible_directly()
{
  let cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_category( "test_category" );

  let category : &str = cmd.category;
  assert_eq!( category, "test_category" );
}

//
// Test: struct with all fields including category
//

/// Verifies creating struct with all fields populated including category.
///
/// This prevents field ordering or initialization issues.
#[ test ]
fn struct_with_all_fields_including_category()
{
  let cmd = StaticCommandDefinition::new( ".test", "namespace", "Test command" )
    .with_hint( "test hint" )
    .with_arguments( &[] )
    .with_routine_link( "routine" )
    .with_status( "stable" )
    .with_version( "1.0.0" )
    .with_tags( &[ "tag1", "tag2" ] )
    .with_aliases( &[ "alias1" ] )
    .with_permissions( &[ "read" ] )
    .with_idempotent( true )
    .with_deprecation_message( "" )
    .with_http_method_hint( "POST" )
    .with_examples( &[ "example" ] )
    .with_auto_help_enabled( false )
    .with_category( "full_test_category" );

  assert_eq!( cmd.name, ".test" );
  assert_eq!( cmd.namespace, "namespace" );
  assert_eq!( cmd.description, "Test command" );
  assert_eq!( cmd.hint, "test hint" );
  assert_eq!( cmd.status, "stable" );
  assert_eq!( cmd.version, "1.0.0" );
  assert!( !cmd.auto_help_enabled );
  assert_eq!( cmd.category, "full_test_category" );
}
