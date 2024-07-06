//!
//! Wrapper to wrap argument for trait `_ToStringWithFallback`.
//!

// use core::fmt;
use core::ops::{ Deref };

/// Reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct ToStringWithFallbackRef< 'a, T, Marker >
( pub _ToStringWithFallbackRef< 'a, T, Marker > )
where
  &'a T : Copy,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct _ToStringWithFallbackRef< 'a, T, Marker >
( pub &'a T, ::core::marker::PhantomData< fn() -> Marker > )
where
  ::core::marker::PhantomData< fn( Marker ) > : Copy,
  &'a T : Copy,
;

impl< 'a, T, Marker > ToStringWithFallbackRef< 'a, T, Marker >
{

  // /// Just a constructor.
  // #[ inline( always ) ]
  // pub fn new( src : &'a T ) -> Self
  // {
  //   Self( src, ::core::marker::PhantomData )
  // }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> &'a T
  {
    self.0.0
  }

}

impl< 'a, T, Marker > Clone for ToStringWithFallbackRef< 'a, T, Marker >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0 )
  }
}

impl< 'a, T, Marker > Clone for _ToStringWithFallbackRef< 'a, T, Marker >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0, std::marker::PhantomData )
  }
}

impl< 'a, T, Marker > Copy for ToStringWithFallbackRef< 'a, T, Marker > {}
impl< 'a, T, Marker > Copy for _ToStringWithFallbackRef< 'a, T, Marker > {}

// impl< 'a, T, Marker > AsRef< T > for ToStringWithFallbackRef< 'a, T, Marker >
// {
//   fn as_ref( &self ) -> &T
//   {
//     &self.0
//   }
// }

impl< 'a, T, Marker > Deref for ToStringWithFallbackRef< 'a, T, Marker >
{
  type Target = _ToStringWithFallbackRef< 'a, T, Marker >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

// xxx2 : wrap into wrap

impl< 'a, T, Marker > From< &'a T > for ToStringWithFallbackRef< 'a, T, Marker >
{
  fn from( src : &'a T ) -> Self
  {
    ToStringWithFallbackRef( _ToStringWithFallbackRef( src, std::marker::PhantomData ) )
  }
}
