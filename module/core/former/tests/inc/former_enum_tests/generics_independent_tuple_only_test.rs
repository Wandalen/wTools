/// Test logic for enum variants with independent generic parameters.
///
/// This file contains the actual `#[ test ]` functions for testing the `Former`
/// derive macro's handling of enums where the enum itself has a generic parameter (`T`)
/// and a variant contains an inner type with a *different* generic parameter (`U`).
///
/// Purpose:
/// - Verify that the generated static method for the variant correctly handles the enum's generic (`T`).
/// - Verify that the subformer for the inner type correctly handles its own generic (`U`).
/// - Ensure that bounds from both the enum (`BoundA` for `T`) and the inner type (`BoundB` for `U`)
///   are correctly applied and satisfied in the generated `impl FormingEnd`.
///
/// This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
/// test files for this scenario (G5).

// File: module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs
use super::*; // Imports items from the parent file (either manual or derive)

// Define dummy bounds for testing purposes
pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// Define concrete types that satisfy the bounds
#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct TypeForT( String ); // Type for the enum's generic
impl BoundA for TypeForT {}

#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct TypeForU( i32 ); // Type for the inner struct's generic
impl BoundB for TypeForU {}

#[ test ]
fn independent_generics_tuple_variant()
{
  let got = EnumG5::< TypeForT >::v1()
    .inner_field( TypeForU( 99 ) )
    .form();

  let expected_inner = InnerG5::< TypeForU > { inner_field : TypeForU( 99 ) };
  // CORRECTED: Add PhantomData to expected variant construction
  let expected = EnumG5::< TypeForT >::V1( expected_inner, PhantomData );

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction_independent_generics()
{
  let got = EnumG5::< TypeForT >::v1()
    .form();

  let expected_inner = InnerG5::< TypeForU > { inner_field : TypeForU::default() };
  // CORRECTED: Add PhantomData to expected variant construction
  let expected = EnumG5::< TypeForT >::V1( expected_inner, PhantomData );

  assert_eq!( got, expected );
}