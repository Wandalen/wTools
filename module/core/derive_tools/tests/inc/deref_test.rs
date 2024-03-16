use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::Deref ) ]
pub struct IsTransparent( bool );

include!( "./only_test/deref.rs" );
