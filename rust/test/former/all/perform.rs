#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

only_for_wtools!
{
  #[ allow( unused_imports ) ]
  use wtools::meta::*;
  #[ allow( unused_imports ) ]
  use wtools::former::Former;
}

only_for_local_module!
{
  #[ allow( unused_imports ) ]
  use meta_tools::*;
  #[ allow( unused_imports ) ]
  use former::Former;
}

#[ derive( Debug, PartialEq, Former ) ]
#[ perform( fn perform1< 'a >() -> Option< &'a str > ) ]
pub struct Struct1
{
  #[ default( 31 ) ]
  pub int_1 : i32,
}

//

impl Struct1
{
  fn perform1< 'a >( &self ) -> Option< &'a str >
  {
    Some( "abc" )
  }
}

//

tests_impls!
{
  #[ test ]
  fn basic()
  {
    let got = Struct1::former().form();
    let expected = Struct1 { int_1 : 31 };
    a_id!( got, expected );

    let got = Struct1::former().perform();
    let expected = Some( "abc" );
    a_id!( got, expected );
  }
}

//

tests_index!
{
  basic,
}
