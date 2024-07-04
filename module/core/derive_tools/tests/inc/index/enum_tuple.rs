use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
enum EnumTuple < T >
{
  A( T ),
  B( T )
}

include!( "./only_test/enum_tuple.rs" );

