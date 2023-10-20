use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl From< bool > for IsTransparent
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct Age{ age: u32 }

impl From< u32 > for Age
{
  #[ inline( always ) ]
  fn from( value : u32 ) -> Self 
  {
      Self { age: value }
  }
}

include!( "./only_test/from_inner.rs" );
