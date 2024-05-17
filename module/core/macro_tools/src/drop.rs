//!
//! xxx : write
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  // xxx : write documentation
  #[ repr( transparent ) ]
  pub struct NoDrop< T : ?Sized >( std::mem::ManuallyDrop< T > );

  impl< T > NoDrop< T >
  {
    // xxx : write documentation
    pub( crate ) fn new( value : T ) -> Self
    where
      T : TrivialDrop,
    {
      NoDrop( std::mem::ManuallyDrop::new( value ) )
    }
  }

  impl< T : ?Sized > std::ops::Deref for NoDrop< T >
  {
    type Target = T;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< T : ?Sized > std::ops::DerefMut for NoDrop< T >
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

  // xxx : write documentation
  pub trait TrivialDrop {}

  impl< T > TrivialDrop for std::iter::Empty< T > {}
  impl< 'a, T > TrivialDrop for std::slice::Iter< 'a, T > {}
  impl< 'a, T > TrivialDrop for std::slice::IterMut< 'a, T > {}
  impl< 'a, T > TrivialDrop for std::option::IntoIter< &'a T > {}
  impl< 'a, T > TrivialDrop for std::option::IntoIter< &'a mut T > {}

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    NoDrop,
    TrivialDrop,
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as drop;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
