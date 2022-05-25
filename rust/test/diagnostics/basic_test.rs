#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

tests_impls!
{

  #[ cfg( feature = "a_pretty" ) ]
  #[ test ]
  fn basic()
  {

    a_id!( "abc", "abc" );
    // a_id!( "abc", "abd" );

    // a_not_id!( "abc", "abc" );
    a_not_id!( "abc", "abd" );

  }

}

//

tests_index!
{
  basic,
}
