use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive ( Deref, DerefMut ) ]
struct GenericsTypesDefault< T = i32 >( T );

include!( "./only_test/generics_types_default.rs" );
