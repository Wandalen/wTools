
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

  // test.case( "split() + perform()" );

  let got = split().src( "abc" ).delimeter( "b" ).perform();
  let exp = vec![ "a", "c" ];
  assert_eq!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

  // test.case( "bool" );

  #[ allow( unused_imports ) ]
  use split::OptionsAdapter;

  let got = split().src( "abc" ).delimeter( "b" ).left( true ).perform();
  let exp = vec![ "a", "c" ];
  assert_eq!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

  let got = split().src( "abc" ).delimeter( "b" ).left( false ).perform();
  let exp = vec![ "c", "a" ];
  assert_eq!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

  Ok( () )
}

//

fn derive() -> anyhow::Result< () >
{

  // test.case( "is PartialOrd implemented" );

  let got = split().src( "abc" ).delimeter( "b" ).form();
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

fn prelude() -> anyhow::Result< () >
{

  // test.case = "prelude";
  {
    use split::prelude::*;
    let got = split().src( "abc" ).delimeter( "b" ).form();
    assert_eq!( got.src(), "abc" );
  }

  // test.case = "SplitOptionsAdapter";
  {
    use split::prelude::SplitOptionsAdapter;
    let got = split().src( "abc" ).delimeter( "b" ).form();
    assert_eq!( got.src(), "abc" );
  }

  Ok( () )
}

//

fn accessor() -> anyhow::Result< () >
{

  use split::prelude::*;
  let mut got = split().src( "abc" ).delimeter( "b" ).form();

  assert_eq!( got.src(), "abc" );
  *got.src_mut() = "def";
  assert_eq!( got.src(), "def" );

  assert_eq!( *got.left(), true );
  *got.left_mut() = false;
  assert_eq!( *got.left(), false );

  Ok( () )
}

//

#[ test ]
fn main_test() -> anyhow::Result< () >
{
  basic()?;
  derive()?;
  prelude()?;
  accessor()?;
  Ok( () )
}
