#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;


#[ derive( the_module::Index, the_module::IndexMut ) ]
struct StructMultipleTuple< T >
(
  bool,
  #[ index ]
  #[ index_mut ]
  Vec< T >
);

include!( "./only_test/struct_multiple_tuple.rs" );

