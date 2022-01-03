
use former::Former;
use std::collections::HashMap;
use std::collections::HashSet;
use maplit::*;

#[derive( Debug, PartialEq, Former )]
pub struct Struct1
{

  #[ default( vec![ 1, 2, 3 ] ) ]
  vec_ints : Vec< i32 >,
  #[ default( hashmap!{ 1 => 11 } ) ]
  hashmap_ints : HashMap< i32, i32 >,
  #[ default( hashset!{ 11 } ) ]
  hashset_ints : HashSet< i32 >,

  #[ default( vec![ "abc".to_string(), "def".to_string() ] ) ]
  vec_strings : Vec< String >,
  #[ default( hashmap!{ "k1".to_string() => "v1".to_string() } ) ]
  hashmap_strings : HashMap< String, String >,
  #[ default( hashset!{ "k1".to_string() } ) ]
  hashset_strings : HashSet< String >,

}

//

fn test_complex() -> anyhow::Result< () >
{

  let command = Struct1::former().form();

  let expected = Struct1
  {
    vec_ints : vec![ 1, 2, 3 ],
    hashmap_ints : hashmap!{ 1 => 11 },
    hashset_ints : hashset!{ 11 },
    vec_strings : vec![ "abc".to_string(), "def".to_string() ],
    hashmap_strings : hashmap!{ "k1".to_string() => "v1".to_string() },
    hashset_strings : hashset!{ "k1".to_string() },
  };
  assert_eq!( command, expected );

  Ok( () )
}

//

#[ test ]
fn main_test()
{
  test_complex().unwrap();
}
