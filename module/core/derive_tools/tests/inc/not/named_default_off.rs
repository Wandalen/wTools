use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off ) ]
struct NamedDefaultOff
{
  a : bool,
  b : u8,
}

include!( "only_test/named_default_off.rs" );
