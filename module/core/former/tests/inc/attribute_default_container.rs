#[ allow( unused_imports ) ]
use super::*;

use std::collections::HashMap;
use std::collections::HashSet;

#[ derive( Debug, PartialEq, TheModule::Former ) ]
pub struct Struct1
{

  #[ default( vec![ 1, 2, 3 ] ) ]
  vec_ints : Vec< i32 >,
  #[ default( hmap!{ 1 => 11 } ) ]
  hashmap_ints : HashMap< i32, i32 >,
  #[ default( hset!{ 11 } ) ]
  hashset_ints : HashSet< i32 >,

  #[ default( vec![ "abc".to_string(), "def".to_string() ] ) ]
  vec_strings : Vec< String >,
  #[ default( hmap!{ "k1".to_string() => "v1".to_string() } ) ]
  hashmap_strings : HashMap< String, String >,
  #[ default( hset!{ "k1".to_string() } ) ]
  hashset_strings : HashSet< String >,

}

//

tests_impls!
{
  fn test_complex()
  {
    let command = Struct1::former().form();
    let expected = Struct1
    {
      vec_ints : vec![ 1, 2, 3 ],
      hashmap_ints : hmap!{ 1 => 11 },
      hashset_ints : hset!{ 11 },
      vec_strings : vec![ "abc".to_string(), "def".to_string() ],
      hashmap_strings : hmap!{ "k1".to_string() => "v1".to_string() },
      hashset_strings : hset!{ "k1".to_string() },
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
