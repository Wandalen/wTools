#![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Default, Debug, PartialEq, former::Former ) ]
// #[ derive( Default, Debug, PartialEq, former::Former ) ] #[ debug ]
// #[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  #[ subform_collection( definition = former::VectorDefinition ) ]
  vec_1 : Vec< String >,
  #[ subform_collection( definition = former::HashMapDefinition ) ]
  hashmap_1 : collection_tools::HashMap< String, String >,
  #[ subform_collection( definition = former::HashSetDefinition ) ]
  hashset_1 : collection_tools::HashSet< String >,
}

// == generated begin

// == generated end

include!( "./only_test/collections_with_subformer.rs" );
