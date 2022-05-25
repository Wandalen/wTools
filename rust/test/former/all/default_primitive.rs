#[ allow( unused_imports ) ]
use super::*;

only_for_wtools!
{
  #[ allow( unused_imports ) ]
  use wtools::meta::*;
  #[ allow( unused_imports ) ]
  use wtools::former::Former;
}

only_for_local_module!
{
  #[ allow( unused_imports ) ]
  use meta_tools::*;
  #[ allow( unused_imports ) ]
  use former::Former;
}

// #[cfg( feature = "in_wtools" )]
// use wtools::meta::*;
// #[cfg( not( feature = "in_wtools" ) )]
// use meta_tools::*;
//
// #[cfg( feature = "in_wtools" )]
// use wtools::former::Former;
// #[cfg( not( feature = "in_wtools" ) )]
// use former::Former;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, PartialEq, Former )]
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
  hashmap_strings_1 : HashMap< String, String >,
  hashset_strings_1 : HashSet< String >,
}

//

tests_impls!
{
  #[ test ]
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
      hashmap_strings_1 : hmap!{},
      hashset_strings_1 : hset!{},
    };
    assert_eq!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
