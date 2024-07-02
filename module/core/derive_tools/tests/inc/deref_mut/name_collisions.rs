#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]

use ::core::ops::Deref;
use derive_tools::{ DerefMut };

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod FromString {}
pub mod FromPair {}
pub mod FromBin {}

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
struct NameCollisions
{
  a : i32,
  b : String,
}

impl Deref for NameCollisions
{
  type Target = i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.a
  }
}

include!( "./only_test/name_collisions.rs" );
