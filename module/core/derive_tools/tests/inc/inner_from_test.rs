use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::InnerFrom ) ]
pub struct IsTransparent( bool );

#[ derive( Debug, Clone, Copy, PartialEq, Default, TheModule::FromInner, TheModule::InnerFrom ) ]
pub struct Age {age: usize}

// include!( "./manual/basic.rs" );
include!( "./only_test/inner_from.rs" );
