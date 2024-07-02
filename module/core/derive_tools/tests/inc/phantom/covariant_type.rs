use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct CovariantType< T >
{
  a: T,
}

include!( "./only_test/covariant_type.rs" );