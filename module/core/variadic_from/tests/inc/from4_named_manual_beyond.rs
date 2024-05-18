#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named4()
{
  use the_module::{ Into1 };

  #[ derive( Default, Debug, PartialEq ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl the_module::wtools::From_1< i32 > for StructNamedFields
  {
    fn from_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
  }

  impl the_module::wtools::From_2< i32, i32 > for StructNamedFields
  {
    fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
  }

  impl the_module::wtools::From_3< i32, i32, i32 > for StructNamedFields
  {
    fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
  }

  // 0

  let got : StructNamedFields = the_module::from!();
  let exp = StructNamedFields{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  // 1

  let got : StructNamedFields = the_module::from!( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = the_module::from!( ( 13, ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = the_module::from!( ( ( 13, ), ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = 13.to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, ).to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, ), ).to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  // let got : StructNamedFields = 13.into();
  // let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  // a_id!( got, exp );

  // let got : StructNamedFields = ( 13, ).into();
  // let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  // a_id!( got, exp );

  // 2

  let got : StructNamedFields = the_module::from!( 0, 1 );
  let exp = StructNamedFields{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  let got : StructNamedFields = the_module::from!( ( 0, 1 ) );
  let exp = StructNamedFields{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  // 3

  let got : StructNamedFields = the_module::from!( 0, 1, 2 );
  let exp = StructNamedFields{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

  let got : StructNamedFields = the_module::from!( ( 0, 1, 2 ) );
  let exp = StructNamedFields{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

}

//

/// Into1 is auto implemented from From_1.
/// From_1< ( All, ) > is auto implemented for From_1< All >.
#[ test ]
fn from_tuple_1()
{
  use the_module::prelude::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl the_module::wtools::From_1< i32 > for StructNamedFields
  {
    fn from_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
  }

  let got : StructNamedFields = from!( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, ), ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, ), ) );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = 13.to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, ).to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, ), ).to();
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

}

//

/// Into1 is auto implemented from From_1.
/// From_1< ( All1, All2 ) > is auto implemented for From_2< All1, All2 >.
#[ test ]
fn from_tuple_from_from2()
{
  use the_module::prelude::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl the_module::wtools::From_2< i32, i32 > for StructNamedFields
  {
    fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
  }

  let got : StructNamedFields = from!( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_2( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, 14 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, 14 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, 14 ), ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 14, d : 14 };
  a_id!( got, exp );

}

//

/// Into1 is auto implemented from From_1.
/// From_1< ( All1, All2, All3 ) > is auto implemented for From_3< All1, All2, All3 >.
#[ test ]
fn from_tuple_from_from3()
{
  use the_module::prelude::*;

  #[ derive( Debug, PartialEq, Default ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl the_module::wtools::From_3< i32, i32, i32 > for StructNamedFields
  {
    fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
  }

  let got : StructNamedFields = from!( 13, 14, 15 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_3( 13, 14, 15 );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, 14, 15 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, 14, 15 ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, 14, 15 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, 14, 15 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14, 15 ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, 14, 15 ), ).to();
  let exp = StructNamedFields{ a : 13, b : 14, c : 15, d : 15 };
  a_id!( got, exp );

}
