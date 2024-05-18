#[ allow( unused_imports ) ]
use super::*;

/// IMPORTANT: length of struct should always be larget by one than
/// maximum number of supported arguments by `VariadicFrom`.
/// Currently it's 3, but if the length will be increased test should be extended too.
///
/// `VariadicFrom` generates nothing in this case.
#[ test ]
fn from_named4()
{
  use the_module::{ Into1, VariadicFrom };

  #[ derive( Default, Debug, PartialEq, VariadicFrom ) ]
  // #[ debug ]
  struct Struct1
  (
    i32,
    i32,
    i32,
    i32,
  );

  impl the_module::From1< i32 > for Struct1
  {
    fn from1( a : i32 ) -> Self { Self( a, a, a, a ) }
  }

  impl the_module::From2< i32, i32 > for Struct1
  {
    fn from2( a : i32, b : i32 ) -> Self { Self( a, b, b, b ) }
  }

  impl the_module::From3< i32, i32, i32 > for Struct1
  {
    fn from3( a : i32, b : i32, c : i32 ) -> Self { Self( a, b, c, c ) }
  }

  // 0

  let got : Struct1 = the_module::from!();
  let exp = Struct1( 0, 0, 0, 0 );
  a_id!( got, exp );

  // 1

  let got : Struct1 = the_module::from!( 13 );
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( 13, ) );
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( ( 13, ), ) );
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  let got : Struct1 = 13.to();
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  let got : Struct1 = ( 13, ).to();
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  let got : Struct1 = ( ( 13, ), ).to();
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  // 2

  let got : Struct1 = the_module::from!( 0, 1 );
  let exp = Struct1( 0, 1, 1, 1 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( 0, 1 ) );
  let exp = Struct1( 0, 1, 1, 1 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( ( 0, 1 ), ) );
  let exp = Struct1( 0, 1, 1, 1 );
  a_id!( got, exp );

  let got : Struct1 = ( 0, 1 ).to();
  let exp = Struct1( 0, 1, 1, 1 );
  a_id!( got, exp );

  let got : Struct1 = ( ( 0, 1 ), ).to();
  let exp = Struct1( 0, 1, 1, 1 );
  a_id!( got, exp );

  // 3

  let got : Struct1 = the_module::from!( 0, 1, 2 );
  let exp = Struct1( 0, 1, 2, 2 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( 0, 1, 2 ) );
  let exp = Struct1( 0, 1, 2, 2 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( ( 0, 1, 2 ), ) );
  let exp = Struct1( 0, 1, 2, 2 );
  a_id!( got, exp );

  let got : Struct1 = ( 0, 1, 2 ).to();
  let exp = Struct1( 0, 1, 2, 2 );
  a_id!( got, exp );

  let got : Struct1 = ( ( 0, 1, 2 ), ).to();
  let exp = Struct1( 0, 1, 2, 2 );
  a_id!( got, exp );

}
