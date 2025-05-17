// File: module/core/former/tests/inc/former_enum_tests/named_tests/standalone_constructor_named_only_test.rs

// Use the items defined in the including file (manual or derive)
use super::*;

/// Tests the standalone constructor for a struct variant.
#[ test ]
fn struct_variant_test() // Use enum-specific test name
{
  // Call the constructor function (manual or derived)
  let former = struct_variant(); // <<< Call with zero args

  // Use the former to build the variant
  let instance = former
  .field( "value".to_string() ) // Set the struct field using the generated setter
  .form();

  // Define the expected enum instance (using the consistent enum name)
  let expected = TestEnum::StructVariant { field : "value".to_string() }; // Use TestEnum

  // Assert that the formed instance matches the expected one
  assert_eq!( instance, expected );
}