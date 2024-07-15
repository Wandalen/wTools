#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( the_module::IndexMut ) ]
struct StructTuple< T >
( 
   #[ index ]
   Vec< T >
);

include!( "./only_test/struct_tuple.rs" );
