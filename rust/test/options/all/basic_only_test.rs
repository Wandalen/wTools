
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

    // test.case( "split() + perform()" );

    let got = split().src( "abc" ).delimeter( "b" ).perform();
    let exp = vec![ "a", "c" ];
    a_id!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

    // test.case( "bool" );

    #[ allow( unused_imports ) ]
    use split::OptionsAdapter;

    let got = split().src( "abc" ).delimeter( "b" ).left( true ).perform();
    let exp = vec![ "a", "c" ];
    a_id!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );

    let got = split().src( "abc" ).delimeter( "b" ).left( false ).perform();
    let exp = vec![ "c", "a" ];
    a_id!( got.map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );
  }

  //

  #[ test ]
  fn derive()
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
  }

  //

  #[ test ]
  fn prelude()
  {
    // test.case = "prelude";
    {
      use split::prelude::*;
      let got = split().src( "abc" ).delimeter( "b" ).form();
      a_id!( got.src(), "abc" );
    }

    // test.case = "SplitOptionsAdapter";
    {
      use split::prelude::SplitOptionsAdapter;
      let got = split().src( "abc" ).delimeter( "b" ).form();
      a_id!( got.src(), "abc" );
    }
  }

  //

  #[ test ]
  fn accessor()
  {
    use split::prelude::*;
    let mut got = split().src( "abc" ).delimeter( "b" ).form();

    a_id!( got.src(), "abc" );
    *got.src_mut() = "def";
    a_id!( got.src(), "def" );

    a_id!( *got.left(), true );
    *got.left_mut() = false;
    a_id!( *got.left(), false );
  }
}

//

tests_index!
{
  basic,
  derive,
  prelude,
  accessor,
}
