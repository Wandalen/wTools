use super::*;

// Define the enum with different kinds of variants, including struct-like ones with varying field counts.
#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ] // Uncomment to see generated code
pub enum EnumWithNamedFields // Renamed enum for clarity
{
  // Struct-like variant with ZERO named fields
  // Expected: EnumWithNamedFields::variant_zero().form() -> EnumWithNamedFields::VariantZero {}
  VariantZero {},

  // Struct-like variant with ONE named field
  // Expected: EnumWithNamedFields::variant_one().field_a("val").form() -> EnumWithNamedFields::VariantOne { field_a: "val" }
  VariantOne
  {
    field_a : String,
  },

  // Struct-like variant with MULTIPLE named fields
  // Expected: EnumWithNamedFields::variant_two().field_b(1).field_c(true).form() -> EnumWithNamedFields::VariantTwo { field_b: 1, field_c: true }
  VariantTwo
  {
    field_b : i32,
    field_c : bool,
  },

  // Keep a unit variant for completeness check
  UnitVariant,
}

// Include the test logic file (using the new name)
include!( "enum_named_fields_only_test.rs" );
