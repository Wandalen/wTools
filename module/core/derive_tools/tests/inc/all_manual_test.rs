use super::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl Default for IsTransparent
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( true )
  }
}

impl From< bool > for IsTransparent
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src )
  }
}

impl From< IsTransparent > for bool
{
  #[ inline( always ) ]
  fn from( src : IsTransparent ) -> Self
  {
    src.0
  }
}

impl core::ops::Deref for IsTransparent
{
  type Target = bool;
  #[ inline( always ) ]
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl core::ops::DerefMut for IsTransparent
{
  #[ inline( always ) ]
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

impl AsRef< bool > for IsTransparent
{
  fn as_ref( &self ) -> &bool
  {
    &self.0
  }
}

impl AsMut< bool > for IsTransparent
{
  fn as_mut( &mut self ) -> &mut bool
  {
    &mut self.0
  }
}

include!( "./only_test/all.rs" );
