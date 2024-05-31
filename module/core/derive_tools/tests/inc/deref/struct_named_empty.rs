use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct StructNamedEmpty{}

include!( "./only_tests/struct_named_empty.rs" );
