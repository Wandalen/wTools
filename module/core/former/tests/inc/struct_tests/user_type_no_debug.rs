#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#[ allow( unused_imports ) ]
use super::*;

#[ allow( unused_imports ) ]
use the_module::Former;

//

tests_impls! {
  fn test_user_type_with_no_debug()
  {
    #[ derive( Default, PartialEq ) ]
    pub struct State
    {
      on : bool
    }

    #[ derive( PartialEq, the_module::Former ) ]
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

tests_index! {
  test_user_type_with_no_debug,
}
