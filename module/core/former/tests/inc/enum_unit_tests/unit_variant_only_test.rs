// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[derive(Former)]` for enums with unit variants, including with `#[standalone_constructors]`.
// This file is included by both `unit_variant_derive.rs` and `unit_variant_manual.rs`.
//
// Coverage:
// - Rule 3a (Unit + Default): Tests static methods `Status::pending()` and `Status::complete()`.
// - Rule 1a (Unit + `#[scalar]`): Tests static methods (as default for unit is scalar).
// - Rule 4a (#[standalone_constructors]): Tests standalone functions `pending()` and `complete()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines test functions (`unit_variant_constructors`, `unit_variant_standalone_constructors`) that
//   invoke constructors provided by the including file (either derived or manual).
// - Asserts that the instances created by these constructors are equal to the expected
//   enum variants (`Status::Pending`, `Status::Complete`).
//
// # Test Matrix for Unit Variants
//
// This matrix outlines the combinations of `former` attributes tested for enum **unit variants**
// and the expected behavior of the generated constructors.
//
// Factors considered:
// 1.  **Variant-Level Attribute:** None (Default behavior), `#[scalar]`, `#[subform_scalar]` (Expected: Error)
// 2.  **Enum-Level Attribute:** None, `#[standalone_constructors]`
//
// | # | Variant Attribute | Enum Attribute              | Expected Constructor Signature (Static Method on Enum) | Expected Standalone Constructor (if `#[standalone_constructors]`) | Relevant Rule(s) | Handler File (Meta)        |
// |---|-------------------|-----------------------------|------------------------------------------------------|--------------------------------------------------------------------|------------------|----------------------------|
// | 1 | Default           | None                        | `MyEnum::my_unit_variant() -> MyEnum`                | N/A                                                                | 3a               | `unit_variant_handler.rs`  |
// | 2 | `#[scalar]`       | None                        | `MyEnum::my_unit_variant() -> MyEnum`                | N/A                                                                | 1a               | `unit_variant_handler.rs`  |
// | 3 | Default           | `#[standalone_constructors]` | `MyEnum::my_unit_variant() -> MyEnum`                | `fn my_unit_variant() -> MyEnum`                                   | 3a, 4            | `unit_variant_handler.rs`  |
// | 4 | `#[scalar]`       | `#[standalone_constructors]` | `MyEnum::my_unit_variant() -> MyEnum`                | `fn my_unit_variant() -> MyEnum`                                   | 1a, 4            | `unit_variant_handler.rs`  |
// | 5 | `#[subform_scalar]`| (Any)                       | *Compile Error*                                      | *Compile Error*                                                    | 2a               | (Dispatch logic in `former_enum.rs` should error) |
//
// *(Note: "Default" for unit variants behaves like `#[scalar]`)*
//
// File: module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs
use super::*;


#[ test ]
fn unit_variant_constructors()
{
  // Test the Status::Pending constructor (expects direct constructor)
  let got_pending = crate::inc::enum_unit_tests::unit_variant_manual::Status::pending();
  let exp_pending = crate::inc::enum_unit_tests::unit_variant_manual::Status::Pending;
  assert_eq!( got_pending, exp_pending );

  // Test the Status::Complete constructor (expects direct constructor)
  let got_complete = crate::inc::enum_unit_tests::unit_variant_manual::Status::complete();
  let exp_complete = crate::inc::enum_unit_tests::unit_variant_manual::Status::Complete;
  assert_eq!( got_complete, exp_complete );
}

#[ test ]
fn unit_variant_standalone_constructors()
{
  // Test the top-level pending() standalone constructor
  let got_pending = crate::inc::enum_unit_tests::unit_variant_manual::pending();
  let exp_pending = crate::inc::enum_unit_tests::unit_variant_manual::Status::Pending; // Use full path to Status
  assert_eq!( got_pending, exp_pending );

  // Test the top-level complete() standalone constructor
  let got_complete = crate::inc::enum_unit_tests::unit_variant_manual::complete();
  let exp_complete = crate::inc::enum_unit_tests::unit_variant_manual::Status::Complete; // Use full path to Status
  assert_eq!( got_complete, exp_complete );
}