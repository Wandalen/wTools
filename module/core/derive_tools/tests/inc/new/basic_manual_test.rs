use super::*;

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
