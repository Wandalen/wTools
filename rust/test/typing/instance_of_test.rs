
use wtest_basic::*;
use instance_of as TheModule;

//

fn implements_basic_test()
{

  let src = Box::new( true );
  assert_eq!( TheModule::implements!( src => Copy ), false );
  assert_eq!( TheModule::implements!( src => Clone ), true );

}

//

fn instance_of_basic_test()
{

  let src = Box::new( true );
  assert_eq!( TheModule::instance_of!( src => Copy ), false );
  assert_eq!( TheModule::instance_of!( src => Clone ), true );

}

//

test_suite!
{
  implements_basic,
  instance_of_basic,
}
