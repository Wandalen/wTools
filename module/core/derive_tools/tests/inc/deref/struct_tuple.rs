use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive ( Deref ) ]
struct StructTuple( String, i32 );

include!( "./only_test/struct_tuple.rs" );
