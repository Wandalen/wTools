use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct StructTupleEmpty();

include!( "./only_test/struct_tuple_empty.rs" );
