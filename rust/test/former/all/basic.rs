
use meta_tools::*;
use former::Former;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, PartialEq, Former )]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : Option< i32 >,
  string_optional_1 : Option< String >,
  vec_1 : Vec< String >,
  hashmap_strings_1 : HashMap< String, String >,
  hashset_strings_1 : HashSet< String >,
}

//

include!( "basic_only_test.rs" );
