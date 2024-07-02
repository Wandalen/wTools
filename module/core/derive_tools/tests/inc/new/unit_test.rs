use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::New ) ]
struct UnitStruct;

include!( "./only_test/unit.rs" );
