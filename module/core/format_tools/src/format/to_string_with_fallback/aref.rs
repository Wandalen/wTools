//!
//! Wrapper to wrap argument for trait `ToStringWithFallback`.
//!

use core::ops::{ Deref };

/// Reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref< 'a, T, How, Fallback1, Fallback2 >
( pub Ref2< 'a, T, How, Fallback1, Fallback2 > )
where
  &'a T : Copy,
  T : ?Sized,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref2< 'a, T, How, Fallback1, Fallback2 >
( pub Ref3< 'a, T, How, Fallback1, Fallback2 > )
where
  &'a T : Copy,
  T : ?Sized,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref3< 'a, T, How, Fallback1, Fallback2 >
( pub &'a T, ::core::marker::PhantomData< fn() -> ( How, Fallback1, Fallback2 ) > )
where
  &'a T : Copy,
  T : ?Sized,
;

impl< 'a, T, How, Fallback1, Fallback2 > Ref< 'a, T, How, Fallback1, Fallback2 >
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> &'a T
  {
    self.0.0.0
  }

}

impl< 'a, T, How, Fallback1, Fallback2 > Clone for Ref< 'a, T, How, Fallback1, Fallback2 >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    *self
  }
}

impl< 'a, T, How, Fallback1, Fallback2 > Clone for Ref2< 'a, T, How, Fallback1, Fallback2 >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    *self
  }
}

impl< 'a, T, How, Fallback1, Fallback2 > Clone for Ref3< 'a, T, How, Fallback1, Fallback2 >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    *self
  }
}

impl< 'a, T, How, Fallback1, Fallback2 > Copy for Ref< 'a, T, How, Fallback1, Fallback2 > {}
impl< 'a, T, How, Fallback1, Fallback2 > Copy for Ref2< 'a, T, How, Fallback1, Fallback2 > {}
impl< 'a, T, How, Fallback1, Fallback2 > Copy for Ref3< 'a, T, How, Fallback1, Fallback2 > {}

impl< 'a, T, How, Fallback1, Fallback2 > Deref for Ref< 'a, T, How, Fallback1, Fallback2 >
{
  type Target = Ref2< 'a, T, How, Fallback1, Fallback2 >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, How, Fallback1, Fallback2 > Deref for Ref2< 'a, T, How, Fallback1, Fallback2 >
{
  type Target = Ref3< 'a, T, How, Fallback1, Fallback2 >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, How, Fallback1, Fallback2 > Deref for Ref3< 'a, T, How, Fallback1, Fallback2 >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, How, Fallback1, Fallback2 > From< &'a T > for Ref< 'a, T, How, Fallback1, Fallback2 >
{
  fn from( src : &'a T ) -> Self
  {
    Ref( Ref2( Ref3( src, std::marker::PhantomData ) ) )
  }
}
