#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{
  fn test_user_type_with_default()
  {
    #[ derive( Debug, PartialEq, Default ) ]
    pub struct UserType
    {
      int : i32,
      uint : u32,
    }

    #[ derive( Debug, PartialEq, TheModule::Former ) ]
    pub struct Struct2
    {
      user : UserType,
      string : String,
    }
    let command = Struct2::former().form();

    let expected = Struct2
    {
      user : UserType { int : 0, uint : 0 },
      string : String::from( "" ),
    };

    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_user_type_with_default,
}
