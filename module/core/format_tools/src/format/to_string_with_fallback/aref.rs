//!
//! Wrapper to wrap argument for trait `_ToStringWithFallback`.
//!

// use core::fmt;
use core::ops::{ Deref };

/// Transparent reference wrapper emphasizing a specific aspect of identity of its internal type.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct ToStringWithFallbackRef< 'a, T, Marker >( pub &'a T, ::core::marker::PhantomData< fn() -> Marker > )
where
  ::core::marker::PhantomData< fn( Marker ) > : Copy,
  &'a T : Copy,
;

impl< 'a, T, Marker > ToStringWithFallbackRef< 'a, T, Marker >
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new( src : &'a T ) -> Self
  {
    Self( src, ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> &'a T
  {
    self.0
  }

}

impl< 'a, T, Marker > Clone for ToStringWithFallbackRef< 'a, T, Marker >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self::new( self.0 )
  }
}

impl< 'a, T, Marker > Copy for ToStringWithFallbackRef< 'a, T, Marker > {}

impl< 'a, T, Marker > AsRef< T > for ToStringWithFallbackRef< 'a, T, Marker >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< 'a, T, Marker > Deref for ToStringWithFallbackRef< 'a, T, Marker >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, Marker > From< &'a T > for ToStringWithFallbackRef< 'a, T, Marker >
{
  fn from( src : &'a T ) -> Self
  {
    ToStringWithFallbackRef::new( src )
  }
}
