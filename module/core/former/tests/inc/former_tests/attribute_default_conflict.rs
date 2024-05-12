#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, Default, the_module::Former ) ]
pub struct Struct1
{
  #[ former( default = 31 ) ]
  pub int_1 : i32,
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
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
