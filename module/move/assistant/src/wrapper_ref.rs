use core::fmt;
use core::ops::{ Deref };

/// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
pub trait IntoRef< 'a, T >
{
  /// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
  fn into_option_cow( self ) -> Ref< 'a, T >;
}

impl< 'a, T > IntoRef< 'a, T > for &'a T
{
  #[ inline( always ) ]
  fn into_option_cow( self ) -> Ref< 'a, T >
  {
    Ref::< 'a, T >( self )
  }
}

/// Transparent reference wrapper emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
#[ derive( Clone, Copy ) ]
pub struct Ref< 'a, T >( &'a T );

impl< 'a, T > Ref< 'a, T >
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new( src : &'a T ) -> Self
  {
    Self( src )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> &'a T
  {
    self.0
  }

}

impl< 'a, T > AsRef< T > for Ref< 'a, T >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< 'a, T > Deref for Ref< 'a, T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T > From< &'a T > for Ref< 'a, T >
{
  fn from( table : &'a T ) -> Self
  {
    Ref( table )
  }
}

// impl< 'a, T > From< Ref< 'a, T > > for &'a T
// {
//   fn from( wrapper : Ref< 'a, T > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

// impl< 'a, T > Default for Ref< 'a, T >
// where
//   T : Default,
// {
//   fn default() -> Self
//   {
//     Ref( &T::default() )
//   }
// }

impl< 'a, T > fmt::Debug for Ref< 'a, T >
where
  T : fmt::Debug,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "Ref" )
    .field( "0", &self.0 )
    .finish()
  }
}
