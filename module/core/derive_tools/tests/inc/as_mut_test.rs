use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::AsMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/as_mut.rs" );
