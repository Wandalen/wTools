use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct GenericsTypes< T >( T );

include!( "./only_test/generics_types.rs" );
