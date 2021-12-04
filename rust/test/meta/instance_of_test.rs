
use wtest_basic::test_suite;
use instance_of as TheModule;

//

fn _implements_basic()
{

  let src = Box::new( true );
  assert_eq!( TheModule::implements!( src => Copy ), false );
  assert_eq!( TheModule::implements!( src => Clone ), true );

}

//

fn _instance_of_basic()
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
