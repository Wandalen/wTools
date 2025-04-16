// File: module/core/former/tests/inc/former_enum_tests/multi_field_derive.rs
use super::*; // Assuming it's in a module within `former_enum_tests`

// Define an inner struct that also derives Former
#[ derive( Debug, Default, PartialEq, former::Former ) ]
pub struct InnerData
{
  data1 : i32,
  data2 : bool,
}

// Define another inner struct for the implicit subform test
#[ derive( Debug, Default, PartialEq, former::Former ) ]
pub struct OtherInnerData
{
  info : String,
}


/// Enum with different variant types for testing.
/// NOTE: Uses the derive macro here!
#[ derive( Debug, PartialEq, the_module::Former ) ]
enum EnumWithMultiField
{
  /// Explicitly scalar: Expects Enum::simple(val)
  #[scalar]
  Simple( String ),
  /// Multi-field tuple: Explicitly scalar required -> Expects Enum::multi_tuple(...)
  #[scalar]
  MultiTuple( i32, String, bool ),
  /// Unit: Expects Enum::empty()
  Empty,
  /// Explicit Subform: Expects Enum::struct_() -> InnerDataFormer<...>
  #[subform_scalar] // Apply attribute to variant
  Struct( InnerData ),
  /// Implicit Subform (default for single field with Former type): Expects Enum::implicit_subform() -> OtherInnerDataFormer<...>
  ImplicitSubform( OtherInnerData ), // No attribute, should default to subformer
}

// Include the actual test logic from the separate file
include!( "multi_field_only_test.rs" ); // Include the same test logic