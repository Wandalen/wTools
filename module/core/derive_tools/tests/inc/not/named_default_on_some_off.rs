use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct NamedDefaultOnSomeOff
{
  a : bool,
  #[ not( off ) ]
  b : u8,
}

include!( "only_test/named_default_on_some_off.rs" );
