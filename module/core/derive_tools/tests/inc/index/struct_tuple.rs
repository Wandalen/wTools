use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
struct StructTuple< T >
( 
   #[ index ]
   Vec< T >
);

include!( "./only_test/struct_tuple.rs" );
