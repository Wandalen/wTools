#![ allow( unused_imports ) ]
use super::*;

//

#[ cfg( feature = "error_untyped" ) ]
tests_impls!
{
  fn basic()
  {
    // test.case( "from parse usize error" );

    let err = the_module::for_app::anyhow!( "err" );
    a_id!( the_module::for_app::Error::is::< &str >( &err ), true );
    a_id!( err.is::< &str >(), true );
    a_id!( err.to_string(), "err" );
  }
}

//

#[ cfg( feature = "error_untyped" ) ]
tests_index!
{
  basic,
}
