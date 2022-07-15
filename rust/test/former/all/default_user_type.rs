#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

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

//

tests_impls!
{
  #[ test ]
  fn test_user_type_with_default()
  {
    #[derive( Debug, PartialEq, Default )]
    pub struct UserType
    {
      int : i32,
      uint : u32,
    }

    #[derive( Debug, PartialEq, Former )]
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
