use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

include!( "./only_tests/generics_lifetimes.rs" );
