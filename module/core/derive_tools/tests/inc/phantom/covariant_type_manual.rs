use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct CovariantType< T >
{
  a: T,
  _phantom: PhantomData< T >,
}

include!( "./only_test/covariant_type.rs" );