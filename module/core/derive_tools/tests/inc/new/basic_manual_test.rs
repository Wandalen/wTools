use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl IsTransparent
{
  #[ inline( always ) ]
  fn new( src : bool ) -> Self
  {
    Self( src )
  }
}

include!( "./only_test/basic.rs" );
