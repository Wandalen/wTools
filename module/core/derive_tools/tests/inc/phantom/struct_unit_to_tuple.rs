use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct StructUnit< T >;

include!( "./only_test/struct_unit_to_tuple.rs" );