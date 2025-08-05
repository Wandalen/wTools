// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[derive(Former)]` for enums with unnamed (tuple) variants that have independent generic
// parameters and bounds, specifically when the variant is marked with `#[scalar]`.
// This file is included by both `generics_independent_tuple_derive.rs` and `generics_independent_tuple_manual.rs`.
//
// Coverage:
// - Rule 1d (Tuple + Single-Field + `#[scalar]` -> Scalar): Tests static method `EnumG5::<T>::v_1()`.
// - Rule 4b (Option 2 Logic): Tests the use of subformer methods and `.form()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines dummy bounds (`BoundA`, `BoundB`) and concrete types (`TypeForT`, `TypeForU`) that satisfy them.
// - Defines test functions (`independent_generics_tuple_variant`, `default_construction_independent_generics`)
//   that invoke the static method `EnumG5::<TypeForT>::v_1()` provided by the including file (either derived or manual).
// - This constructor returns a subformer (`InnerG5Former` specialized with `TypeForU` and configured to return `EnumG5<TypeForT>`).
// - The tests use the subformer setter (`._0()`) and `.form()` to build the final enum instance.
// - Asserts that the resulting `EnumG5` enum instances are equal to the expected variants
//   (`EnumG5::V1(InnerG5 { ... }, PhantomData)`), confirming correct handling of independent generics and the `#[scalar]` attribute.
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
  let got = EnumG5::< TypeForT >::v_1()
    ._0( TypeForU( 99 ) ) // Use the generated setter name for the first field
    .form();

  let expected_inner = InnerG5::< TypeForU > { inner_field : TypeForU( 99 ) };
  let expected = EnumG5::< TypeForT >::V1( expected_inner, PhantomData );

  assert_eq!( got, expected );
}

#[ test ]
fn default_construction_independent_generics()
{
  let got = EnumG5::< TypeForT >::v_1()
    ._0( TypeForU::default() ) // Use the generated setter name for the first field
    .form();

  let expected_inner = InnerG5::< TypeForU > { inner_field : TypeForU::default() };
  let expected = EnumG5::< TypeForT >::V1( expected_inner, PhantomData );

  assert_eq!( got, expected );
}