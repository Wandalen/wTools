#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1
{

  #[ former( default = collection_tools::vec![ 1, 2, 3 ] ) ]
  #[ former( default = collection_tools::vec![ 2, 3, 4 ] ) ]
  vec_ints : Vec< i32 >,

}

//

tests_impls!
{
  fn test_complex()
  {
    let command = Struct1::former().form();
    let expected = Struct1
    {
      vec_ints : collection_tools::vec![ 2, 3, 4 ],
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
