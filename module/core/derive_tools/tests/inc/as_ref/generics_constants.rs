use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct GenericsConstants< const N : usize >( i32 );

include!( "./only_test/generics_constants.rs" );
