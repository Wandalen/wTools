use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct GenericsTypes< T >( T );

include!( "./only_test/generics_types.rs" );
