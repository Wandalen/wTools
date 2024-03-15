use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, TheModule::Deref ) ]
pub struct IsTransparent( bool );

include!( "./only_test/deref.rs" );
