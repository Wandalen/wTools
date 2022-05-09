use test_tools::*;

#[cfg( feature = "in_wtools" )]
use wtools::meta as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use meta_tools as TheModule;

//

fn basic_test()
{
  let left : TheModule::Either< _, () > = TheModule::Either::Left( 13 );
  assert_eq!( left.flip(), TheModule::Either::Right( 13 ) );
}

//

test_suite!
{
  basic,
}
