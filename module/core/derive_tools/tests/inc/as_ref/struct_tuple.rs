use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct StructTuple( String, i32 );

include!( "./only_test/struct_tuple.rs" );
