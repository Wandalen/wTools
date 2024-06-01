use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive ( Deref, DerefMut ) ]
struct StructTuple( String, i32 );

include!( "./only_test/struct_tuple.rs" );
