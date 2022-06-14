#![ allow( deprecated ) ]
#![ allow( unused_imports ) ]
use super::*;

//

#[ cfg( feature = "use_std" ) ]
tests_impls!
{
  #[ test ]
  fn basic()
  {

    use std::error::Error;

    // test.case( "basic" );

    let err1 = TheModule::BasicError::new( "Some error" );
    a_id!( err1.to_string(), "Some error" );
    a_id!( err1.description(), "Some error" );
    a_id!( err1.msg(), "Some error" );
    a_id!( format!( "err1 : {}", err1 ), "err1 : Some error" );

    // test.case( "compare" );

    let err1 = TheModule::BasicError::new( "Some error" );
    let err2 = TheModule::BasicError::new( "Some error" );
    a_id!( err1, err2 );
    a_id!( err1.description(), err2.description() );

    // test.case( "clone" );

    let err1 = TheModule::BasicError::new( "Some error" );
    let err2 = err1.clone();
    a_id!( err1, err2 );
    a_id!( err1.description(), err2.description() );

  }

  //

  fn use1()
  {

    use std::error::Error as ErrorInterface;
    use TheModule::BasicError as Error;

    // test.case( "basic" );

    let err1 = Error::new( "Some error" );
    a_id!( err1.to_string(), "Some error" );
    a_id!( err1.description(), "Some error" );
    a_id!( err1.msg(), "Some error" );
    a_id!( format!( "err1 : {}", err1 ), "err1 : Some error" );

  }

  //

  fn use2()
  {
    use TheModule::{ BasicError, ErrorInterface };

    // test.case( "basic" );

    let err1 = BasicError::new( "Some error" );
    a_id!( err1.to_string(), "Some error" );
    a_id!( err1.description(), "Some error" );
    a_id!( err1.msg(), "Some error" );
    a_id!( format!( "err1 : {}", err1 ), "err1 : Some error" );
  }

  //

  fn use3()
  {

    use std::error::Error;

    // test.case( "basic" );

    let err1 = TheModule::BasicError::new( "Some error" );
    a_id!( err1.to_string(), "Some error" );
    a_id!( err1.description(), "Some error" );
    a_id!( err1.msg(), "Some error" );
    a_id!( format!( "err1 : {}", err1 ), "err1 : Some error" );

  }

  //

  fn err_basic()
  {

    // test.case( "basic" );
    let err = TheModule::err!( "abc" );
    a_id!( err.to_string(), "abc" );

    // test.case( "with args" );
    let err = TheModule::err!( "abc{}{}", "def", "ghi" );
    a_id!( err.to_string(), "abcdefghi" );

  }
}

//

#[ cfg( feature = "use_std" ) ]
tests_index!
{
  basic,
  use1,
  use2,
  use3,
  err_basic,
}
