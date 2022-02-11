
use wtest_basic::*;
use iter_tools as TheModule;

//

fn _basic()
{

  use TheModule::*;

  // test.case( "basic" );
  let src = vec![ 1, 2, 3 ];
  let exp = ( vec![ 2, 3, 4 ], vec![ 0, 1, 2 ] );
  let got : ( Vec< _ >, Vec< _ > ) = src.iter().map( | e |
  {(
    e + 1,
    e - 1,
  )}).multiunzip();
  assert_eq!( got, exp );

}

//

test_suite!
{
  basic,
}
