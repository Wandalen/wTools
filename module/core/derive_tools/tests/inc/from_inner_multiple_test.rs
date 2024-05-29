use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::From ) ]
struct StructWithManyFields( i32, bool );

include!( "./only_test/from_inner_multiple.rs" );
