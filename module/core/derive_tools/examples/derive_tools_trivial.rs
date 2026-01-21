//! Demonstrates basic usage of `Display` and `FromStr` derive macros.
//!
//! This example shows how to derive `Display` and `FromStr` traits for a simple struct,
//! enabling automatic conversion between the struct and its string representation.
//! The format uses `:` as separator to avoid conflicts with negative number signs.

fn main()
{
  #[ cfg( all(
    feature = "derive_from",
    feature = "derive_inner_from",
    feature = "derive_display",
    feature = "derive_from_str"
  ) ) ]
  {
    use derive_tools::*;

    #[ derive( Display, FromStr, PartialEq, Debug ) ]
    #[ display( "{a}:{b}" ) ]
    struct Struct1
    {
      a: i32,
      b: i32,
    }

    // derived Display
    let src = Struct1 { a: 1, b: 3 };
    let got = format!( "{src}" );
    let exp = "1:3";
    println!( "{got}" );
    assert_eq!( got, exp );

    // derived FromStr
    use core::str::FromStr;
    let src = Struct1::from_str( "1:3" );
    let exp = Ok( Struct1 { a: 1, b: 3 } );
    assert_eq!( src, exp );
  }
}
