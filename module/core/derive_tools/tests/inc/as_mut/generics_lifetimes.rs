use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

include!( "./only_test/generics_lifetimes.rs" );
