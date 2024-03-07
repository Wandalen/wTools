#[ allow( unused_imports ) ]
use super::*;

// #[ allow( unused_imports ) ]
// use test_tools::exposed::*;
//
// only_for_aggregating_module!
// {
//   #[ allow( unused_imports ) ]
//   use wtools::meta::*;
//   #[ allow( unused_imports ) ]
//   use wtools::former::Former;
// }
//
// only_for_terminal_module!
// {
//   #[ allow( unused_imports ) ]
//   use meta_tools::*;
//   #[ allow( unused_imports ) ]
//   use former::Former;
// }

//

tests_impls!
{
  fn test_user_type_with_no_debug()
  {
    #[ derive( Default, PartialEq ) ]
    pub struct State
    {
      on : bool
    }

    #[ derive( PartialEq, TheModule::Former ) ]
    pub struct Device
    {
      device : String,
      state : State,
    }

    let device = Device::former()
    .form();

    let expected = Device
    {
      device : "".to_string(),
      state : State { on : false },
    };

    assert!( device == expected );
  }
}

//

tests_index!
{
  test_user_type_with_no_debug,
}
