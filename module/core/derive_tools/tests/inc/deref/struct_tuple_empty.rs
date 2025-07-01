use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive ( Deref ) ]
struct StructTupleEmpty();

include!( "./only_test/struct_tuple_empty.rs" );
