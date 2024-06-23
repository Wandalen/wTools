use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructNamedEmpty{}

include!( "./only_test/struct_named_empty.rs" );
