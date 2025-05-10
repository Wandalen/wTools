// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_unit_only_test.rs

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a unit variant.
#[ test ]
fn unit_variant_test() // Use enum-specific test name
{
  // Call the constructor function (manual or derived)
  // Assumes `unit_variant` is defined in the including scope
  let instance = unit_variant();

  // Define the expected enum instance (using the consistent enum name)
  let expected = TestEnum::UnitVariant; // Use TestEnum

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}