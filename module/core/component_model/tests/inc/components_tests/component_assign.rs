#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use component_model::Assign;

//

#[ derive( Default, PartialEq, Debug, component_model::Assign ) ]
// #[ debug ]
struct Person {
  age: i32,
  name: String,
}

//

include!("./only_test/component_assign.rs");
