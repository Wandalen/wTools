use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct StructTuple< T >( String, i32, PhantomData< T > );

include!( "./only_test/struct_tuple.rs" );