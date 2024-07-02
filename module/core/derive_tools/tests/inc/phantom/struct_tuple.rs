use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct StructTuple< T >( String, i32 );

include!( "./only_test/struct_tuple.rs" );