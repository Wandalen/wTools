use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

include!( "./only_test/generics_constants_default.rs" );
