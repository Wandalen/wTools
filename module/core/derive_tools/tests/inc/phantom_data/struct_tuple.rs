use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom_data ]
struct StructTuple< T >( String, i32 );

include!( "./only_test/struct_tuple.rs" );