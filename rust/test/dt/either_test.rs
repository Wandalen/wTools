use test_tools::*;

use super::TheModule;

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
