use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct StructNamedEmpty{}

include!( "./only_test/struct_named_empty.rs" );
