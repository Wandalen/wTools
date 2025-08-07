// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[derive(Former)]` for enums with unnamed (tuple) variants that have shared generic
// parameters and bounds, using the default subform behavior. This file is included by both
// `generics_shared_tuple_derive.rs` and `generics_shared_tuple_manual.rs`.
//
// Coverage:
// - Rule 3d (Tuple + Single-Field + Default -> Subform): Tests static method `EnumG3::<T>::v_1()`.
// - Rule 4b (Option 2 Logic): Tests the use of subformer methods and `.form()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines dummy bounds (`BoundA`, `BoundB`) and a concrete type (`MyType`) that satisfies both.
// - Defines test functions (`shared_generics_tuple_variant`, `default_construction`) that invoke the static method
//   `EnumG3::<MyType>::v_1()` provided by the including file (either derived or manual).
// - This constructor returns a subformer (`InnerG3Former<MyType>`).
// - The tests use the subformer setter (`.inner_field()`) and `.form()` to build the final enum instance.

use super::*; // Imports items from the parent file (either manual or derive)

// Define dummy bounds for testing purposes
pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// Define a concrete type that satisfies both bounds for testing
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MyType {
  pub value: i32,
}

impl BoundA for MyType {}
impl BoundB for MyType {}

#[ test ]
fn shared_generics_tuple_variant()
{
  // Call static method provided by the including file - should return a subformer
  let got = EnumG3::<MyType>::v_1()
    .inner_field(MyType { value: 42 })
    .form();

  let expected = EnumG3::V1(InnerG3 { inner_field: MyType { value: 42 } });
  assert_eq!(got, expected);
}

#[ test ]
fn default_construction()
{
  // Test default construction and shared generic functionality
  let got = EnumG3::<MyType>::v_1()
    .inner_field(MyType::default())
    .form();

  let expected = EnumG3::V1(InnerG3 { inner_field: MyType::default() });
  assert_eq!(got, expected);
}