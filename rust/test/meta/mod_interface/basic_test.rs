use test_tools::*;

#[cfg( feature = "in_wtools" )]
use wtools::meta as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use mod_interface as TheModule;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

//

fn fn_name_test()
{

  // assert!( false );

}

//

test_suite!
{
  fn_name,
}
