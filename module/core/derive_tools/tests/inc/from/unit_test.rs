use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::From ) ]
struct UnitStruct;

include!( "./only_test/unit.rs" );
