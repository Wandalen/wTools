use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::InnerFrom ) ]
pub struct UnitStruct;


include!( "./only_test/unit.rs" );
