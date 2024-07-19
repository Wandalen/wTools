use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off ) ]
struct TupleDefaultOffReferenceOn< 'a >( #[ not( on ) ] &'a bool, u8 );

include!( "./only_test/tuple_default_off_reference_on.rs" );
