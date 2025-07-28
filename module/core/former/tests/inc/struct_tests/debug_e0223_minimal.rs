//! Test case for debugging E0223 error in subform_collection
//! This is a minimal reproduction test

use super::*;

#[derive(Default, Debug, PartialEq, former::Former)]
#[debug]
pub struct MinimalStruct {
  #[subform_collection( definition = former::VectorDefinition )]
  vec_1: Vec<String>,
  #[subform_collection( definition = former::HashMapDefinition )]  
  hashmap_1: collection_tools::HashMap<String, String>,
  #[subform_collection( definition = former::HashSetDefinition )]
  hashset_1: collection_tools::HashSet<String>,
}

#[test]
fn minimal_test() {
  let _instance = MinimalStruct::former()
    .vec_1()
    .add("test".to_string())
    .end()
    .form();
}