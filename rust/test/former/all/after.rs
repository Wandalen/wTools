
use wtest_basic::dependencies::*;

#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
use former::Former;

#[ derive( Debug, PartialEq, Former ) ]
#[ form_after( fn after1< 'a >() -> Option< &'a str > ) ]
pub struct Struct1
{
  #[ default( 31 ) ]
  pub int_1 : i32,
}

//

impl Struct1
{
  fn after1< 'a >( &self ) -> Option< &'a str >
  {
    Some( "abc" )
  }
}

//

fn basic() -> anyhow::Result< () >
{

  let got = Struct1::former().form();
  let expected = Some( "abc" );
  assert_eq!( got, expected );

  let got = Struct1::former()._form();
  let expected = Struct1 { int_1 : 31 };
  assert_eq!( got, expected );

  Ok( () )
}

//

#[ test ]
fn main_test() -> anyhow::Result< () >
{
  basic()?;
  Ok( () )
}
