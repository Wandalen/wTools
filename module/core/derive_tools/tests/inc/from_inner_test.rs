use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::FromInner ) ]
pub struct IsTransparent( bool );

// include!( "./manual/basic.rs" );
include!( "./only_test/from_inner.rs" );
