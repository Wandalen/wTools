#[ allow( unused_imports ) ]
use super::*;

use the_module::prelude::*;

#[ derive( Debug, PartialEq ) ]
struct Struct1
{
  a : i32,
  b : i32,
}

impl the_module::From_2< i32, i32 > for Struct1
{
  fn from_2( a : i32, b : i32 ) -> Self { Self{ a : a, b : b } }
}

impl From< ( i32, i32 ) > for Struct1
{
  #[ inline( always ) ]
  fn from( ( a, b ) : ( i32, i32 ) ) -> Self
  {
    Self::from_2( a, b )
  }
}

include!( "./only_test/from2_named.rs" );
