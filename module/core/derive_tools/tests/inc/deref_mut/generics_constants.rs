use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct GenericsConstants< const N : usize >( i32 );

include!( "./only_test/generics_constants.rs" );
