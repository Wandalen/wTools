#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( the_module::IndexMut ) ]
#[ index( name = b ) ]
struct StructMultipleNamed< T > 
{
  a : Vec< T >,
  b : Vec< T >,
}

include!( "./only_test/struct_multiple_named.rs" );


