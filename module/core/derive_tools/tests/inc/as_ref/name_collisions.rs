#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]

use derive_tools::AsRef;

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod FromString {}
pub mod FromPair {}
pub mod FromBin {}

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct NameCollisions
{
  a : i32,
  b : String,
}

include!( "./only_test/name_collisions.rs" );
