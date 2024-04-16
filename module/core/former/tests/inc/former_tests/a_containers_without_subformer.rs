#[ allow( unused_imports ) ]
use super::*;

use std::collections::HashMap;
use std::collections::HashSet;

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_1 : HashMap< String, String >,
  hashset_1 : HashSet< String >,
}

// = begin of generated

// = end of generated

include!( "./only_test/containers_without_subformer.rs" );
