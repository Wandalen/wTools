
use wtest_basic::dependencies::*;

//

fn basic() -> anyhow::Result< () >
{

  // test.case( "basic" );

  let got = split().src( "abc" ).delimeter( "b" ).left( true )._form();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : false,
  };
  assert_eq!( got, exp );

  Ok( () )
}

//

#[ test ]
fn main_test() -> anyhow::Result< () >
{
  basic()?;
  Ok( () )
}
