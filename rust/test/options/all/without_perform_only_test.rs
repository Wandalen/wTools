
use test_tools::*;

//

tests_impls!
{
  #[ test ]
  fn basic()
  {
    // test.case( "former + form()" );

    let got = split::former().src( "abc" ).delimeter( "b" ).form();
    let exp = split::Options
    {
      src : "abc",
      delimeter : "b",
      left : true,
    };
    a_id!( got, exp );

    // test.case( "split() + form()" );

    let got = split().src( "abc" ).delimeter( "b" ).form();
    let exp = split::Options
    {
      src : "abc",
      delimeter : "b",
      left : true,
    };
    a_id!( got, exp );

    // test.case( "split() + perform()" );

    let got = split().src( "abc" ).delimeter( "b" ).perform();
    let exp = split::Options
    {
      src : "abc",
      delimeter : "b",
      left : true,
    };
    a_id!( got, exp );
  }

  //

  #[ test ]
  fn derive()
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
  }
}

//

tests_index!
{
  basic,
  derive,
}
