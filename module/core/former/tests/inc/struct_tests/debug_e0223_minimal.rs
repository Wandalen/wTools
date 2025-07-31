//! Test case for debugging E0223 error in subform_collection
//! This is a minimal reproduction test

use super::*;

#[derive(Default, Debug, PartialEq, former::Former)]
pub struct MinimalStruct {
  #[subform_collection( definition = former::VectorDefinition )]
  vec_1: Vec<String>,
}

#[test]
fn minimal_test() {
  let _instance = MinimalStruct::former()
    .vec_1()
    .add("test".to_string())
    .end()
    .form();
}