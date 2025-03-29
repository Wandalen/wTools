use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct TupleDefaultOnMutReferenceOff< 'a >( #[ not( off ) ] &'a bool, u8);

include!( "only_test/tuple_default_on_mut_reference_off.rs" );
