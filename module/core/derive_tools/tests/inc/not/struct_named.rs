use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct StructNamed
{
  a : bool,
  b : u8,
}

include!( "./only_test/struct_named.rs" );
