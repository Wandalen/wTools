use core::fmt;
use std::borrow::Cow;
use core::ops::{ Deref };

/// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
pub trait IntoOptionCow< 'a, T, Marker >
where
  T : Clone,
{
  /// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
  fn into_option_cow( self ) -> MaybeAs< 'a, T, Marker >;
}

impl< 'a, T, Marker > IntoOptionCow< 'a, T, Marker > for T
where
  T : Clone,
{
  #[ inline( always ) ]
  fn into_option_cow( self ) -> MaybeAs< 'a, T, Marker >
  {
    MaybeAs::< 'a, T, Marker >::new( self )
  }
}

impl< 'a, T, Marker > IntoOptionCow< 'a, T, Marker > for &'a T
where
  T : Clone,
{
  #[ inline( always ) ]
  fn into_option_cow( self ) -> MaybeAs< 'a, T, Marker >
  {
    MaybeAs::< 'a, T, Marker >::new_with_ref( self )
  }
}

// xxx
// impl< 'a, T, Marker > IntoOptionCow< 'a, T, Marker > for ()
// where
//   T : Clone,
// {
//   #[ inline( always ) ]
//   fn into_option_cow( self ) -> MaybeAs< 'a, T, Marker >
//   {
//     MaybeAs::< 'a, T, Marker >( None )
//   }
// }

/// Universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
#[ derive( Clone ) ]
pub struct MaybeAs< 'a, T, Marker >( pub Option< Cow< 'a, T > >, ::core::marker::PhantomData< fn() -> Marker > )
where
  T : Clone,
;

impl< 'a, T, Marker > MaybeAs< 'a, T, Marker >
where
  T : Clone,
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn none() -> Self
  {
    Self( None, ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new( src : T ) -> Self
  {
    Self( Some( Cow::Owned( src ) ), ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new_with_ref( src : &'a T ) -> Self
  {
    Self( Some( Cow::Borrowed( src ) ), ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> Option< Cow< 'a, T > >
  {
    self.0
  }

}

// impl< 'a, T, Marker > AsRef< T > for MaybeAs< 'a, T, Marker >
// where
//   T : Clone,
//   Self : 'a,
// {
//   fn as_ref( &self ) -> &'a T
//   {
//     match &self.0
//     {
//       Some( src ) =>
//       {
//         match src
//         {
//           Cow::Borrowed( src ) => src,
//           Cow::Owned( src ) => &src,
//         }
//       },
//       None => panic!( "MaybeAs is None" ),
//     }
//   }
// }
//
// impl< 'a, T, Marker > Deref for MaybeAs< 'a, T, Marker >
// where
//   T : Clone,
// {
//   type Target = T;
//   fn deref( &self ) -> &'a T
//   {
//     self.as_ref()
//   }
// }

impl< 'a, T, Marker > From< T >
for MaybeAs< 'a, T, Marker >
where
  T : Clone,
{
  fn from( src : T ) -> Self
  {
    MaybeAs::new( src )
  }
}

impl< 'a, T, Marker > From< &'a T >
for MaybeAs< 'a, T, Marker >
where
  T : Clone,
{
  fn from( src : &'a T ) -> Self
  {
    MaybeAs::new_with_ref( src )
  }
}

// impl< 'a, T, Marker > From< () > for MaybeAs< 'a, T, Marker >
// where
//   T : (),
// {
//   fn from( src : &'a T ) -> Self
//   {
//     MaybeAs( None )
//   }
// }

// xxx : more from

// impl< 'a, T, Marker > From< MaybeAs< 'a, T, Marker > > for &'a T
// where
//   T : Clone,
// {
//   fn from( wrapper : MaybeAs< 'a, T, Marker > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

impl< 'a, T, Marker > Default for MaybeAs< 'a, T, Marker >
where
  T : Clone,
  T : Default,
{
  fn default() -> Self
  {
    MaybeAs::new( T::default() )
  }
}

impl< 'a, T, Marker > fmt::Debug for MaybeAs< 'a, T, Marker >
where
  T : fmt::Debug,
  T : Clone,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "MaybeAs" )
    .field( "0", &self.0 )
    .finish()
  }
}
