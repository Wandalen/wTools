use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive ( Deref ) ]
struct StructUnit;

include!( "./only_test/struct_unit.rs" );
