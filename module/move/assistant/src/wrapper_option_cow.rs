use core::fmt;
use core::ops::{ Deref };

/// Transparent reference wrapper to emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
#[ derive( Clone, Copy ) ]
pub struct AsOptionCow< 'a, T >( &'a T );

impl< 'a, T > AsOptionCow< 'a, T >
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

impl< 'a, T > AsRef< T > for AsOptionCow< 'a, T >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< 'a, T > Deref for AsOptionCow< 'a, T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T > From< &'a T > for AsOptionCow< 'a, T >
{
  fn from( table : &'a T ) -> Self
  {
    AsOptionCow( table )
  }
}

// impl< 'a, T > From< AsOptionCow< 'a, T > > for &'a T
// {
//   fn from( wrapper : AsOptionCow< 'a, T > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

// impl< 'a, T > Default for AsOptionCow< 'a, T >
// where
//   T : Default,
// {
//   fn default() -> Self
//   {
//     AsOptionCow( &T::default() )
//   }
// }

impl< 'a, T > fmt::Debug for AsOptionCow< 'a, T >
where
  T : fmt::Debug,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "AsOptionCow" )
    .field( "0", &self.0 )
    .finish()
  }
}
