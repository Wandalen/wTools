#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Provides shared test assertions for verifying constructors of a unit variant
// within a simple generic enum.
// This file is included by `generic_enum_simple_unit_manual.rs` and `generic_enum_simple_unit_derive.rs`.

use super::*; // Imports EnumOuter from the including file.
// use std::fmt::Debug; // Removed, should be imported by the including file.

#[ derive( Copy, Clone, Debug, PartialEq ) ]
struct MyType(i32);

#[ test ]
fn generic_other_variant_test()
{
  // Test with a concrete type for the generic parameter.
  let got = EnumOuter::<MyType>::other_variant();
  let expected = EnumOuter::<MyType>::OtherVariant;
  assert_eq!(got, expected);

  // Test with another concrete type to be sure.
  let got_u32 = EnumOuter::<u32>::other_variant();
  let expected_u32 = EnumOuter::<u32>::OtherVariant;
  assert_eq!(got_u32, expected_u32);
}