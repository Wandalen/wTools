//! Comprehensive tests for `InnerFrom` derive macro.
//!
//! Tests all corner cases for the `#[derive(InnerFrom)]` macro including:
//! - Single-field tuple structs (auto `inner_from`)
//! - Single-field named structs (auto `inner_from`)
//! - Generic types
//! - Constructor-like conversion behavior
//!
//! ## Test Matrix
//!
//! | Test Case | Struct Type | Fields | Expected | Status |
//! |-----------|-------------|--------|----------|--------|
//! | `test_single_field_tuple` | Tuple | 1 | Success | ✅ |
//! | `test_single_field_named` | Named | 1 | Success | ✅ |
//! | `test_generic_single_field` | Tuple | 1 generic | Success | ✅ |
//!
//! **Semantics:**
//! The `InnerFrom` derive implements an inherent `inner_from()` method that constructs
//! the struct from its inner field type. Similar to `From` but uses a different method name.
//!
//! ## Root Cause (Issue #3 - FIXED)
//!
//! Original implementation generated `impl crate::InnerFrom<T>` for non-existent trait,
//! causing compile errors. All derive usage failed with "expected trait, found derive macro".
//!
//! ## Why Not Caught
//!
//! All `InnerFrom` tests in parent `derive_tools` crate were commented out with note
//! "`InnerFrom` derive not available" (`tests/inc/inner_from_only_test.rs:5`).
//! No tests existed in `derive_tools_meta` to catch the issue.
//!
//! ## Fix Applied
//!
//! Changed implementation from trait impl to inherent method (`src/derive/inner_from.rs:88`).
//! Now generates `impl MyStruct { pub fn inner_from(...) -> Self }` instead of
//! `impl crate::InnerFrom<T> for MyStruct`.
//!
//! ## Prevention
//!
//! Added comprehensive test suite with 3 test cases covering tuple/named structs and generics.
//! Test file documents semantics and validates inherent method behavior.
//!
//! ## Pitfall
//!
//! Proc macros that generate trait implementations must verify the trait exists and is in scope.
//! Inherent implementations are safer for utility methods like constructors.

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should generate inherent `inner_from()` method for the inner field type.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( InnerFrom ) ]
  struct Wrapper( String );

  let w = Wrapper::inner_from( "hello".to_string() );
  assert_eq!( w.0, "hello" );
}

/// Test 2: Single-field named struct
///
/// Should generate inherent `inner_from()` method for the single field type.
#[ test ]
fn test_single_field_named()
{
  #[ derive( InnerFrom ) ]
  struct Container
  {
    value: i32
  }

  let c = Container::inner_from( 42 );
  assert_eq!( c.value, 42 );
}

/// Test 3: Generic single-field struct
///
/// Should work with generic type parameters.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( InnerFrom ) ]
  struct GenericWrapper< T >( T );

  let w: GenericWrapper< String > = GenericWrapper::inner_from( "generic".to_string() );
  assert_eq!( w.0, "generic" );

  let w2: GenericWrapper< Vec< i32 >> = GenericWrapper::inner_from( vec![ 1, 2, 3 ] );
  assert_eq!( w2.0.len(), 3 );
}
