#[ allow( unused_imports ) ]
use super::*;

use the_module::{ from, From_1, From_2, Into1 };

#[ derive( Debug, PartialEq ) ]
struct Struct1( i32, i32 );

impl the_module::From_2< i32, i32 > for Struct1
{
  fn from_2( a : i32, b : i32 ) -> Self { Self( a, b ) }
}

impl From< ( i32, i32 ) > for Struct1
{
  #[ inline( always ) ]
  fn from( ( a, b ) : ( i32, i32 ) ) -> Self
  {
    Self::from_2( a, b )
  }
}

include!( "./only_test/from2_unnamed.rs" );
