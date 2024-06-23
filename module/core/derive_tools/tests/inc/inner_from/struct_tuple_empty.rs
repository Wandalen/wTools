use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructTupleEmpty();

include!( "./only_test/struct_tuple_empty.rs" );
