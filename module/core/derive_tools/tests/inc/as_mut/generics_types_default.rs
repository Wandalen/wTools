use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct GenericsTypesDefault< T = i32 >( T );

include!( "./only_test/generics_types_default.rs" );
