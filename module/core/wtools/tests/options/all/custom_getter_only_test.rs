
use test_tools::exposed::*;

//

tests_impls!
{
  #[ test ]
  fn basic()
  {
    // test.case( "basic" );

    let got = split().src( "abc" ).delimeter( "b" ).left( true ).form();
    let exp = split::Options
    {
      src : "abc",
      delimeter : "b",
      left : true,
    };
    a_id!( got, exp );

    use split::OptionsAdapter;
    a_id!( *got.left(), false );

    // zzz : uncoment later
    // let exp = vec![ "c", "a" ];
    // a_id!( got.perform().map( | e | String::from( e ) ).collect::< Vec< _ > >(), exp );
  }
}

//

tests_index!
{
  basic,
}
