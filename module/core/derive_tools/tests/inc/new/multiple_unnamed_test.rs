use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::New ) ]
struct StructWithManyFields( i32, bool );

include!( "./only_test/multiple_unnamed.rs" );
