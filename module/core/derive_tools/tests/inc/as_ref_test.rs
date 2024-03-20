use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::exposed::AsRef ) ]
pub struct IsTransparent( bool );

include!( "./only_test/as_ref.rs" );
