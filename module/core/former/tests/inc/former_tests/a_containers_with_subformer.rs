#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Default, Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  #[ subformer( former::VectorDefinition ) ]
  vec_1 : Vec< String >,
  #[ subformer( former::HashMapDefinition ) ]
  hashmap_1 : std::collections::HashMap< String, String >,
  #[ subformer( former::HashSetDefinition ) ]
  hashset_1 : std::collections::HashSet< String >,
}

// = generated

include!( "./only_test/containers_with_subformer.rs" );
