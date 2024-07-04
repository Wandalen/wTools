use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
struct StructTuple< T >( Vec< T >, u8, u8 );

include!( "./only_test/struct_tuple.rs" );
