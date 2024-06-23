use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct StructTupleEmpty();

include!( "./only_test/struct_tuple_empty.rs" );
