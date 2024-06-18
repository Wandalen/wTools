use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct StructTuple( String, i32 );

include!( "./only_test/struct_tuple.rs" );
