#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]

use ::core::ops::Deref;
use derive_tools::Deref;

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod FromString {}
pub mod FromPair {}
pub mod FromBin {}

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct NameCollisions
{
  a : i32,
  b : String,
}

include!( "./only_tests/name_collisions.rs" );
