use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive ( Deref ) ]
struct StructTupleEmpty();

include!( "./only_tests/struct_tuple_empty.rs" );
