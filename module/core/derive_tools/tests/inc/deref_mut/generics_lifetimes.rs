use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

include!( "./only_tests/generics_lifetimes.rs" );
