
#[cfg( feature = "in_wtools" )]
use wtools::test::*;
#[cfg( not( feature = "in_wtools" ) )]
use wtest_basic::*;

include!( "./dynamic/basic.rs" );

//

fn basic_test()
{

  // let t = trybuild::TestCases::new();
  // t.pass( "../../../rust/test/test/dynamic/basic.rs" );

}

//

test_suite!
{
  basic,
}
