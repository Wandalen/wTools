#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct0
{
  pub int_1 : i32,
}

// #[ derive( Debug, PartialEq ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
#[ derive( Debug, PartialEq, the_module::Former ) ]
#[ perform( fn perform1< 'a >() -> Option< &'a str > ) ]
pub struct Struct1
{
  pub int_1 : i32,
}

// == begin of generated

// == end of generated

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

  fn basecase()
  {

    let got = Struct0::former().form();
    let expected = Struct0 { int_1 : 0 };
    a_id!( got, expected );

    let got = Struct0::former().perform();
    let expected = Struct0 { int_1 : 0 };
    a_id!( got, expected );

  }

  fn basic()
  {

    let got = Struct1::former().form();
    let expected = Struct1 { int_1 : 0 };
    a_id!( got, expected );

    let got = Struct1::former().perform();
    let expected = Some( "abc" );
    a_id!( got, expected );

  }

}

//

tests_index!
{
  basecase,
  basic,
}
