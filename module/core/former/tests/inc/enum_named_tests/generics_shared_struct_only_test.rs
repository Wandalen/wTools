//! Purpose: Provides shared test assertions and logic for both the derived and manual implementations
//! of a former builder for a named (struct-like) variant (`V1`) within a generic enum (`EnumG4<T>`),
//! where the variant contains a field with a shared generic type (`InnerG4<T>`). It tests that the
//! constructors generated/implemented for this scenario behave as expected (returning former builders
//! for nested building), correctly handling shared generics.
//!
//! Coverage:
//! - Rule 3g (Struct + Multi-Field + Default): Tests that the constructor for a named variant without specific attributes is a former builder (`v_1()` returns a former).
//! - Rule 4b (Option 2 Logic): Tests the usage of the former builder's setters (`.inner()`, `.flag()`) and `.form()` method, verifying the subformer mechanism in the context of shared generics.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines dummy bounds (`BoundA`, `BoundB`) and a concrete type (`MyType`) satisfying them.
//! - Defines the inner struct `InnerG4<T: BoundB>` which also derives `Former`.
//! - Defines the `EnumG4<T: BoundA + BoundB>` enum structure with the named variant `V1 { inner: InnerG4<T>, flag: bool }`.
//! - Contains test functions (`shared_generics_struct_variant`, `default_construction_shared_struct_variant`) that are included by the derive and manual test files.
//! - The `shared_generics_struct_variant` test calls the static method `EnumG4::<MyType>::v_1()`, uses the returned former's setters (`.inner()`, `.flag()`), and calls `.form()`.
//! - The `default_construction_shared_struct_variant` test omits the `.inner()` setter call to verify default value handling for the inner field.
//! - Both tests assert that the resulting enum instances match manually constructed expected values. This verifies that both derived and manual implementations correctly provide former builders that handle fields with shared generic types and non-generic fields within a generic enum.

// File: module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs
use super::*; // Imports items from the parent file (either manual or derive)

// Define dummy bounds for testing purposes
pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// Define a concrete type that satisfies the bounds
#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct MyType( i32 );
impl BoundA for MyType {}
impl BoundB for MyType {}

#[ test ]
fn shared_generics_struct_variant()
{
  // Test Matrix Row: T26.1 (Implicitly, as this tests the behavior expected by the matrix)
  //! Tests the construction of a struct variant (`V1`) where the inner field (`inner`)
  //! uses a generic type (`InnerG4<T>`) that shares the enum's generic parameter (`T`).
  //! It verifies that the implicit former's setters for both the generic inner field
  //! and the simple `flag` field work correctly.

  // CORRECTED: Use v_1() instead of v1()
  let inner_val = InnerG4::< MyType > { inner_field : MyType( 42 ) };
  let got = EnumG4::< MyType >::v_1() // Expects static method `v_1` returning the implicit former
    .inner( inner_val.clone() )      // Use the `inner` setter
    .flag( true )                    // Use the `flag` setter
    .form();                         // Calls the specialized End struct
                                     // qqq : xxx : check if this test is correct

  let expected_inner = InnerG4::< MyType > { inner_field : MyType( 42 ) };
  let expected = EnumG4::< MyType >::V1 { inner : expected_inner, flag : true }; // Construct expected enum

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction_shared_struct_variant()
{
  // Test Matrix Row: T26.2 (Implicitly, as this tests the behavior expected by the matrix)
  //! Tests the construction of a struct variant (`V1`) relying on the `Default`
  //! implementation for the inner field (`inner`) which has a generic type (`InnerG4<T>`).
  //! It verifies that the implicit former correctly uses the default value when the setter is not called.

  // Test that default construction works if the inner type has defaults
  // CORRECTED: Use v_1() instead of v1()
  let got = EnumG4::< MyType >::v_1()
    // .inner is not called, relying on default
    .flag( false ) // Set the non-generic field
    .form();
                                     // qqq : xxx : check if this test is correct

  let expected_inner = InnerG4::< MyType > { inner_field : MyType::default() }; // Expect default inner
  // Construct expected enum with default inner and specified flag
  let expected = EnumG4::< MyType >::V1 { inner : expected_inner, flag : false };

  assert_eq!( got, expected );
}