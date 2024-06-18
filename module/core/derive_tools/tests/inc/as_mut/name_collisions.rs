#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]

use derive_tools::AsMut;

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod FromString {}
pub mod FromPair {}
pub mod FromBin {}

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct NameCollisions
{
  a : i32,
  b : String,
}

include!( "./only_test/name_collisions.rs" );
