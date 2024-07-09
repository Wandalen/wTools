//!
//! Wrapper to wrap argument for trait `ToStringWith`.
//!

// use core::fmt;
// use core::ops::{ Deref };

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

impl< 'a, T, How > Ref2< 'a, T, How >
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
    self.0
  }

}

impl< 'a, T, How > Clone for Ref2< 'a, T, How >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0, std::marker::PhantomData )
  }
}

impl< 'a, T, How > Copy for Ref2< 'a, T, How > {}

impl< 'a, T, How > From< &'a T > for Ref2< 'a, T, How >
{
  fn from( src : &'a T ) -> Self
  {
    Ref2( src, std::marker::PhantomData )
  }
}
