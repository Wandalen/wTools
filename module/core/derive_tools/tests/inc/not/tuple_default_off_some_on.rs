use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
#[ not( off ) ]
struct TupleDefaultOffSomeOn( bool, #[ not( on ) ] u8 );

include!( "only_test/tuple_default_off_some_on.rs" );
