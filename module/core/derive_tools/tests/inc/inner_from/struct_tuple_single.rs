use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructTupleSingle( i32 );

include!( "./only_test/struct_tuple_single.rs" );
