//! qqq : write proper description
use variadic_from::exposed::*;

fn main()
{
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
  }

  let got : StructNamedFields = From::from( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  assert_eq!( got, exp );

  let got : StructNamedFields = from!( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14 };
  assert_eq!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).to();
  let exp = StructNamedFields{ a : 13, b : 14 };
  assert_eq!( got, exp );

}
