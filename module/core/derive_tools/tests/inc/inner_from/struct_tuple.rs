use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructTuple( String, i32 );

include!( "./only_test/struct_tuple.rs" );
