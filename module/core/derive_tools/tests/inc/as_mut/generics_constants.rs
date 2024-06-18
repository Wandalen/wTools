use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct GenericsConstants< const N : usize >( i32 );

include!( "./only_test/generics_constants.rs" );
