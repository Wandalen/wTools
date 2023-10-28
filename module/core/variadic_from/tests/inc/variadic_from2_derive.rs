#[ allow( unused_imports ) ]
use super::*;

/// Standard From and Into implemented for From_1.
#[ test ]
fn std_from_and_into_derive()
{
  #[ allow( unused_imports ) ]
  use TheModule::exposed::*;

  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
  }

  //

  include!( "./only_test/variadic_from2_named.rs" );

  //

  let got : StructNamedFields = From::from( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).into();
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

}

/// Standard From and Into auto derive From_1 and To_1.
#[ test ]
fn auto_from_std_from_and_into()
{
  use TheModule::exposed::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
  }

  //

  // impl TheModule::wtools::From_2< i32, i32 > for StructNamedFields
  // {
  //   fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b } }
  // }

  impl From< ( i32, i32 ) > for StructNamedFields
  {
    #[ inline( always ) ]
    fn from( ( a, b ) : ( i32, i32 ) ) -> Self
    {
      Self { a, b }
    }
  }

  //

  let got : StructNamedFields = From::from( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).into();
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  //

//   let got : StructNamedFields = from!( 13, 14 );
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = StructNamedFields::from_2( 13, 14 );
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = from!( ( 13, 14 ) );
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = StructNamedFields::from_1( ( 13, 14 ) );
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = from!( ( ( 13, 14 ), ) );
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, 14 ), ) );
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = ( 13, 14 ).to();
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = ( ( 13, 14 ), ).to();
//   let exp = StructNamedFields{ a : 13, b : 14 };
//   a_id!( got, exp );

  //

}
