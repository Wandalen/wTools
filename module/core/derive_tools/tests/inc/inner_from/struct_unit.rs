use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructUnit;

include!( "./only_test/struct_unit.rs" );
