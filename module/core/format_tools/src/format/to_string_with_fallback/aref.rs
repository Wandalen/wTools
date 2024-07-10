//!
//! Wrapper to wrap argument for trait `ToStringWithFallback`.
//!

// use core::fmt;
use core::ops::{ Deref };

/// Reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref< 'a, T, How, Fallback >
( pub Ref2< 'a, T, How, Fallback > )
where
  &'a T : Copy,
  T : ?Sized,
;


/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref2< 'a, T, How, Fallback >
( pub Ref3< 'a, T, How, Fallback > )
where
  &'a T : Copy,
  T : ?Sized,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref3< 'a, T, How, Fallback >
( pub Ref4< 'a, T, How, Fallback > )
where
  &'a T : Copy,
  T : ?Sized,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref4< 'a, T, How, Fallback >
( pub &'a T, ::core::marker::PhantomData< fn() -> ( How, Fallback ) > )
where
  ::core::marker::PhantomData< fn() -> ( How, Fallback ) > : Copy,
  &'a T : Copy,
  T : ?Sized,
;

impl< 'a, T, How, Fallback > Ref< 'a, T, How, Fallback >
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
    self.0.0.0.0
  }

}

impl< 'a, T, How, Fallback > Clone for Ref< 'a, T, How, Fallback >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    // xxx : self?
    Self( self.0 )
  }
}

impl< 'a, T, How, Fallback > Clone for Ref2< 'a, T, How, Fallback >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0 )
  }
}

impl< 'a, T, How, Fallback > Clone for Ref3< 'a, T, How, Fallback >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0 )
  }
}

impl< 'a, T, How, Fallback > Clone for Ref4< 'a, T, How, Fallback >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0, std::marker::PhantomData )
  }
}

impl< 'a, T, How, Fallback > Copy for Ref< 'a, T, How, Fallback > {}
impl< 'a, T, How, Fallback > Copy for Ref2< 'a, T, How, Fallback > {}
impl< 'a, T, How, Fallback > Copy for Ref3< 'a, T, How, Fallback > {}
impl< 'a, T, How, Fallback > Copy for Ref4< 'a, T, How, Fallback > {}

// impl< 'a, T, How, Fallback > AsRef< T > for Ref< 'a, T, How, Fallback >
// {
//   fn as_ref( &self ) -> &T
//   {
//     &self.0
//   }
// }

impl< 'a, T, How, Fallback > Deref for Ref< 'a, T, How, Fallback >
{
  type Target = Ref2< 'a, T, How, Fallback >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, How, Fallback > Deref for Ref2< 'a, T, How, Fallback >
{
  type Target = Ref3< 'a, T, How, Fallback >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, How, Fallback > Deref for Ref3< 'a, T, How, Fallback >
{
  type Target = Ref4< 'a, T, How, Fallback >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

// xxx2 : wrap into wrap

impl< 'a, T, How, Fallback > From< &'a T > for Ref< 'a, T, How, Fallback >
{
  fn from( src : &'a T ) -> Self
  {
    Ref( Ref2( Ref3( Ref4( src, std::marker::PhantomData ) ) ) )
  }
}
