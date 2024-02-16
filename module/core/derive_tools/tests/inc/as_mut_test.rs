use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::AsMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/as_mut.rs" );
