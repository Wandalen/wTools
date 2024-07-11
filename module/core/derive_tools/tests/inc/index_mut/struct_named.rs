#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( the_module::IndexMut ) ]
struct StructNamed< T > 
{
  #[ index ]
  a : Vec< T >,
}

include!( "./only_test/struct_named.rs" );
