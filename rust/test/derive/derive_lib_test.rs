
use wtest_basic::*;

// xxx qqq : removoe the feature?
#[cfg( feature = "in_wtools" )]
use wtools::derive as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use derive_tools as TheModule;

//

fn _basic()
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

  let src = Struct1 { a : 1, b : 3 };
  let got = format!( "{}", src );
  let exp = "1-3";
  assert_eq!( got, exp );

  // let src = Struct1 { a : 1, b : 3 };
  // let got : [ i32 ; 2 ] = src.into();
  // let exp = ( 1, 3 );
  // assert_eq!( got, exp );
  /* zzz : make it working */

}

//

test_suite!
{
  basic,
}
