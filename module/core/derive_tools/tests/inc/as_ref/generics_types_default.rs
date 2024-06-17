use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct GenericsTypesDefault< T = i32 >( T );

include!( "./only_test/generics_types_default.rs" );
