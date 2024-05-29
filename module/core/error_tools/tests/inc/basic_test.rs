#![ allow( deprecated ) ]
#![ allow( unused_imports ) ]
use super::*;

//

#[ cfg( not( feature = "no_std" ) ) ]
tests_impls!
{
  fn basic()
  {
    use std::error::Error;

    // test.case( "basic" );

    let err1 = the_module::BasicError::new( "Some error" );
    a_id!( err1.to_string(), "Some error" );
    a_id!( err1.description(), "Some error" );
    a_id!( err1.msg(), "Some error" );
    a_id!( format!( "err1 : {}", err1 ), "err1 : Some error" );

    // test.case( "compare" );

    let err1 = the_module::BasicError::new( "Some error" );
    let err2 = the_module::BasicError::new( "Some error" );
    a_id!( err1, err2 );
    a_id!( err1.description(), err2.description() );

    // test.case( "clone" );

    let err1 = the_module::BasicError::new( "Some error" );
    let err2 = err1.clone();
    a_id!( err1, err2 );
    a_id!( err1.description(), err2.description() );
  }

  //

  fn use1()
  {
    use std::error::Error as ErrorInterface;
    use the_module::BasicError as Error;

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
    use the_module::{ BasicError, ErrorInterface };

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

    let err1 = the_module::BasicError::new( "Some error" );
    a_id!( err1.to_string(), "Some error" );
    a_id!( err1.description(), "Some error" );
    a_id!( err1.msg(), "Some error" );
    a_id!( format!( "err1 : {}", err1 ), "err1 : Some error" );
  }

  //

  fn err_basic()
  {
    // test.case( "basic" );
    let err : the_module::BasicError = the_module::err!( "abc" );
    a_id!( err.to_string(), "abc" );

    // test.case( "with args" );
    let err : the_module::BasicError = the_module::err!( "abc{}{}", "def", "ghi" );
    a_id!( err.to_string(), "abcdefghi" );
  }

  //

  fn sample()
  {
    #[ cfg( not( feature = "no_std" ) ) ]
    fn f1() -> the_module::Result< () >
    {
      let _read = std::fs::read_to_string( "Cargo.toml" )?;
      Err( the_module::BasicError::new( "Some error" ).into() )
      // the_module::BasicError::new( "Some error" ).into()
      // zzz : make it working maybe
    }

    #[ cfg( not( feature = "no_std" ) ) ]
    {
      let err = f1();
      println!( "{err:#?}" );
      // < Err(
      // <    BasicError {
      // <        msg: "Some error",
      // <    },
      // < )
    }
  }


}

//

#[ cfg( not( feature = "no_std" ) ) ]
tests_index!
{
  basic,
  use1,
  use2,
  use3,
  err_basic,
  sample,
}
