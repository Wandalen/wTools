#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Default, Debug, PartialEq, former::Former ) ]
#[ debug ]
// #[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  #[ subformer( former::VectorDefinition ) ]
  vec_1 : Vec< String >,
  // #[ subformer( former::HashMapSubformer ) ]
  hashmap_1 : std::collections::HashMap< String, String >,
  // // #[ subformer( former::HashSetSubformer ) ]
  hashset_1 : std::collections::HashSet< String >,
}

// = generated


// = generated

include!( "./only_test/containers_with_subformer.rs" );
// xxx : uncomment
