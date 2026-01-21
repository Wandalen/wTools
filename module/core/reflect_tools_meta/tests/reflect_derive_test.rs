//! Compilation tests for Reflect derive macro.
//!
//! Tests that the `#[derive(Reflect)]` macro compiles successfully for various
//! struct definitions. The implementation is currently a stub returning empty
//! `TokenStream`, so these tests verify compilation success rather than runtime
//! behavior.
//!
//! ## Test Matrix
//!
//! | Test Case | Input Type | Fields | Expected | Status |
//! |-----------|-----------|--------|----------|--------|
//! | `test_unit_struct_compiles` | Unit struct | 0 | Compiles | ✅ |
//! | `test_tuple_struct_compiles` | Tuple struct | 1 | Compiles | ✅ |
//! | `test_named_struct_compiles` | Named struct | 2+ | Compiles | ✅ |
//! | `test_generic_struct_compiles` | Generic | 1 generic | Compiles | ✅ |
//! | `test_lifetime_struct_compiles` | With lifetime | 1 ref | Compiles | ✅ |
//!
//! ## Implementation Note
//!
//! The Reflect derive implementation is currently a stub (`qt! {}`) that returns
//! empty `TokenStream`. These tests verify the macro infrastructure works correctly
//! (parsing, attribute handling, code generation) even though the generated code
//! is empty.
//!
//! When the implementation is complete, these tests should be extended to verify
//! the generated `Entity` trait implementations.

#![ allow( missing_docs ) ]

use reflect_tools_meta::*;

/// Test 1: Unit struct compiles with Reflect derive
///
/// Verifies the macro handles zero-field structs correctly.
#[ test ]
fn test_unit_struct_compiles()
{
  #[ derive( Reflect ) ]
  struct UnitStruct;

  let _ = UnitStruct;
}

/// Test 2: Tuple struct compiles with Reflect derive
///
/// Verifies the macro handles single-field tuple structs (newtype pattern).
#[ test ]
#[ allow( dead_code ) ]
fn test_tuple_struct_compiles()
{
  #[ derive( Reflect ) ]
  struct TupleStruct( i32 );

  let _ = TupleStruct( 42 );
}

/// Test 3: Named struct compiles with Reflect derive
///
/// Verifies the macro handles multi-field named structs.
#[ test ]
#[ allow( dead_code ) ]
fn test_named_struct_compiles()
{
  #[ derive( Reflect ) ]
  struct NamedStruct
  {
    field1: i32,
    field2: String,
  }

  let _ = NamedStruct
  {
    field1: 42,
    field2: "test".to_string(),
  };
}

/// Test 4: Generic struct compiles with Reflect derive
///
/// Verifies the macro handles generic type parameters.
#[ test ]
#[ allow( dead_code ) ]
fn test_generic_struct_compiles()
{
  #[ derive( Reflect ) ]
  struct GenericStruct< T >
  {
    value: T,
  }

  let _ = GenericStruct { value: 42 };
  let _ = GenericStruct { value: "test" };
}

/// Test 5: Struct with lifetime compiles with Reflect derive
///
/// Verifies the macro handles lifetime parameters.
#[ test ]
#[ allow( dead_code ) ]
fn test_lifetime_struct_compiles()
{
  #[ derive( Reflect ) ]
  struct LifetimeStruct< 'a >
  {
    value: &'a str,
  }

  let s = "test";
  let _ = LifetimeStruct { value: s };
}

/// Test 6: Struct with debug attribute compiles
///
/// Verifies the `#[debug]` attribute is correctly handled by the derive macro.
/// The implementation checks for this attribute and prints debug info when present.
#[ test ]
#[ allow( dead_code ) ]
fn test_debug_attribute_compiles()
{
  #[ derive( Reflect ) ]
  #[ debug ]
  struct DebugStruct
  {
    field: i32,
  }

  let _ = DebugStruct { field: 42 };
}

/// Test 7: Multiple derives including Reflect
///
/// Verifies Reflect works correctly alongside other standard derives.
#[ test ]
#[ allow( dead_code ) ]
fn test_multiple_derives_compiles()
{
  #[ derive( Debug, Clone, Reflect ) ]
  struct MultiStruct
  {
    value: i32,
  }

  let s = MultiStruct { value: 42 };
  let _ = s.clone();
}
