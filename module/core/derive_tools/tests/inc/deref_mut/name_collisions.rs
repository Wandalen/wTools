#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]

use ::core::ops::Deref;
use derive_tools::{ Deref, DerefMut };

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod FromString {}
pub mod FromPair {}
pub mod FromBin {}

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct NameCollisions
{
  a : i32,
  b : String,
}

include!( "./only_test/name_collisions.rs" );
