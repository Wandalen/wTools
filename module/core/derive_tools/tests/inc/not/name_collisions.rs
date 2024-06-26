use super::*;

pub mod core {}
pub mod std {}

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct NameCollisions
{
  a : bool,
  b : u8,
}

include!( "./only_test/name_collisions.rs" );
