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

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct Age { age: u32 }

impl Default for Age
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self{ age: 0 }
  }
}

impl From< u32 > for Age
{
  #[ inline( always ) ]
  fn from( src : u32 ) -> Self
  {
    Self{ age: src }
  }
}

impl From< Age > for u32
{
  #[ inline( always ) ]
  fn from( src : Age ) -> Self
  {
    src.age
  }
}

include!( "./only_test/all.rs" );
