//! Purpose: Provides shared test assertions and logic for both the derived and manual implementations
//! of a former builder for a named (struct-like) variant (`V1`) within a generic enum (`EnumG6<T>`),
//! where the variant contains a field with an independent concrete generic type (`InnerG6<TypeForU>`).
//! It tests that the constructors generated/implemented for this scenario behave as expected (returning
//! former builders for nested building), correctly handling independent generics.
//!
//! Coverage:
//! - Rule 3g (Struct + Multi-Field + Default): Tests that the constructor for a named variant without specific attributes is a former builder (`v_1()` returns a former).
//! - Rule 4b (Option 2 Logic): Tests the usage of the former builder's setters (`.inner()`, `.flag()`) and `.form()` method, verifying the subformer mechanism in the context of independent generics.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines dummy bounds (`BoundA`, `BoundB`) and concrete types (`TypeForT`, `TypeForU`) satisfying them.
//! - Defines the inner struct `InnerG6<U: BoundB>` which also derives `Former`.
//! - Defines the `EnumG6<T: BoundA>` enum structure with the named variant `V1 { inner: InnerG6<TypeForU>, flag: bool, _phantom_t: PhantomData<T> }`.
//! - Contains test functions (`independent_generics_struct_variant`, `default_construction_independent_struct_variant`) that are included by the derive and manual test files.
//! - The `independent_generics_struct_variant` test calls the static method `EnumG6::<TypeForT>::v_1()`, uses the returned former's setters (`.inner()`, `.flag()`), and calls `.form()`.
//! - The `default_construction_independent_struct_variant` test omits the `.inner()` setter call to verify default value handling for the inner field.
//! - Both tests assert that the resulting enum instances match manually constructed expected values. This verifies that both derived and manual implementations correctly provide former builders that handle fields with independent concrete generic types and non-generic fields within a generic enum.

// File: module/core/former/tests/inc/former_enum_tests/generics_independent_struct_only_test.rs

/// # Test Logic: Independent Generics in Struct Variants
///
/// This file contains the core test logic for verifying the `Former` derive macro's
/// handling of enums where:
/// - The enum itself has generic parameters (e.g., `EnumG6<T>`).
/// - A struct-like variant within the enum contains fields whose types might use
///   different generic parameters or concrete types, independent of the enum's generics
///   (e.g., `V1 { inner: InnerG6<TypeForU>, flag: bool }`).
///
/// ## Purpose:
///
/// - **Verify Generic Propagation:** Ensure the enum's generics (`T`) and bounds (`BoundA`) are correctly
///   applied to the generated implicit former, storage, definitions, former struct, and end struct for the variant.
/// - **Verify Concrete Inner Type Handling:** Ensure the implicit former correctly handles fields
///   with concrete types (like `InnerG6<TypeForU>`) within the generic enum context.
/// - **Verify Setter Functionality:** Confirm that setters generated for the implicit former work correctly
///   for both generic-dependent fields (if any existed) and fields with concrete or independent types.
/// - **Verify Default Construction:** Test that relying on `Default` for fields within the struct variant works as expected.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario (G6).

use super::*; // Imports items from the parent file (either manual or derive)
use std::marker::PhantomData;

// Define dummy bounds for testing purposes
pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// Define concrete types that satisfy the bounds
#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct TypeForT( String ); // Type for the enum's generic
impl BoundA for TypeForT {}

#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct TypeForU( i32 ); // Type for the inner struct's generic field
impl BoundB for TypeForU {}

// Define the inner struct that will be used in the enum variant's field
// It needs its own Former implementation (manual or derived)
#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
pub struct InnerG6< U : BoundB > // BoundB required by the inner struct
{
  pub inner_field : U,
}


#[ test ]
fn independent_generics_struct_variant()
{
  // Test Matrix Row: T25.1 (Implicitly, as this tests the behavior expected by the matrix)
  //! Tests the construction of a struct variant (`V1`) where the inner field (`inner`)
  //! uses a concrete type (`InnerG6<TypeForU>`) independent of the enum's generic (`T`).
  //! It verifies that the implicit former's setters for both the concrete inner field
  //! and the simple `flag` field work correctly.

  // Expects static method `v1` returning the implicit former for the variant
  let got = EnumG6::< TypeForT >::v_1()
    // Set the field holding the *concrete* InnerG6<TypeForU>
    // This requires InnerG6 to have its own Former or a direct setter
    .inner( InnerG6 { inner_field: TypeForU( 99 ) } )
    // Set the non-generic field
    .flag( true )
    .form(); // Calls the specialized End struct for V1

  let expected_inner = InnerG6::< TypeForU > { inner_field : TypeForU( 99 ) };
  // Construct expected enum variant
  // FIX: Re-added _phantom_t field to expected value construction, as both manual and derive enums now have it.
  let expected = EnumG6::< TypeForT >::V1 { inner : expected_inner, flag : true, _phantom_t: PhantomData };

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction_independent_struct_variant()
{
  // Test Matrix Row: T25.2 (Implicitly, as this tests the behavior expected by the matrix)
  //! Tests the construction of a struct variant (`V1`) relying on the `Default`
  //! implementation for the inner field (`inner`) which has a concrete type (`InnerG6<TypeForU>`).
  //! It verifies that the implicit former correctly uses the default value when the setter is not called.

  // Test that default construction works if the inner type has defaults
  let got = EnumG6::< TypeForT >::v_1()
    // .inner is not called, relying on default
    .flag( false ) // Set the non-generic field
    .form();

  let expected_inner = InnerG6::< TypeForU >::default(); // Expect default inner
  // Construct expected enum with default inner and specified flag
  // FIX: Re-added _phantom_t field to expected value construction, as both manual and derive enums now have it.
  let expected = EnumG6::< TypeForT >::V1 { inner : expected_inner, flag : false, _phantom_t: PhantomData };

  assert_eq!( got, expected );
}