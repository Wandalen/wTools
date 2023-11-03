use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::InnerFrom ) ]
pub struct UnitStruct;

include!( "./only_test/inner_from.rs" );
