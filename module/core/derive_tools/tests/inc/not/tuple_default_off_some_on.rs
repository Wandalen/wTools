use super :: *;

#[ allow( dead_code ) ]
// #[ derive( the_module ::Not ) ]
// #[ not( off ) ]
struct TupleDefaultOffSomeOn( bool, u8 );

include!( "only_test/tuple_default_off_some_on.rs" );
