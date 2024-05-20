
#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  #[ cfg( all( feature = "derive_from", feature = "derive_inner_from", feature = "derive_display", feature = "derive_from_str" ) ) ]
  fn samples()
  {
    use the_module::*;

    #[ derive( From, InnerFrom, Display, FromStr, PartialEq, Debug ) ]
    #[ display( "{a}-{b}" ) ]
    struct Struct1
    {
      a : i32,
      b : i32,
    }

    // derived InnerFrom
    let src = Struct1 { a : 1, b : 3 };
    let got : ( i32, i32 ) = src.into();
    let exp = ( 1, 3 );
    assert_eq!( got, exp );

    // derived From
    let src : Struct1 = ( 1, 3 ).into();
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

  #[ cfg( all( feature = "derive_from", feature = "derive_inner_from", feature = "derive_display" ) ) ]
  fn basic()
  {
    use the_module::*;

    #[ derive( From, InnerFrom, Display ) ]
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

  //

  #[ cfg( all( feature = "strum", feature = "strum_derive" ) ) ]
  fn enum_with_strum()
  {
    use the_module::{ EnumIter, IntoEnumIterator };

    #[ derive( EnumIter, Debug, PartialEq ) ]
    enum Foo
    {
      Bar,
      Baz
    }

    let mut iter = Foo::iter();
    a_id!( iter.next(), Some( Foo::Bar ) );
    a_id!( iter.next(), Some( Foo::Baz ) );
  }
}

//

tests_index!
{
  samples,
  basic,
  enum_with_strum,
}
