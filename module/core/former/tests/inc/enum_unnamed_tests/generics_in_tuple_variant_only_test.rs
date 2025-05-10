//! Purpose: Provides shared test assertions and logic for verifying the constructors generated
//! by `#[derive(Former)]` for enums with unnamed (tuple) variants that have shared generic
//! parameters and bounds, using the default subform behavior. This file is included by both
//! `generics_in_tuple_variant_tuple_derive.rs` and `generics_in_tuple_variant_tuple_manual.rs`.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default -> Subform): Tests static method `EnumOuter::<X>::variant()`.
//! - Rule 4b (Option 2 Logic): Tests the use of subformer methods and `.form()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines dummy bounds (`BoundA`, `BoundB`) and concrete types (`TypeForT`, `TypeForU`) that satisfy them.
//! - Defines test functions (`basic_construction`, `construction_with_bounds`) that invoke the static method
//!   `EnumOuter::<X>::variant()` provided by the including file (either derived or manual).
//! - This constructor returns a subformer (`InnerGenericFormer<X>`).
//! - The tests use the subformer setter (`.inner_field()`) and `.form()` to build the final enum instance.
//! - Asserts that the resulting `EnumOuter` enum instances are equal to the expected variants
//!   (`EnumOuter::Variant(InnerGeneric { ... })`), confirming correct handling of shared generics and bounds.
//! Test logic for enum variants with independent generic parameters.
//!
//! Purpose:
//! - Define `EnumG5<T: BoundA>` and `InnerG5<U: BoundB>` with independent generics.
//! - Apply `#[derive(Former)]` to both the enum and the inner struct.
//! - Use the included `_only_test.rs` file to verify that the macro-generated code
//!   correctly handles the distinct generics `T` and `U` (instantiated as `TypeForU`
//!   in the variant) and their respective bounds.
//!
// File: module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_only_test.rs
#[ allow( unused_imports ) ]
use super::*; // Should import EnumOuter and InnerGeneric from either the manual or derive file
use std::fmt::Debug; // Removed redundant import (E0252 fix)

#[ test ]
fn basic_construction()
{
  // Define a concrete type that satisfies the bounds (Debug + Copy + Default + PartialEq)
  let inner_value = 42_i32; // i32 implements all needed bounds

  // Expect EnumOuter::<i32>::variant() to return a former for InnerGeneric<i32>
  // This tests the basic generic propagation and subformer mechanism.
  let got = EnumOuter::< i32 >::variant()
  .inner_field( inner_value ) // Assuming InnerGenericFormer has this setter
  .form();                   // This should call the specialized End struct

  // Define the expected enum instance
  let expected_inner = InnerGeneric::< i32 >
  {
    inner_field : inner_value,
  };
  let expected = EnumOuter::< i32 >::Variant( expected_inner );

  assert_eq!( got, expected );
}

#[ test ]
fn construction_with_bounds()
{
  // Test with a custom type that meets the specific bounds: Debug + Copy + Default + PartialEq
  #[ derive( Debug, Copy, Clone, PartialEq, Default ) ] // Added Default
  struct MyCopyableDebug( f32 );

  let inner_value = MyCopyableDebug( 3.14 );

  // Expect EnumOuter::<MyCopyableDebug>::variant() to work because
  // MyCopyableDebug satisfies all required bounds.
  // This tests the handling and merging of bounds from both the enum and the inner type.
  let got = EnumOuter::< MyCopyableDebug >::variant()
  .inner_field( inner_value )
  .form();

  let expected_inner = InnerGeneric::< MyCopyableDebug >
  {
    inner_field : inner_value,
  };
  let expected = EnumOuter::< MyCopyableDebug >::Variant( expected_inner );

  assert_eq!( got, expected );
}

// Optional: Add a test that *should* fail compilation if bounds are not met.
// This requires a compile-fail test setup (like trybuild), which is outside
// the scope of just adding files here.
// #[ test ]
// fn construction_without_bounds_fails()
// {
//   // Define a type that does NOT satisfy the bounds (e.g., no Copy or no Default)
//   #[derive(Debug, PartialEq)]
//   struct MyDebugOnly(f32);
//
//   let inner_value = MyDebugOnly(1.0);
//
//   // This call should ideally fail to compile because MyDebugOnly does not implement Copy/Default
//   let _got = EnumOuter::< MyDebugOnly >::variant() // << Compile error expected here
//     .inner_field( inner_value )
//     .form();
// }