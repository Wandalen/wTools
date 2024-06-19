use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct ContravariantType< T >
{
  a: T,
  _phantom: PhantomData< T >,
}

include!( "./only_test/contravariant_type.rs" );