//!
//! Wrapper to wrap argument for trait `_ToStringWithFallback`.
//!

// use core::fmt;
use core::ops::{ Deref };

/// Reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct ToStringWithFallbackRef< 'a, T, How, Fallback >
( pub _ToStringWithFallbackRef< 'a, T, How, Fallback > )
where
  &'a T : Copy,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct _ToStringWithFallbackRef< 'a, T, How, Fallback >
( pub &'a T, ::core::marker::PhantomData< fn() -> ( How, Fallback ) > )
where
  ::core::marker::PhantomData< fn( How, Fallback ) > : Copy,
  &'a T : Copy,
;

impl< 'a, T, How, Fallback > ToStringWithFallbackRef< 'a, T, How, Fallback >
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

impl< 'a, T, How, Fallback > Clone for ToStringWithFallbackRef< 'a, T, How, Fallback >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0 )
  }
}

impl< 'a, T, How, Fallback > Clone for _ToStringWithFallbackRef< 'a, T, How, Fallback >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0, std::marker::PhantomData )
  }
}

impl< 'a, T, How, Fallback > Copy for ToStringWithFallbackRef< 'a, T, How, Fallback > {}
impl< 'a, T, How, Fallback > Copy for _ToStringWithFallbackRef< 'a, T, How, Fallback > {}

// impl< 'a, T, How, Fallback > AsRef< T > for ToStringWithFallbackRef< 'a, T, How, Fallback >
// {
//   fn as_ref( &self ) -> &T
//   {
//     &self.0
//   }
// }

impl< 'a, T, How, Fallback > Deref for ToStringWithFallbackRef< 'a, T, How, Fallback >
{
  type Target = _ToStringWithFallbackRef< 'a, T, How, Fallback >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

// xxx2 : wrap into wrap

impl< 'a, T, How, Fallback > From< &'a T > for ToStringWithFallbackRef< 'a, T, How, Fallback >
{
  fn from( src : &'a T ) -> Self
  {
    ToStringWithFallbackRef( _ToStringWithFallbackRef( src, std::marker::PhantomData ) )
  }
}
