use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

include!( "./only_test/generics_constants_default.rs" );
