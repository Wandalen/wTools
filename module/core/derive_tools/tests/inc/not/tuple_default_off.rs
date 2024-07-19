use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off ) ]
struct TupleDefaultOff( bool, u8 );

include!( "only_test/tuple_default_off.rs" );
