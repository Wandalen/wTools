use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct StructNamedEmpty< T > {}

include!( "./only_test/struct_named_empty.rs" );