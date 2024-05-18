#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
struct StructNamedFields
{
  a : i32,
  b : i32,
  c : i32,
  d : i32,
}

impl the_module::wtools::From_0 for StructNamedFields
{
  fn from_0() -> Self
  {
    let a = Default::default();
    let b = Default::default();
    let c = Default::default();
    let d = Default::default();
    Self{ a, b, c, d }
  }
}

impl the_module::wtools::From_1< i32 > for StructNamedFields
{
  fn from_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
}

//   impl the_module::wtools::From_2< i32, i32 > for StructNamedFields
//   {
//     fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
//   }
//
//   impl the_module::wtools::From_3< i32, i32, i32 > for StructNamedFields
//   {
//     fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
//   }

include!( "./only_test/from4_named.rs" );

//
