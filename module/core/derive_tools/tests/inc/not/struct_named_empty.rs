use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct StructNamedEmpty{}

include!( "./only_test/struct_named_empty.rs" );
