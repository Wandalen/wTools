use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::InnerFrom ) ]
pub struct UnitStruct;

// include!( "./manual/basic.rs" );
include!( "./only_test/inner_from.rs" );
