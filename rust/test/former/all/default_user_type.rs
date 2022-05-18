#[ allow( unused_imports ) ]
use super::*;
use test_tools::dependencies::*;

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
// use wtools::former::Former;
// #[cfg( not( feature = "in_wtools" ) )]
// use former::Former;

//

fn test_user_type_with_default() -> anyhow::Result< () >
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

  assert_eq!( command, expected );

  Ok( () )
}

#[ test ]
fn user_type_test()
{
  test_user_type_with_default().unwrap();
}
