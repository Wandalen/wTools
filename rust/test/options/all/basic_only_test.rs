
//

fn basic() -> anyhow::Result< () >
{

  // test.case( "former + _form()" );

  let got = split::former().src( "abc" ).delimeter( "b" )._form();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( got, exp );

  // test.case( "split() + _form()" );

  let got = split().src( "abc" ).delimeter( "b" )._form();
  let exp = split::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( got, exp );

  // test.case( "split() + form()" );

  let got = split().src( "abc" ).delimeter( "b" ).form();
  let exp = vec![ "a", "c" ];
  assert_eq!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

  // test.case( "is PartialOrd implemented" );

  let got = split().src( "abc" ).delimeter( "b" )._form();
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
  Ok( () )
}
