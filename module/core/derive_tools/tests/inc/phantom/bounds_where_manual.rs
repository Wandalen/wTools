use std::
{
  fmt::Debug,
  marker::PhantomData,
};

#[ allow( dead_code ) ]
struct BoundsWhere< T, U >
  where
    T: ToString,
    U: Debug,
{
  _phantom: PhantomData< ( T, U ) >
}

include!( "./only_test/bounds_where.rs" );
