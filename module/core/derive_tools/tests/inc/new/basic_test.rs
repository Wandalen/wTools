use super::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::New ) ]
pub struct IsTransparent( bool );

include!( "./only_test/basic.rs" );
