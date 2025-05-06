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
  let got_pending = Status::pending();
  let exp_pending = Status::Pending;
  assert_eq!( got_pending, exp_pending );

  // Test the Status::Complete constructor (expects direct constructor)
  let got_complete = Status::complete();
  let exp_complete = Status::Complete;
  assert_eq!( got_complete, exp_complete );
}