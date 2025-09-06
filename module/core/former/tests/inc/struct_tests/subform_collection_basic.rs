#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![deny(missing_docs)]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Default, Debug, PartialEq, former::Former ) ]
// #[ debug ] // Commented out - debug attribute only for temporary debugging
// #[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1 {
  #[ subform_collection( definition = former::VectorDefinition ) ]
  vec_1: Vec<String>,
  #[ subform_collection( definition = former::HashMapDefinition ) ]
  hashmap_1: collection_tools::HashMap< String, String >,
  #[ subform_collection( definition = former::HashSetDefinition ) ]
  hashset_1: collection_tools::HashSet< String >,
}

// == generated begin

// == generated end

include!("./only_test/collections_with_subformer.rs");
