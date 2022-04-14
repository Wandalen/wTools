
use wtest_basic::dependencies::*;

//

fn basic() -> anyhow::Result< () >
{

  // test.case( "basic" );

  let got = split().src( "abc" ).delimeter( "b" ).left( true ).form();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( got, exp );

  use split::OptionsAdapter;
  assert_eq!( *got.left(), false );

  // xxx : uncoment later
  // let exp = vec![ "c", "a" ];
  // assert_eq!( got.perform().map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

  Ok( () )
}

//

#[ test ]
fn main_test() -> anyhow::Result< () >
{
  basic()?;
  Ok( () )
}
