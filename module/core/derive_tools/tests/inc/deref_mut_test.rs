use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::Deref, the_module::DerefMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/deref_mut.rs" );
