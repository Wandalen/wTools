use std::marker::PhantomData;
use super::*;

#[ allow( dead_code ) ]
struct StructUnit< T >( PhantomData< T > );

include!( "./only_test/struct_unit_to_tuple.rs" );