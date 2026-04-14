//! Comprehensive tests for `VariadicFrom` derive macro.
//!
//! Tests all corner cases for the `#[derive(VariadicFrom)]` macro including:
//! - Single-field tuple structs (auto `variadic_from`)
//! - Single-field named structs (auto `variadic_from`)
//! - Generic types
//!
//! ## Test Matrix
//!
//! | Test Case | Type | Fields | Expected | Status |
//! |-----------|------|--------|----------|--------|
//! | `test_single_field_tuple` | Struct Tuple | 1 | Success | ✅ |
//! | `test_single_field_named` | Struct Named | 1 | Success | ✅ |
//! | `test_generic_single_field` | Struct Tuple | 1 generic | Success | ✅ |
//!
//! **Semantics:**
//! The `VariadicFrom` derive implements an inherent `variadic_from()` method that constructs
//! the struct from its inner field type. Similar to `From` but uses a different method name.
//!
//! **Known Limitation:**
//! Enum support is NOT functional with inherent method approach. Enums with multiple variants
//! would generate duplicate method names (one per variant), which Rust rejects. Enum support
//! requires trait-based implementation, which needs `VariadicFrom` trait definition to exist.
//!
//! ## Root Cause (Issue #4 - FIXED for structs)
//!
//! Original implementation generated `impl crate::VariadicFrom<T>` for non-existent trait,
//! causing compile errors. All derive usage failed with "expected trait, found derive macro".
//!
//! ## Why Not Caught
//!
//! No comprehensive test coverage for `VariadicFrom` derive existed in `derive_tools_meta`.
//! Parent crate tests may have been disabled or not comprehensive.
//!
//! ## Fix Applied
//!
//! Changed implementation from trait impl to inherent method (`src/derive/variadic_from.rs:107`, `208`).
//! Now generates `impl MyStruct { pub fn variadic_from(...) -> Self }` instead of
//! `impl crate::VariadicFrom<T> for MyStruct`.
//!
//! ## Prevention
//!
//! Added comprehensive test suite with 3 test cases covering tuple/named structs and generics.
//! Test file documents semantics, validates inherent method behavior, and documents enum limitation.
//!
//! ## Pitfall
//!
//! Variadic conversion patterns require custom traits. Must define trait before implementing it,
//! or use inherent methods for utility constructors. Inherent methods cannot support enums with
//! multiple variants (duplicate method names).

use derive_tools_meta::*;

/// Test 1: Single-field tuple struct
///
/// Should generate inherent `variadic_from()` method for the inner field type.
#[ test ]
fn test_single_field_tuple()
{
  #[ derive( VariadicFrom ) ]
  struct Wrapper( String );

  let w = Wrapper::variadic_from( "hello".to_string() );
  assert_eq!( w.0, "hello" );
}

/// Test 2: Single-field named struct
///
/// Should generate inherent `variadic_from()` method for the single field type.
#[ test ]
fn test_single_field_named()
{
  #[ derive( VariadicFrom ) ]
  struct Container
  {
    value: i32
  }

  let c = Container::variadic_from( 42 );
  assert_eq!( c.value, 42 );
}

/// Test 3: Generic single-field struct
///
/// Should work with generic type parameters.
#[ test ]
fn test_generic_single_field()
{
  #[ derive( VariadicFrom ) ]
  struct GenericWrapper< T >( T );

  let w: GenericWrapper< String > = GenericWrapper::variadic_from( "generic".to_string() );
  assert_eq!( w.0, "generic" );

  let w2: GenericWrapper< Vec< i32 >> = GenericWrapper::variadic_from( vec![ 1, 2, 3 ] );
  assert_eq!( w2.0.len(), 3 );
}

