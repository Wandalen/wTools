use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::exposed::InnerFrom ) ]
pub struct UnitStruct;


include!( "./only_test/inner_from_unit.rs" );
