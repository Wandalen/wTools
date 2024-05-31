use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl From< IsTransparent > for bool
{
  #[ inline( always ) ]
  fn from( src : IsTransparent ) -> Self
  {
    src.0
  }
}

include!( "./only_test/basic.rs" );
