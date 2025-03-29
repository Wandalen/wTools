use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct TupleDefaultOnSomeOff( bool, #[ not( off ) ] u8);

include!( "only_test/tuple_default_on_some_off.rs" );
