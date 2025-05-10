// File: module/core/former/tests/inc/former_enum_tests/unit_tests/standalone_constructor_args_unit_only_test.rs

// Use the items defined in the including file (manual or derive for args)
use super::*;

/// Tests the standalone constructor for a unit variant (still takes no args).
#[ test ]
fn unit_variant_args_test() // New test name
{
  // Assumes `unit_variant_args` is defined in the including scope
  let instance = unit_variant_args(); // Returns Enum directly
  let expected = TestEnumArgs::UnitVariantArgs;
  assert_eq!( instance, expected );
}