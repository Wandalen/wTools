
use test_tools::*;

use super::TheModule;

// xxx : qqq : for Dima : removoe the feature? /* aaa : Dmytro : removed */

//

fn basic_test()
{
  use TheModule::*;

  #[ derive( From, Into, Display ) ]
  #[ display( "{a}-{b}" ) ]
  struct Struct1
  {
    a : i32,
    b : i32,
  }

  let src = Struct1 { a : 1, b : 3 };
  let got : ( i32, i32 ) = src.into();
  let exp = ( 1, 3 );
  assert_eq!( got, exp );

  // let src = Struct1 { a : 1, b : 3 };
  // let got : [ i32 ; 2 ] = src.into();
  // let exp = ( 1, 3 );
  // assert_eq!( got, exp );
  /* zzz : make it working */

  let src = Struct1 { a : 1, b : 3 };
  let got = format!( "{}", src );
  let exp = "1-3";
  assert_eq!( got, exp );

}

//

test_suite!
{
  basic,
}
