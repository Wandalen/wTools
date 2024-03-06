use super::*;

#[ derive( Debug, PartialEq, Eq, TheModule::InnerFrom ) ]
struct StructWithManyFields( i32, bool );

include!( "./only_test/inner_from_multiple.rs" );
