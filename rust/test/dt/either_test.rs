use test_tools::*;

use super::TheModule;

//

fn basic()
{
  let left : TheModule::Either< _, () > = TheModule::Either::Left( 13 );
  a_id!( left.flip(), TheModule::Either::Right( 13 ) );
}

//

test_suite!
{
  basic,
}
