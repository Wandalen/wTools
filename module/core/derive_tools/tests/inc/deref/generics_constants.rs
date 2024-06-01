use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct GenericsConstants< const N : usize >( i32 );

include!( "./only_tests/generics_constants.rs" );
