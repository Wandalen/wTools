// File: module/core/former/tests/inc/former_enum_tests/named_tests/enum_named_fields_named_derive.rs
use super::*;

// Define the inner struct needed for subform tests directly in this file
#[derive(Debug, PartialEq, Default, Clone)] // No Former derive needed for manual test
pub struct InnerForSubform {
    pub value: i64,
}

// Define the enum with named field variants for testing.
#[ derive( Debug, PartialEq, former::Former ) ]
#[ debug ]
#[ standalone_constructors ]
pub enum EnumWithNamedFields
{
  // --- Zero Fields (Named - Struct-like) ---
  VariantZeroScalar {}, // Expect: variant_zero_scalar() -> Enum
  // VariantZeroDefault {}, // Error case - no manual impl needed

  // --- One Field (Named - Struct-like) ---
  VariantOneScalar { field_a : String }, // Expect: variant_one_scalar(String) -> Enum
  VariantOneSubform { field_b : InnerForSubform }, // Expect: variant_one_subform() -> InnerForSubformFormer
  VariantOneDefault { field_c : InnerForSubform }, // Expect: variant_one_default() -> InnerForSubformFormer

  // --- Two Fields (Named - Struct-like) ---
  VariantTwoScalar { field_d : i32, field_e : bool }, // Expect: variant_two_scalar(i32, bool) -> Enum
  // VariantTwoDefault { field_f : i32, field_g : bool }, // Error case - no manual impl needed
}

// Include the test logic file
include!( "enum_named_fields_named_only_test.rs" );