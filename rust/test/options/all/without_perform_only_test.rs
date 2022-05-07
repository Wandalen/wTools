
use test_tools::dependencies::*;

//

fn basic() -> anyhow::Result< () >
{

  // test.case( "former + form()" );

  let got = split::former().src( "abc" ).delimeter( "b" ).form();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( got, exp );

  // test.case( "split() + form()" );

  let got = split().src( "abc" ).delimeter( "b" ).form();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( got, exp );

  // test.case( "split() + perform()" );

  let got = split().src( "abc" ).delimeter( "b" ).perform();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( got, exp );

  Ok( () )
}

//

fn derive() -> anyhow::Result< () >
{

  // test.case( "is PartialOrd implemented" );

  let got = split().src( "abc" ).delimeter( "b" ).perform();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert!( !( got > exp ) && !( got < exp ) );

  Ok( () )
}

//

#[ test ]
fn main_test() -> anyhow::Result< () >
{
  basic()?;
  derive()?;
  Ok( () )
}
