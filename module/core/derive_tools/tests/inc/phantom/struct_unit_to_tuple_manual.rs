use std::marker::PhantomData;
use super::*;

// Expected that unit struct will be replaced with tuple struct with single phantom field
#[ allow( dead_code ) ]
struct StructUnit< T >( PhantomData< T > );

include!( "./only_test/struct_unit_to_tuple.rs" );