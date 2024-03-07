#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

#[ derive( Debug, PartialEq, TheModule::Former ) ]
pub struct Struct1
{
  #[ subformer( former::VectorSubformer ) ]
  vec_1 : Vec< String >,
  #[ subformer( former::HashMapSubformer ) ]
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  #[ subformer( former::HashSetSubformer ) ]
  hashset_strings_1 : std::collections::HashSet< String >,
}

include!( "only_test/containers_with_runtime.rs" );
