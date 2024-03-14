#[ allow( unused_imports ) ]
use super::*;

use std::collections::HashMap;
use std::collections::HashSet;

#[ derive( Debug, PartialEq, TheModule::Former ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_strings_1 : HashMap< String, String >,
  hashset_strings_1 : HashSet< String >,
}

//

include!( "only_test/containers_without_runtime.rs" );
