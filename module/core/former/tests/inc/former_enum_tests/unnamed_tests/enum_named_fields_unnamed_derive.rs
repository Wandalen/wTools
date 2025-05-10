// File: module/core/former/tests/inc/former_enum_tests/unnamed_tests/enum_named_fields_unnamed_derive.rs
use super::*;

// Define the enum with zero-field unnamed (tuple) variants for testing.
#[ derive( Debug, PartialEq, former::Former ) ]
#[ debug ]
#[ standalone_constructors ]
pub enum EnumWithNamedFields
{
  // --- Zero Fields (Unnamed - Tuple-like) ---
  VariantZeroUnnamedDefault(), // Expect: variant_zero_unnamed_default() -> Enum (Default is scalar for 0 fields)
  #[ scalar ] // Expect: variant_zero_unnamed_scalar() -> Enum
  VariantZeroUnnamedScalar(),
}

// Include the test logic file
include!( "enum_named_fields_unnamed_only_test.rs" );