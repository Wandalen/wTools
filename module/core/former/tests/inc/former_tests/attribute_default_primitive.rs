#[ allow( unused_imports ) ]
use super::*;

// #[ allow( unused_imports ) ]
// use test_tools::exposed::*;
//
// only_for_aggregating_module!
// {
//   #[ allow( unused_imports ) ]
//   use wtools::meta::*;
//   #[ allow( unused_imports ) ]
//   use wtools::the_module::Former;
// }
//
// only_for_terminal_module!
// {
//   #[ allow( unused_imports ) ]
//   use meta_tools::*;
//   #[ allow( unused_imports ) ]
//   use the_module::Former;
// }

use std::collections::HashMap;
use std::collections::HashSet;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1
{
  #[ default( 31 ) ]
  pub int_1 : i32,
  #[ default( "abc" ) ]
  string_1 : String,
  #[ default( 31 ) ]
  int_optional_1 : Option< i32 >,
  #[ default( "abc" ) ]
  string_optional_1 : Option< String >,

  vec_1 : Vec< String >,
  hashmap_1 : HashMap< String, String >,
  hashset_1 : HashSet< String >,
}

//

tests_impls!
{
  fn test_complex()
  {
    let command = Struct1::former().form();

    let expected = Struct1
    {
      int_1 : 31,
      string_1 : "abc".to_string(),
      int_optional_1 : Some( 31 ),
      string_optional_1 : Some( "abc".to_string() ),
      vec_1 : vec![],
      hashmap_1 : hmap!{},
      hashset_1 : hset!{},
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
