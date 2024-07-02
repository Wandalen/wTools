use super::*;

pub mod std {}
pub mod core {}
pub mod marker {}

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct NameCollisions< T >
{
  a : String,
  b : i32,
}

include!( "./only_test/name_collisions.rs" );