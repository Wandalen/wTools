use super::*;
#[ allow( unused_imports ) ]
use component_model::{Assign, AssignWithType};

// Define TupleStruct1 with more fields/types
#[ derive( Debug, Default, PartialEq, component_model::Assign, component_model::ComponentsAssign ) ]
struct TupleStruct1(i32, String, f32);

// Define TupleStruct2 with a subset of types from TupleStruct1
#[ derive( Debug, Default, PartialEq, component_model::Assign, component_model::ComponentsAssign ) ]
struct TupleStruct2(i32, String);

// Implement From<&TupleStruct1> for the types present in TupleStruct2
impl From<&TupleStruct1> for i32 {
  #[ inline( always ) ]
  fn from(src: &TupleStruct1) -> Self {
    src.0
  }
}

impl From<&TupleStruct1> for String {
  #[ inline( always ) ]
  fn from(src: &TupleStruct1) -> Self {
    src.1.clone()
  }
}

//

include!("./only_test/components_assign_tuple.rs");
