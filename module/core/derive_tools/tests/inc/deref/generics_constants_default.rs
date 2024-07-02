use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

include!( "./only_test/generics_constants_default.rs" );
