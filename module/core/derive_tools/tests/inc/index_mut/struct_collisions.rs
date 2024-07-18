
#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]
use super::*;

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod a {}
pub mod b {}

#[ derive( the_module::IndexMut ) ]
#[ allow( dead_code ) ]
struct StructMultipleNamed< T > 
{
  a : Vec< T >,
  #[ index ]
  b : Vec< T >,
}

include!( "./only_test/struct_multiple_named.rs" );
