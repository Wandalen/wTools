use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct GenericsTypes< T >( T );

include!( "./only_tests/generics_types.rs" );
