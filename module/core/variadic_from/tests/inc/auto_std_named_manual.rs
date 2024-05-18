#[ allow( unused_imports ) ]
use super::*;


#[ allow( unused_imports ) ]
use the_module::exposed::*;

#[ derive( Debug, PartialEq, Default ) ]
struct Struct1
{
  a : i32,
  b : i32,
}

impl the_module::From_1< i32 > for Struct1
{
  fn from_1( a : i32 ) -> Self { Self{ a : a, b : a } }
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
    Self { a, b }
  }
}

// Standard From and Into auto derive From_1 and To_1.

include!( "./only_test/from2_named.rs" );
include!( "./only_test/from2_std_named.rs" );
