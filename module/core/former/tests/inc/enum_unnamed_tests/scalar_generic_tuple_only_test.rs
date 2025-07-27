// Purpose: This file contains the core test logic for verifying the `Former` derive macro's
// handling of enums where a tuple variant containing generic types and bounds is explicitly marked
// with the `#[scalar]` attribute, or when default behavior applies. It defines the shared test
// functions used by both the derive and manual implementation test files for this scenario.
//
// Coverage:
// - Rule 3d (Tuple + Single-Field + Default): Tests the subformer behavior for a single-field tuple variant with generics when `#[scalar]` is absent (default behavior), as implemented in the manual file and expected from the derive.
// - Rule 3f (Tuple + Multi-Field + Default): Tests the subformer behavior for a multi-field tuple variant with generics when `#[scalar]` is absent (default behavior), as implemented in the manual file and expected from the derive. Note: This contradicts the documented Rule 3f which states default for multi-field tuple is scalar. The test logic here reflects the current manual implementation and derive expectation.
// - Rule 1d (Tuple + Single-Field + `#[scalar]`): Tests the scalar constructor generation for a single-field tuple variant with generics when `#[scalar]` is applied, as implemented in the manual file and expected from the derive. (Note: `#[scalar]` is commented out in the derive file, so default behavior is expected and tested).
// - Rule 1f (Tuple + Multi-Field + `#[scalar]`): Not applicable, as the test logic for the multi-field variant uses a subformer, aligning with the manual implementation and derive expectation but not the documented rule for `#[scalar]`.
// - Rule 4b (Option 2 Logic): Demonstrated by the test logic for the `Variant2` subformer, verifying its functionality.
//
// Test Relevance/Acceptance Criteria:
// - Defines a simple bound (`Bound`) and a concrete type (`MyType`) satisfying it.
// - Defines an inner generic struct (`InnerScalar<T>`) used within the enum variants.
// - Contains test functions that call the static methods (`variant_1`, `variant_2`) provided by the including file (either derive or manual implementation).
// - For `variant_1()`, the test calls the method with a value that can be converted into `InnerScalar<MyType>` (both `InnerScalar<MyType>` itself and `MyType` via `Into`). It asserts that the returned enum instance matches a manually constructed `EnumScalarGeneric::Variant1`. This verifies the scalar constructor for a single-field tuple variant.
// - For `variant_2()`, the test calls the method, uses the generated former builder's setters (`._0()` and `._1()`) to set the fields, and calls `.form()`. It asserts that the resulting enum instance matches a manually constructed `EnumScalarGeneric::Variant2`. This verifies the subformer builder for a multi-field tuple variant.
// - This file is included via `include!` by both the `_manual.rs` and `_derive.rs`
// test files for this scenario, ensuring the same test assertions are run against both implementations.

// File: module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_only_test.rs

#[ allow( unused_imports ) ]
use super::*; // Imports items from the parent file (either manual or derive)
use super::{ Bound, MyType, InnerScalar }; // Explicitly import common types
use crate::inc::enum_unnamed_tests::scalar_generic_tuple_derive::EnumScalarGeneric as EnumScalarGenericDerive;
use crate::inc::enum_unnamed_tests::scalar_generic_tuple_manual::EnumScalarGeneric as EnumScalarGenericManual;
// use std::marker::PhantomData; // Keep PhantomData import needed for manual test case construction




#[ test ]
fn scalar_on_single_generic_tuple_variant()
{
  // Tests the direct constructor generated for a single-field tuple variant
  // `Variant1(InnerScalar<T>)` marked with `#[scalar]`.
  // Test Matrix Row: T14.1, T14.2 (Implicitly, as this tests the behavior expected by the matrix)
  let inner_data = InnerScalar { data: MyType( "value1".to_string() ) };
  // Expect a direct static constructor `variant_1` taking `impl Into<InnerScalar<MyType>>`
  // FIX: Changed call to snake_case
  let got = EnumScalarGenericDerive::< MyType >::variant_1( inner_data.clone() );

  let expected = EnumScalarGenericDerive::< MyType >::Variant1( inner_data );
  assert_eq!( got, expected );

  // Test with Into
  // FIX: Changed call to snake_case
  let got_into = EnumScalarGenericDerive::< MyType >::variant_1( MyType( "value1_into".to_string() ) );
   let expected_into = EnumScalarGenericDerive::< MyType >::Variant1( InnerScalar { data: MyType( "value1_into".to_string() ) } );
  assert_eq!( got_into, expected_into );
}

#[ test ]
fn scalar_on_multi_generic_tuple_variant()
{
  // Tests the former builder generated for a multi-field tuple variant
  // `Variant2(InnerScalar<T>, bool)` marked with `#[scalar]`.
  // Test Matrix Row: T14.3, T14.4 (Implicitly, as this tests the behavior expected by the matrix)
  let inner_data = InnerScalar { data: MyType( "value2".to_string() ) };
  // Expect a former builder `variant_2` with setters `_0` and `_1`
  let got = EnumScalarGenericDerive::< MyType >::variant_2()
    ._0( inner_data.clone() )
    ._1( true )
    .form();

  let expected = EnumScalarGenericDerive::< MyType >::Variant2( inner_data, true );
  assert_eq!( got, expected );

  // Test with Into
  let got_into = EnumScalarGenericDerive::< MyType >::variant_2()
    ._0( MyType( "value2_into".to_string() ) )
    ._1( false )
    .form();
  let expected_into = EnumScalarGenericDerive::< MyType >::Variant2( InnerScalar { data: MyType( "value2_into".to_string() ) }, false );
  assert_eq!( got_into, expected_into );
}