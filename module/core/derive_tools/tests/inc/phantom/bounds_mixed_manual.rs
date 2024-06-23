use std::
{
  fmt::Debug,
  marker::PhantomData,
};

#[ allow( dead_code ) ]
struct BoundsMixed< T: ToString, U >
where
  U: Debug,
{
  _phantom: PhantomData< ( T, U ) >,
}

include!( "./only_test/bounds_mixed.rs" );