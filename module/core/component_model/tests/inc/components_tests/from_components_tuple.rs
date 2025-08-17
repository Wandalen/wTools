use super::*;

// Define a source tuple struct with several fields
#[ derive( Debug, Default, PartialEq ) ]
struct SourceTuple(i32, String, f32);

// Implement From<&SourceTuple> for each type it contains
// This is needed for the FromComponents bounds `T : Into< FieldType >` to work in the test
impl From<&SourceTuple> for i32 {
  #[ inline( always ) ]
  fn from( src : &SourceTuple  ) -> Self {
    src.0
  }
}

impl From<&SourceTuple> for String {
  #[ inline( always ) ]
  fn from( src : &SourceTuple  ) -> Self {
    src.1.clone()
  }
}

impl From<&SourceTuple> for f32 {
  #[ inline( always ) ]
  fn from( src : &SourceTuple  ) -> Self {
    src.2
  }
}

// Define a target tuple struct with a subset of fields/types
#[ derive( Debug, Default, PartialEq, component_model::FromComponents ) ]
struct TargetTuple(i32, String);

//

include!("./only_test/from_components_tuple.rs");
