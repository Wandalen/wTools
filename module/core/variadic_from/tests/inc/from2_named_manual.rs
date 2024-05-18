#[ allow( unused_imports ) ]
use super::*;

use variadic_from::{ from, From1, From2, Into1 };

#[ derive( Debug, PartialEq ) ]
struct Struct1
{
  a : i32,
  b : i32,
}

impl variadic_from::From2< i32, i32 > for Struct1
{
  fn from2( a : i32, b : i32 ) -> Self { Self{ a : a, b : b } }
}

impl From< ( i32, i32 ) > for Struct1
{
  #[ inline( always ) ]
  fn from( ( a, b ) : ( i32, i32 ) ) -> Self
  {
    Self::from2( a, b )
  }
}

include!( "./only_test/from2_named.rs" );
