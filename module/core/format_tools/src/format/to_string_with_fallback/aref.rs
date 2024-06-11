//!
//! Wrapper to wrap argument for trait `_ToStringWithFallback`.
//!

// use core::fmt;
use core::ops::{ Deref };

// /// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
// pub trait ToStringWithFallbackRef< 'a, T, Marker >
// {
//   /// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
//   fn into_string_with_fallback_ref( self ) -> ToStringWithFallbackRef< 'a, T, Marker >;
// }
//
// impl< 'a, T, Marker > ToStringWithFallbackRef< 'a, T, Marker > for &'a T
// {
//   #[ inline( always ) ]
//   fn into_string_with_fallback_ref( self ) -> ToStringWithFallbackRef< 'a, T, Marker >
//   {
//     ToStringWithFallbackRef::< 'a, T, Marker >::new( self )
//   }
// }

/// Transparent reference wrapper emphasizing a specific aspect of identity of its internal type.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct ToStringWithFallbackRef< 'a, T, Marker >( pub &'a T, ::core::marker::PhantomData< fn() -> Marker > )
where
  ::core::marker::PhantomData< fn( Marker ) > : Copy,
  &'a T : Copy,
;

impl< 'a, T, Marker > Clone for ToStringWithFallbackRef< 'a, T, Marker >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self::new( self.0 )
  }
}

impl< 'a, T, Marker > Copy for ToStringWithFallbackRef< 'a, T, Marker > {}

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

// impl< 'a, T, Marker > From< ToStringWithFallbackRef< 'a, T, Marker > > for &'a T
// {
//   fn from( wrapper : ToStringWithFallbackRef< 'a, T, Marker > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

// impl< 'a, T, Marker > Default for ToStringWithFallbackRef< 'a, T, Marker >
// where
//   T : Default,
// {
//   fn default() -> Self
//   {
//     ToStringWithFallbackRef( &T::default() )
//   }
// }

// impl< 'a, T, Marker > fmt::Debug for ToStringWithFallbackRef< 'a, T, Marker >
// where
//   T : fmt::Debug,
// {
//   fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
//   {
//     f.debug_struct( "ToStringWithFallbackRef" )
//     .field( "0", &self.0 )
//     .finish()
//   }
// }
