//!
//! Parse structures, like `struct { a : i32 }`.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  /// xxx : write description
  /// Syn's iterator.
  pub trait IterTrait< 'a, T : 'a >
  where
    Self : Clone + Iterator< Item = &'a T > + ExactSizeIterator< Item = &'a T > + DoubleEndedIterator,
  {
  }

  impl< 'a, T : 'a, I > IterTrait< 'a, T > for I
  where
    Self : Clone + Iterator< Item = &'a T > + ExactSizeIterator< Item = &'a T > + DoubleEndedIterator,
  {
  }

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
  pub use super::protected as iter;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    IterTrait,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
