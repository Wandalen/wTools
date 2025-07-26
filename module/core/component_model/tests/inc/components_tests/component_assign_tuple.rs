use super::*;
#[allow(unused_imports)]
use component_model::Assign;

#[derive(Default, PartialEq, Debug, component_model::Assign)]
struct TupleStruct(i32, String);

//

include!("./only_test/component_assign_tuple.rs");
