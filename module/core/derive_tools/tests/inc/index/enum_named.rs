use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
enum EnumNamed < T >
{
  A { a : T },
 }

include!( "./only_test/enum_named.rs" );
