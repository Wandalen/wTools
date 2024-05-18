//!
//! zzz : write
//!

// zzz : investiage and reuse for iterating
// https://docs.rs/syn/latest/src/syn/punctuated.rs.html#724
// https://docs.rs/syn/latest/src/syn/drops.rs.html#11-16

/// Internal namespace.
pub mod private
{
  // use crate::*;

//   /// zzz : write documentation
//   #[ repr( transparent ) ]
//   pub struct NoDrop< T : ?Sized >( std::mem::ManuallyDrop< T > );
//
//   impl< T > NoDrop< T >
//   {
//     /// zzz : write documentation
//     pub fn new( value : T ) -> Self
//     where
//       T : TrivialDrop,
//     {
//       NoDrop( std::mem::ManuallyDrop::new( value ) )
//     }
//   }
//
//   impl< T : ?Sized > std::ops::Deref for NoDrop< T >
//   {
//     type Target = T;
//     fn deref( &self ) -> &Self::Target
//     {
//       &self.0
//     }
//   }
//
//   impl< T : ?Sized > std::ops::DerefMut for NoDrop< T >
//   {
//     fn deref_mut( &mut self ) -> &mut Self::Target
//     {
//       &mut self.0
//     }
//   }
//
//   /// zzz : write documentation
//   pub trait TrivialDrop {}
//
//   impl< T > TrivialDrop for std::iter::Empty< T > {}
//   impl< 'a, T > TrivialDrop for std::slice::Iter< 'a, T > {}
//   impl< 'a, T > TrivialDrop for std::slice::IterMut< 'a, T > {}
//   impl< 'a, T > TrivialDrop for std::option::IntoIter< &'a T > {}
//   impl< 'a, T > TrivialDrop for std::option::IntoIter< &'a mut T > {}

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
    // NoDrop,
    // TrivialDrop,
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
