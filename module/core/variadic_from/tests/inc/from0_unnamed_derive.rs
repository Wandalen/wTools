#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
struct Struct1( i32, i32, i32, i32 );

impl Default for Struct1
{
  fn default() -> Self
  {
    let a = Default::default();
    let b = Default::default();
    let c = Default::default();
    let d = Default::default();
    Self( a, b, c, d )
  }
}

include!( "./only_test/from0.rs" );

