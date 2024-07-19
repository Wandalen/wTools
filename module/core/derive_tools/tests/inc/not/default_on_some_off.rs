use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct DefaultOnSomeOff
{
  a : bool,
  #[ not( off ) ]
  b : u8,
}

include!( "./only_test/default_on_some_off.rs" );
