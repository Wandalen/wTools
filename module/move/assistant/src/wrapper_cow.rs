use core::fmt;
use core::ops::{ Deref };

/// Transparent reference wrapper to emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
#[ derive( Clone, Copy ) ]
pub struct AsCow< 'a, T >( &'a T );

impl< 'a, T > AsCow< 'a, T >
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

impl< 'a, T > AsRef< T > for AsCow< 'a, T >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< 'a, T > Deref for AsCow< 'a, T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T > From< &'a T > for AsCow< 'a, T >
{
  fn from( table : &'a T ) -> Self
  {
    AsCow( table )
  }
}

// impl< 'a, T > From< AsCow< 'a, T > > for &'a T
// {
//   fn from( wrapper : AsCow< 'a, T > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

// impl< 'a, T > Default for AsCow< 'a, T >
// where
//   T : Default,
// {
//   fn default() -> Self
//   {
//     AsCow( &T::default() )
//   }
// }

impl< 'a, T > fmt::Debug for AsCow< 'a, T >
where
  T : fmt::Debug,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "AsCow" )
    .field( "0", &self.0 )
    .finish()
  }
}
