use std::
{
  fmt::Debug,
  marker::PhantomData,
};

#[ allow( dead_code ) ]
struct BoundsInlined< T: ToString, U: Debug >
{
  _phantom: PhantomData< ( T, U ) >,
}

include!( "./only_test/bounds_inlined.rs" );