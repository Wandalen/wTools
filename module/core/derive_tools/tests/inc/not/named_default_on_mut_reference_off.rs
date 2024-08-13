use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct NamedDefaultOnMutReferenceOff< 'a >
{
  #[ not( off ) ]
  a : &'a bool,
  b : u8,
}

include!( "only_test/named_default_on_mut_reference_off.rs" );
