use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off ) ]
struct NamedDefaultOffReferenceOn< 'a >
{
  #[ not( on ) ]
  a : &'a bool,
  b : u8,
}

include!( "only_test/named_default_off_reference_on.rs" );
