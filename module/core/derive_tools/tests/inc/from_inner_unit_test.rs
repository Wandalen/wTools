use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::exposed::FromInner ) ]
struct UnitStruct;

include!( "./only_test/from_inner_unit.rs" );
