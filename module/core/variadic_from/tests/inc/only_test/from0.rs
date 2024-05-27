#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from0()
{

  // - from2

  let got : Struct1 = from!();
  let exp = Struct1{};
  a_id!( got, exp );

  let got : Struct1 = Struct1::default();
  let exp = Struct1{};
  a_id!( got, exp );

  let got : Struct1 = Default::default();
  let exp = Struct1{};
  a_id!( got, exp );

  // - from unit

  let got : Struct1 = from!( () );
  let exp = Struct1{};
  a_id!( got, exp );

  let got : Struct1 = from!( ( (), ) );
  let exp = Struct1{};
  a_id!( got, exp );

  let got : Struct1 = ().to();
  let exp = Struct1{};
  a_id!( got, exp );

  let got : Struct1 = ( (), ).to();
  let exp = Struct1{};
  a_id!( got, exp );

  // - std from unit

  let got : Struct1 = ().into();
  let exp = Struct1{};
  a_id!( got, exp );

  let got : Struct1 = From::from( () );
  let exp = Struct1{};
  a_id!( got, exp );

}
