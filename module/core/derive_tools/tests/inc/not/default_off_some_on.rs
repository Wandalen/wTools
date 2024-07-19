use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off )]
struct DefaultOffSomeOn
{
  a : bool,
  #[ not( on ) ]
  b : u8,
}

include!( "./only_test/default_off_some_on.rs" );
