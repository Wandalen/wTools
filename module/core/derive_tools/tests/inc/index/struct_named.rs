use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
struct StructNamed< T > 
{
  a : T,
  b : T,
}

include!( "./only_test/struct_named.rs" );
