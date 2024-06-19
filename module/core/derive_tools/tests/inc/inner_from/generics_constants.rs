use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct GenericsConstants< const N : usize >( i32 );

include!( "./only_test/generics_constants.rs" );
