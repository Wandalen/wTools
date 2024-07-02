use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct StructTupleEmpty< T >(  PhantomData< T > );

include!( "./only_test/struct_tuple_empty.rs" );