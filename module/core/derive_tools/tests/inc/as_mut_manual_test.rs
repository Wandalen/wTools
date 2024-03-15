use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl AsMut< bool > for IsTransparent
{
  fn as_mut( &mut self ) -> &mut bool
  {
    &mut self.0
  }
}

include!( "./only_test/as_mut.rs" );
