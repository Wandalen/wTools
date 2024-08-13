use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct StructTuple( bool, u8 );

include!( "./only_test/struct_tuple.rs" );
