use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct GenericsTypes< T >( T );

include!( "./only_test/generics_types.rs" );
