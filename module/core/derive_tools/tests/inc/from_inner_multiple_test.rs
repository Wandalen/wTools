use super::*;

#[ derive( Debug, PartialEq, Eq, TheModule::FromInner ) ]
struct StructWithManyFields( i32, bool );

include!( "./only_test/from_inner_multiple.rs" );
