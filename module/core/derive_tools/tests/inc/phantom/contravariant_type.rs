use super :: *;
use core ::marker ::PhantomData;

#[ allow( dead_code ) ]
// #[ the_module ::phantom ]
struct ContravariantType< T >
{
  a: T,
  _phantom: PhantomData< T >,
}

include!( "./only_test/contravariant_type.rs" );
