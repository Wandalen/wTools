
use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index, Debug) ]
struct StructNamed<T>
{
  a : T,
}

include!( "./only_test/struct_named.rs" );
