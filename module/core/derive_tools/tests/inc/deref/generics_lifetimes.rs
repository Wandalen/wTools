use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

include!( "./only_test/generics_lifetimes.rs" );
