use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
enum EnumNamed < T >
{
  A { a : T, b : T },
  B { a : T, b : T }
}

include!( "./only_test/enum_named.rs" );
