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
///   applied to the generated implicit former, storage, definitions, and end struct for the variant.
/// - **Verify Concrete Inner Type Handling:** Ensure the implicit former correctly handles fields
///   with concrete types (like `InnerG6<TypeForU>`) within the generic enum context.
/// - **Verify Setter Functionality:** Confirm that setters generated for the implicit former work correctly
///   for both generic-dependent fields (if any existed) and fields with concrete or independent types.
/// - **Verify Default Construction:** Test that relying on `Default` for fields within the struct variant works as expected.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario (G6).

use super::*; // Imports items from the parent file (either manual or derive)
// FIX: Removed redundant import, it's imported in _manual.rs where needed.
// use std::marker::PhantomData;

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
  // FIX: Added _phantom_t field to expected value construction
  // Use std::marker::PhantomData directly here since it's not imported
  let expected = EnumG6::< TypeForT >::V1 { inner : expected_inner, flag : true, _phantom_t: std::marker::PhantomData };

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction_independent_struct_variant()
{
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
  // FIX: Added _phantom_t field to expected value construction
  // Use std::marker::PhantomData directly here since it's not imported
  let expected = EnumG6::< TypeForT >::V1 { inner : expected_inner, flag : false, _phantom_t: std::marker::PhantomData };

  assert_eq!( got, expected );
}