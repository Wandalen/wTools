#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Default, Debug, PartialEq, former::Former ) ]
// #[ derive( Default, Debug, PartialEq, former::Former ) ] #[ debug ]
// #[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  #[ container( former::VectorDefinition ) ]
  vec_1 : Vec< String >,
  #[ container( former::HashMapDefinition ) ]
  hashmap_1 : std::collections::HashMap< String, String >,
  #[ container( former::HashSetDefinition ) ]
  hashset_1 : std::collections::HashSet< String >,
}

// == generated begin

// == generated end

include!( "./only_test/containers_with_subformer.rs" );
