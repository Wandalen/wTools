use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct StructNamed< T >
{
  a : String,
  b : i32,
}

include!( "./only_test/struct_named.rs" );