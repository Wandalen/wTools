use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

include!( "./only_test/generics_lifetimes.rs" );
