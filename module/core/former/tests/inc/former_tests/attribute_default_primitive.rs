#[ allow( unused_imports ) ]
use super::*;

use collection_tools::HashMap;
use collection_tools::HashSet;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1
{
  #[ former( default = 31 ) ]
  pub int_1 : i32,
  #[ former( default = "abc" ) ]
  string_1 : String,
  #[ former( default = 31 ) ]
  int_optional_1 : Option< i32 >,
  #[ former( default = "abc" ) ]
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
      vec_1 : collection_tools::vec![],
      hashmap_1 : collection_tools::hmap!{},
      hashset_1 : collection_tools::hset!{},
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
