//!
//! Iterators.
//!

/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;

  /// Trait that encapsulates an iterator with specific characteristics, tailored for use with the `syn` crate.
  ///
  /// The `IterTrait` trait is designed to represent iterators that yield references to items (`&'a T`) within the `syn` crate.
  /// These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
  /// This combination ensures that the iterator can:
  /// - Provide an exact size hint (`ExactSizeIterator`),
  /// - Be traversed from both ends (`DoubleEndedIterator`).
  ///
  pub trait IterTrait< 'a, T : 'a >
  where
    Self : Iterator< Item = &'a T > + ExactSizeIterator< Item = &'a T > + DoubleEndedIterator,
  {
  }

  impl< 'a, T : 'a, I > IterTrait< 'a, T > for I
  where
    Self : Iterator< Item = &'a T > + ExactSizeIterator< Item = &'a T > + DoubleEndedIterator,
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