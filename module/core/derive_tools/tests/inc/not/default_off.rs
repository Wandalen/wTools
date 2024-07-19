use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off )]
#[debug]
struct DefaultOff
{
  a : bool,
  b : u8,
}

include!( "./only_test/default_off.rs" );
