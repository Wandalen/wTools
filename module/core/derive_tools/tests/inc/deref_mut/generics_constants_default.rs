use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

include!( "./only_test/generics_constants_default.rs" );
