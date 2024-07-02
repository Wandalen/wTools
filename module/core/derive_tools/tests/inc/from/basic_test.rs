use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::From ) ]
pub struct IsTransparent( bool );

// include!( "./manual/basic.rs" );
include!( "./only_test/basic.rs" );
