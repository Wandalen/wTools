use core::fmt;
use core::ops::{ Deref };

/// Transparent wrapper to emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
#[ derive( Clone, Copy ) ]
pub struct As< 'a, T >( &'a T );

impl< 'a, T > As< 'a, T >
{
  /// Just a constructor.
  pub fn new( src : &'a T ) -> Self
  {
    Self( src )
  }
  /// Just a constructor.
  pub fn inner( self ) -> &'a T
  {
    self.0
  }

}

impl< 'a, T > AsRef< T > for As< 'a, T >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< 'a, T > Deref for As< 'a, T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T > From< &'a T > for As< 'a, T >
{
  fn from( table : &'a T ) -> Self
  {
    As( table )
  }
}

// impl< 'a, T > From< As< 'a, T > > for &'a T
// {
//   fn from( wrapper : As< 'a, T > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

// impl< 'a, T > Default for As< 'a, T >
// where
//   T : Default,
// {
//   fn default() -> Self
//   {
//     As( &T::default() )
//   }
// }

impl< 'a, T > fmt::Debug for As< 'a, T >
where
  T : fmt::Debug,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "As" )
    .field( "0", &self.0 )
    .finish()
  }
}
