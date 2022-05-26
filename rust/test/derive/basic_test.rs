
use test_tools::*;
use super::TheModule;

//

tests_impls!
{
  #[ test ]
  fn basic()
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
    a_id!( got, exp );

    // let src = Struct1 { a : 1, b : 3 };
    // let got : [ i32 ; 2 ] = src.into();
    // let exp = ( 1, 3 );
    // a_id!( got, exp );
    /* zzz : make it working */

    let src = Struct1 { a : 1, b : 3 };
    let got = format!( "{}", src );
    let exp = "1-3";
    a_id!( got, exp );
  }
}

//

tests_index!
{
  basic,
}
