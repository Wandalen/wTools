use super::*; // Assuming it's in a module within `former_enum_tests`
// No need for FormingEnd or StoragePreform here for the scalar-style method

/// Enum with different variant types for testing.
#[ derive( Debug, PartialEq ) ]
enum EnumWithMultiField
{
  /// A simple variant with one field.
  Simple( String ),
  /// A variant with multiple unnamed fields.
  MultiTuple( i32, String, bool ),
  /// A variant with no fields.
  Empty,
}

// --- Manual implementation of static methods ---
impl EnumWithMultiField
{
  /// Manually implemented "scalar setter" style constructor for the Simple variant.
  #[ inline( always ) ]
  pub fn simple( value : impl Into< String > ) -> Self
  {
    // Directly construct the variant with the provided value.
    Self::Simple( value.into() )
  }

  /// Manually implemented constructor for the MultiTuple variant.
  #[ inline( always ) ]
  pub fn multi_tuple( field0 : i32, field1 : impl Into< String >, field2 : bool ) -> Self
  {
    Self::MultiTuple( field0, field1.into(), field2 )
  }

  /// Manually implemented constructor for the Empty variant.
  #[ inline( always ) ]
  pub fn empty() -> Self
  {
    Self::Empty
  }
}

// Include the actual test logic from the adjacent file
include!( "multi_field_only_test.rs" );