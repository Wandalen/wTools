use core::fmt;
use std::borrow::Cow;
use core::ops::{ Deref };

/// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
pub trait IntoOptionCow< 'a, T >
where
  T : Clone,
{
  /// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
  fn into_option_cow( self ) -> MaybeAs< 'a, T >;
}

impl< 'a, T > IntoOptionCow< 'a, T > for T
where
  T : Clone,
{
  #[ inline( always ) ]
  fn into_option_cow( self ) -> MaybeAs< 'a, T >
  {
    MaybeAs::< 'a, T >( Some( Cow::Owned( self ) ) )
  }
}

impl< 'a, T > IntoOptionCow< 'a, T > for &'a T
where
  T : Clone,
{
  #[ inline( always ) ]
  fn into_option_cow( self ) -> MaybeAs< 'a, T >
  {
    MaybeAs::< 'a, T >( Some( Cow::Borrowed( self ) ) )
  }
}

// xxx
// impl< 'a, T > IntoOptionCow< 'a, T > for ()
// where
//   T : Clone,
// {
//   #[ inline( always ) ]
//   fn into_option_cow( self ) -> MaybeAs< 'a, T >
//   {
//     MaybeAs::< 'a, T >( None )
//   }
// }

/// Universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
#[ derive( Clone ) ]
pub struct MaybeAs< 'a, T >( pub Option< Cow< 'a, T > > )
where
  T : Clone,
;

impl< 'a, T > MaybeAs< 'a, T >
where
  T : Clone,
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new( src : T ) -> Self
  {
    Self( Some( Cow::Owned( src ) ) )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new_with_ref( src : &'a T ) -> Self
  {
    Self( Some( Cow::Borrowed( src ) ) )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> Option< Cow< 'a, T > >
  {
    self.0
  }

}

// impl< 'a, T > AsRef< T > for MaybeAs< 'a, T >
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
// impl< 'a, T > Deref for MaybeAs< 'a, T >
// where
//   T : Clone,
// {
//   type Target = T;
//   fn deref( &self ) -> &'a T
//   {
//     self.as_ref()
//   }
// }

impl< 'a, T > From< T > for MaybeAs< 'a, T >
where
  T : Clone,
{
  fn from( table : T ) -> Self
  {
    MaybeAs( Some( Cow::Owned( table ) ) )
  }
}

impl< 'a, T > From< &'a T > for MaybeAs< 'a, T >
where
  T : Clone,
{
  fn from( table : &'a T ) -> Self
  {
    MaybeAs( Some( Cow::Borrowed( table ) ) )
  }
}

// impl< 'a, T > From< () > for MaybeAs< 'a, T >
// where
//   T : (),
// {
//   fn from( table : &'a T ) -> Self
//   {
//     MaybeAs( None )
//   }
// }

// xxx : more from

// impl< 'a, T > From< MaybeAs< 'a, T > > for &'a T
// where
//   T : Clone,
// {
//   fn from( wrapper : MaybeAs< 'a, T > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

impl< 'a, T > Default for MaybeAs< 'a, T >
where
  T : Clone,
  T : Default,
{
  fn default() -> Self
  {
    MaybeAs( Some( Cow::Owned( T::default() ) ) )
  }
}

impl< 'a, T > fmt::Debug for MaybeAs< 'a, T >
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
