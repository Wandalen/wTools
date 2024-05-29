use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl AsRef< bool > for IsTransparent
{
  fn as_ref( &self ) -> &bool
  {
    &self.0
  }
}

include!( "./only_test/as_ref.rs" );
