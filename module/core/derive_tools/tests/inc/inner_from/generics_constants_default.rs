use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

include!( "./only_test/generics_constants_default.rs" );
