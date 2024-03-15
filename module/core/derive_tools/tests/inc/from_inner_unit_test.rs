use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::FromInner ) ]
struct UnitStruct;

include!( "./only_test/from_inner_unit.rs" );
