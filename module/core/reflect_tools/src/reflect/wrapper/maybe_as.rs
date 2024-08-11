//!
//! It's often necessary to wrap something inot a local structure and this file contains wrapper of `Option< Cow< 'a, T > >`.
//!

use core::fmt;
use std::borrow::Cow;
use core::ops::{ Deref };

/// Universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
pub struct MaybeAs< 'a, T, Marker >( pub Option< Cow< 'a, T > >, ::core::marker::PhantomData< fn() -> Marker > )
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
;

impl< 'a, T, Marker > MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{

  /// Check is it borrowed.
  #[ inline( always ) ]
  pub fn is_borrowed( &self ) -> bool
  {
    if self.0.is_none()
    {
      return false;
    }
    match self.0.as_ref().unwrap()
    {
      Cow::Borrowed( _ ) => true,
      Cow::Owned( _ ) => false,
    }
  }

  /// Check does it have some value.
  #[ inline( always ) ]
  pub fn is_some( &self ) -> bool
  {
    return self.0.is_some()
  }

  /// Constructor returning none.
  #[ inline( always ) ]
  pub fn none() -> Self
  {
    Self( None, ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new( src : < T as std::borrow::ToOwned >::Owned ) -> Self
  {
    Self( Some( Cow::Owned( src ) ), ::core::marker::PhantomData )
  }

  // xxx : review
  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new_with_ref( src : &'a T ) -> Self
  {
    Self( Some( Cow::Borrowed( src ) ), ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new_with_inner( src : Option< Cow< 'a, T > > ) -> Self
  {
    Self( src, ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> Option< Cow< 'a, T > >
  {
    self.0
  }

}

// impl< 'a, T, Marker > std::borrow::ToOwned for MaybeAs< 'a, T, Marker >
// where
//   T : std::borrow::ToOwned + ?Sized,
// {
//   type Owned = MaybeAs< 'static, T::Owned, Marker >;
//
//   fn to_owned( &self ) -> Self::Owned
//   {
//     MaybeAs
//     (
//       self.0.as_ref().map( | cow | Cow::Owned( cow.to_owned() ) ),
//       std::marker::PhantomData
//     )
//   }
// }

impl< 'a, T, Marker > Clone for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{
  fn clone( &self ) -> Self
  {
    Self( self.0.clone(), ::core::marker::PhantomData )
  }
}

impl< 'a, T, Marker > AsRef< Option< Cow< 'a, T > > > for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{
  fn as_ref( &self ) -> &Option< Cow< 'a, T > >
  {
    &self.0
  }
}

impl< 'a, T, Marker > Deref for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{
  type Target = Option< Cow< 'a, T > >;
  fn deref( &self ) -> &Option< Cow< 'a, T > >
  {
    self.as_ref()
  }
}

impl< 'a, T, Marker > From< Cow< 'a, T > >
for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{
  fn from( src : Cow< 'a, T > ) -> Self
  {
    MaybeAs::new_with_inner( Some( src ) )
  }
}

impl< 'a, T, Marker > From< Option< Cow< 'a, T > > >
for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{
  fn from( src : Option< Cow< 'a, T > > ) -> Self
  {
    MaybeAs::new_with_inner( src )
  }
}

impl< 'a, T, Marker > From< &'a T >
for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
{
  fn from( src : &'a T ) -> Self
  {
    MaybeAs::new_with_ref( src )
  }
}

impl< 'a, T, Marker > Default for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Default,
  Marker : Clone + Copy + 'static,
{
  fn default() -> Self
  {
    MaybeAs::new( < T as std::borrow::ToOwned >::Owned::default() )
  }
}

impl< 'a, T, Marker > fmt::Debug for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : fmt::Debug,
  Marker : Clone + Copy + 'static,
  T : fmt::Debug,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "MaybeAs" )
    .field( "0", &self.0 )
    .finish()
  }
}

impl< 'a, T, Marker > PartialEq for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
  T : PartialEq,
{
  fn eq( &self, other : &Self ) -> bool
  {
    self.as_ref() == other.as_ref()
  }
}

impl< 'a, T, Marker > Eq for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  Marker : Clone + Copy + 'static,
  T : Eq,
{
}
