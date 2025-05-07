// File: module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs
use super::*;

// Define the inner struct needed for subform tests directly in this file
#[derive(Debug, PartialEq, Default, Clone, former::Former)]
pub struct InnerForSubform {
    pub value: i64,
}

// Define the enum with different kinds of variants, including struct-like ones with varying field counts.
#[ derive( Debug, PartialEq, former::Former ) ]
#[ debug ]
pub enum EnumWithNamedFields
{
//   // --- Unit Variant ---
//   // Expect: unit_variant_default() -> Enum (Default is scalar for unit)
//   UnitVariantDefault, // Renamed from UnitVariant
//   #[ scalar ] // Expect: unit_variant_scalar() -> Enum
//   UnitVariantScalar, // New
//
//   // --- Zero Fields (Named - Struct-like) ---
//   // VariantZeroDefault {}, // Expect: Compile Error (No #[scalar]) - Cannot test directly
//   #[ scalar ] // Expect: variant_zero_scalar() -> Enum
//   VariantZeroScalar {},
//
//   // --- Zero Fields (Unnamed - Tuple-like) ---
//   VariantZeroUnnamedDefault(), // Expect: variant_zero_unnamed_default() -> Enum (Default is scalar for 0 fields)
//   #[ scalar ] // Expect: variant_zero_unnamed_scalar() -> Enum
//   VariantZeroUnnamedScalar(),

  // // --- One Field (Named - Struct-like) ---
  // // Expect: variant_one_default() -> InnerForSubformFormer<...> (Default behavior for single field is subform)
  // VariantOneDefault { field_c : InnerForSubform },
  // #[ scalar ] // Expect: variant_one_scalar( String ) -> Enum
  // VariantOneScalar { field_a : String },
  // #[ subform_scalar ] // Expect: variant_one_subform() -> InnerForSubformFormer<...>
  // VariantOneSubform { field_b : InnerForSubform },

  // // --- Two Fields (Named - Struct-like) --- (Commented out for isolation)
  // // // VariantTwoDefault { field_f : i32, field_g : bool }, // Expect: Compile Error (No #[scalar]) - Cannot test directly
  // // #[ scalar ] // Expect: variant_two_scalar( i32, bool ) -> Enum
  // // VariantTwoScalar { field_d : i32, field_e : bool },

}

// Include the test logic file (using the new name)
include!( "enum_named_fields_only_test.rs" );