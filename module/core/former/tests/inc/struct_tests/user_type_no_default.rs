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
//   use wtools::the_module::Former;
// }
//
// only_for_terminal_module!
// {
//   #[ allow( unused_imports ) ]
//   use meta_tools::*;
//   #[ allow( unused_imports ) ]
//   use the_module::Former;
// }

//

tests_impls!
{
  fn test_user_type_with_no_default()
  {
    #[ derive( Debug, PartialEq ) ]
    pub enum State
    {
      On,
      Off,
    }

    #[ derive( Debug, PartialEq, the_module::Former ) ]
    pub struct Device
    {
      device : String,
      state : State,
    }

    let device = Device::former()
    .state( State::On )
    .form();

    let expected = Device
    {
      device : "".to_string(),
      state : State::On,
    };

    a_id!( device, expected );
  }

  //

  #[ should_panic ]
  fn test_user_type_with_no_default_throwing()
  {
    #[ derive( Debug, PartialEq ) ]
    pub enum State
    {
      On,
      Off,
    }

    #[ derive( Debug, PartialEq, the_module::Former ) ]
    pub struct Device
    {
      device : String,
      state : State,
    }
    let device = Device::former().form();
  }
}

//

tests_index!
{
  test_user_type_with_no_default,
  test_user_type_with_no_default_throwing,
}
