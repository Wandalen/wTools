
use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct StructUnit;

include!( "./only_test/struct_unit.rs" );
