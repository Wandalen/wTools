
// use test_tools::*;
use super::*;

//

tests_impls!
{

  #[ test ]
  fn basic()
  {
    use TheModule::string::indentation;

    /* test.case( "basic" ) */
    {
      let src = "a\nbc";
      let exp = "---a\n---bc";
      let got = indentation( "---", src, "" );
      a_id!( got, exp );
    }

    /* test.case( "empty string" ) */
    {
      let src = "";
      let exp = "---";
      let got = indentation( "---", src, "" );
      a_id!( got, exp );
    }

    /* test.case( "two empty string" ) */
    {
      let src = "\n";
      let exp = "---\n---";
      let got = indentation( "---", src, "" );
      a_id!( got, exp );
    }

  }

}

//

tests_index!
{
  basic,
}
