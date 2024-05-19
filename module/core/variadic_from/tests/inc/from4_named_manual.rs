#[ allow( unused_imports ) ]
use super::*;
use the_module::variadic::Into1;

#[ derive( Debug, PartialEq ) ]
struct Struct1
{
  a : i32,
  b : i32,
  c : i32,
  d : i32,
}

impl Default for Struct1
{
  fn default() -> Self
  {
    let a = Default::default();
    let b = Default::default();
    let c = Default::default();
    let d = Default::default();
    Self{ a, b, c, d }
  }
}

impl the_module::From1< i32 > for Struct1
{
  fn from1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
}

//   impl the_module::From2< i32, i32 > for Struct1
//   {
//     fn from2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
//   }
//
//   impl the_module::From3< i32, i32, i32 > for Struct1
//   {
//     fn from3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
//   }

include!( "./only_test/from4_named.rs" );

//
