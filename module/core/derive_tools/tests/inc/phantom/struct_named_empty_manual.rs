use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct StructNamedEmpty< T >
{
  _phantom : PhantomData< T >,
}

include!( "./only_test/struct_named_empty.rs" );