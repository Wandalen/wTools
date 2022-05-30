
use test_tools::*;
#[ allow( unused_imports ) ]
use super::TheModule;

//

tests_impls!
{

  #[ cfg( all( feature = "derive_from", feature = "derive_into", feature = "derive_display", feature = "derive_from_str" ) ) ]
  #[ test ]
  fn samples()
  {
    use TheModule::*;

    #[ derive( From, Into, Display, FromStr, PartialEq, Debug ) ]
    #[ display( "{a}-{b}" ) ]
    struct Struct1
    {
      a : i32,
      b : i32,
    }

    // derived Into
    let src = Struct1 { a : 1, b : 3 };
    let got : ( i32, i32 ) = src.into();
    let exp = ( 1, 3 );
    assert_eq!( got, exp );

    // derived Display
    let src = Struct1 { a : 1, b : 3 };
    let got = format!( "{}", src );
    let exp = "1-3";
    println!( "{}", got );
    assert_eq!( got, exp );

    // derived FromStr
    use std::str::FromStr;
    let src = Struct1::from_str( "1-3" );
    let exp = Ok( Struct1 { a : 1, b : 3 } );
    assert_eq!( src, exp );

  }

  //

  #[ cfg( all( feature = "derive_from", feature = "derive_into", feature = "derive_display" ) ) ]
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
  samples,
  basic,
}
