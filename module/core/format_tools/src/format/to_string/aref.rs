//!
//! Wrapper to wrap argument for trait `ToStringWith`.
//!

// use core::fmt;
use core::ops::{ Deref };

/// Reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct Ref< 'a, T, How >
( pub _Ref< 'a, T, How > )
where
  &'a T : Copy,
  T : ?Sized,
;

/// Internal reference wrapper to make into string conversion with fallback.
#[ allow( missing_debug_implementations ) ]
#[ repr( transparent ) ]
pub struct _Ref< 'a, T, How >
( pub &'a T, ::core::marker::PhantomData< fn() -> How > )
where
  ::core::marker::PhantomData< fn() -> How > : Copy,
  &'a T : Copy,
  T : ?Sized,
;

impl< 'a, T, How > Ref< 'a, T, How >
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

impl< 'a, T, How > Clone for Ref< 'a, T, How >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0 )
  }
}

impl< 'a, T, How > Clone for _Ref< 'a, T, How >
{
  #[ inline( always ) ]
  fn clone( &self ) -> Self
  {
    Self( self.0, std::marker::PhantomData )
  }
}

impl< 'a, T, How > Copy for Ref< 'a, T, How > {}
impl< 'a, T, How > Copy for _Ref< 'a, T, How > {}

// impl< 'a, T, How > AsRef< T > for Ref< 'a, T, How >
// {
//   fn as_ref( &self ) -> &T
//   {
//     &self.0
//   }
// }

impl< 'a, T, How > Deref for Ref< 'a, T, How >
{
  type Target = _Ref< 'a, T, How >;
  fn deref( &self ) -> &Self::Target
  {
    // panic!( "deref" );
    &self.0
  }
}

impl< 'a, T, How > From< &'a T > for Ref< 'a, T, How >
{
  fn from( src : &'a T ) -> Self
  {
    Ref( _Ref( src, std::marker::PhantomData ) )
  }
}
