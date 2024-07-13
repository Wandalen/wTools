//!
//! Wrapper to wrap argument for trait `ToStringWith`.
//!

// zzz : qqq : write derive for this with variable length
use core::ops::{ Deref };

/// Reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref< 'a, T, How >
( pub Ref2< 'a, T, How > )
where
  &'a T : Copy,
  T : ?Sized,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref2< 'a, T, How >
( pub &'a T, ::core::marker::PhantomData< fn() -> How > )
where
  ::core::marker::PhantomData< fn() -> How > : Copy,
  &'a T : Copy,
  T : ?Sized,
;

impl< 'a, T, How > Ref< 'a, T, How >
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> &'a T
  {
    self.0.0
  }

}

impl< 'a, T, How > Clone for Ref< 'a, T, How >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    *self
  }
}

impl< 'a, T, How > Clone for Ref2< 'a, T, How >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    *self
  }
}

impl< 'a, T, How > Copy for Ref< 'a, T, How > {}
impl< 'a, T, How > Copy for Ref2< 'a, T, How > {}

impl< 'a, T, How > Deref for Ref< 'a, T, How >
{
  type Target = Ref2< 'a, T, How >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, How > From< &'a T > for Ref< 'a, T, How >
where
  T : ?Sized,
{
  fn from( src : &'a T ) -> Self
  {
    Ref( Ref2( src, std::marker::PhantomData ) )
  }
}
