use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

include!( "./only_test/generics_lifetimes.rs" );
