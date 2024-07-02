use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::InnerFrom ) ]
struct StructWithManyFields( i32, bool );

include!( "./only_test/multiple.rs" );
