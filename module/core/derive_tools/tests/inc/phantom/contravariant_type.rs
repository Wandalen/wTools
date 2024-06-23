use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct ContravariantType< T >
{
  a: T,
}

include!( "./only_test/contravariant_type.rs" );