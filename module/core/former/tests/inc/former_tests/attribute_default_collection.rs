#[ allow( unused_imports ) ]
use super::*;

use collection_tools::HashMap;
use collection_tools::HashSet;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1
{

  #[ former( default = collection_tools::vec![ 1, 2, 3 ] ) ]
  vec_ints : Vec< i32 >,
  #[ former( default = collection_tools::hmap!{ 1 => 11 } ) ]
  hashmap_ints : HashMap< i32, i32 >,
  #[ former( default = collection_tools::hset!{ 11 } ) ]
  hashset_ints : HashSet< i32 >,

  #[ former( default = collection_tools::vec![ "abc".to_string(), "def".to_string() ] ) ]
  vec_strings : Vec< String >,
  #[ former( default = collection_tools::hmap!{ "k1".to_string() => "v1".to_string() } ) ]
  hashmap_strings : HashMap< String, String >,
  #[ former( default = collection_tools::hset!{ "k1".to_string() } ) ]
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
      vec_ints : collection_tools::vec![ 1, 2, 3 ],
      hashmap_ints : collection_tools::hmap!{ 1 => 11 },
      hashset_ints : collection_tools::hset!{ 11 },
      vec_strings : collection_tools::vec![ "abc".to_string(), "def".to_string() ],
      hashmap_strings : collection_tools::hmap!{ "k1".to_string() => "v1".to_string() },
      hashset_strings : collection_tools::hset!{ "k1".to_string() },
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
