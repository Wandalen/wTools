#![allow(deprecated)]

use wtest_basic::*;

//

fn _basic()
{

  use werror as TheModule;
  use std::error::Error;

  // test.case( "basic" );

  let err1 = TheModule::Error::new( "Some error" );
  assert_eq!( err1.to_string(), "Some error" );
  assert_eq!( err1.description(), "Some error" );
  assert_eq!( err1.msg(), "Some error" );
  assert_eq!( format!( "err1 : {}", err1 ), "err1 : Some error" );

  // test.case( "compare" );

  let err1 = TheModule::Error::new( "Some error" );
  let err2 = TheModule::Error::new( "Some error" );
  assert_eq!( err1, err2 );
  assert_eq!( err1.description(), err2.description() );

  // test.case( "clone" );

  let err1 = TheModule::Error::new( "Some error" );
  let err2 = err1.clone();
  assert_eq!( err1, err2 );
  assert_eq!( err1.description(), err2.description() );

}

//

fn _use1()
{

  use std::error::Error as ErrorTrait;
  use werror::Error;

  // test.case( "basic" );

  let err1 = Error::new( "Some error" );
  assert_eq!( err1.to_string(), "Some error" );
  assert_eq!( err1.description(), "Some error" );
  assert_eq!( err1.msg(), "Some error" );
  assert_eq!( format!( "err1 : {}", err1 ), "err1 : Some error" );

}

//

fn _use2()
{

  use werror::*;

  // test.case( "basic" );

  let err1 = Error::new( "Some error" );
  assert_eq!( err1.to_string(), "Some error" );
  assert_eq!( err1.description(), "Some error" );
  assert_eq!( err1.msg(), "Some error" );
  assert_eq!( format!( "err1 : {}", err1 ), "err1 : Some error" );

}

//

fn _use3()
{

  use std::error::Error;

  // test.case( "basic" );

  let err1 = werror::Error::new( "Some error" );
  assert_eq!( err1.to_string(), "Some error" );
  assert_eq!( err1.description(), "Some error" );
  assert_eq!( err1.msg(), "Some error" );
  assert_eq!( format!( "err1 : {}", err1 ), "err1 : Some error" );

}

//

test_suite!
{
  basic,
  use1,
  use2,
  use3,
}
