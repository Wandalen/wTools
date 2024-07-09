#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( the_module::Index ) ]
struct StructMultipleNamed< T > 
{
  a : Vec< T >,
  #[ index ]
  b : Vec< T >,
}

include!( "./only_test/struct_multiple_named.rs" );

