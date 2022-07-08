#![ allow( unused_imports ) ]
use super::*;

//

// fn unit_or_err() -> Result< (), TheModule::for_app::Error >
// {
//   TheModule::for_app::Error::from( Err( "err" ) )
// }

//

#[ cfg( feature = "error_handling_for_app" ) ]
tests_impls!
{
  fn basic()
  {
    // test.case( "from parse usize error" );

    let err = TheModule::for_app::anyhow!( "err" );
    a_id!( TheModule::for_app::Error::is::< &str >( &err ), true );
    a_id!( err.is::< &str >(), true );
    a_id!( err.to_string(), "err" );
  }
}

//

#[ cfg( feature = "error_handling_for_app" ) ]
tests_index!
{
  basic,
}
