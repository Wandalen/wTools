//!
//! Iterators.
//!

// use std::fmt::Debug;

/// Internal namespace.
pub( crate ) mod private
{
  use std::fmt;

  // use crate::*;

  /// Trait that encapsulates an iterator with specific characteristics, tailored for use with the `syn` crate.
  ///
  /// The `IterTrait` trait is designed to represent iterators that may yield references to items (`&'a T`) within the `syn` crate.
  /// These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
  /// This combination ensures that the iterator can:
  /// - Provide an exact size hint (`ExactSizeIterator`),
  /// - Be traversed from both ends (`DoubleEndedIterator`).
  ///
  pub trait IterTrait< 'a, T >
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
  {
  }

  impl< 'a, T, I > IterTrait< 'a, T > for I
  where
    T : 'a,
    I : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
  {
  }

  /// Trait that encapsulates a clonable iterator with specific characteristics, tailored for use with the `syn` crate.
  ///
  /// The `IterTraitClonable` trait is designed to represent iterators that may yield references to items (`&'a T`) within the `syn` crate.
  /// These iterators must also implement the `ExactSizeIterator`, `DoubleEndedIterator`, and `Clone` traits.
  /// This combination ensures that the iterator can:
  /// - Provide an exact size hint (`ExactSizeIterator`),
  /// - Be traversed from both ends (`DoubleEndedIterator`),
  /// - Be clonable (`Clone`).
  ///
  pub trait IterTraitClonable< 'a, T >
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator + Clone,
  {
  }

  impl< 'a, T, I > IterTraitClonable< 'a, T > for I
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator + Clone,
  {
  }

  /// Wrapper around a boxed iterator that implements `IterTrait`.
  ///
  /// The `DynIter` struct provides a way to work with trait objects that implement the `IterTrait` trait. It acts as a
  /// wrapper around a boxed iterator and provides methods to interact with the iterator in a type-safe manner.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use crate::DynIter;
  /// use std::vec::Vec;
  ///
  /// let v = vec![1, 2, 3];
  /// let iter = DynIter::new(v.iter());
  /// for val in iter {
  ///     println!("{}", val);
  /// }
  /// ```
  pub struct DynIter< 'a, T >( Box< dyn IterTrait< 'a, & 'a T > + 'a > );

  impl< 'a, T > fmt::Debug for DynIter< 'a, T >
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "DynIter" ) )
    }
  }

  impl< 'a, T > DynIter< 'a, T >
  {
    /// Creates a new `DynIter` from an iterator that implements `IterTrait`.
    ///
    /// # Parameters
    ///
    /// - `src`: The source iterator to be wrapped.
    ///
    /// # Returns
    ///
    /// A new instance of `DynIter`.
    pub fn new< It >( src : It ) -> Self
    where
      It : IterTrait< 'a, & 'a T > + 'a,
    {
      Self( Box::new( src ) )
    }
  }

  impl< 'a, T > From< DynIter< 'a, T > > for Box< dyn IterTrait< 'a, & 'a T > + 'a >
  {
    fn from( src : DynIter< 'a, T > ) -> Self
    {
      src.0
    }
  }

  impl< 'a, T > core::ops::Deref for DynIter< 'a, T >
  {
    type Target = Box< dyn IterTrait< 'a, & 'a T > + 'a >;

    fn deref( & self ) -> & Self::Target
    {
      & self.0
    }
  }

  impl< 'a, T > core::convert::AsRef< Box< dyn IterTrait< 'a, & 'a T > + 'a > > for DynIter< 'a, T >
  {
    fn as_ref( & self ) -> & Box< dyn IterTrait< 'a, & 'a T > + 'a >
    {
      & self.0
    }
  }

  impl< 'a, T > Iterator for DynIter< 'a, T >
  {
    type Item = & 'a T;

    fn next( & mut self ) -> Option< Self::Item >
    {
      self.0.next()
    }
  }

  impl< 'a, T > ExactSizeIterator for DynIter< 'a, T >
  {
    fn len( & self ) -> usize
    {
      self.0.len()
    }
  }

  impl< 'a, T > DoubleEndedIterator for DynIter< 'a, T >
  {
    fn next_back( & mut self ) -> Option< Self::Item >
    {
      self.0.next_back()
    }
  }

//   pub trait IterTrait< 'a, T >
//   where
//     T : 'a,
//     Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator + Clone,
//   {
//     // fn clone_box( self ) -> Box< dyn IterTrait< 'a, T > + 'a >;
//   }
//
//   impl< 'a, T, I > IterTrait< 'a, T > for I
//   where
//     T : 'a,
//     I : 'a,
//     Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator + Clone,
//   {
//
//     // fn clone_box( self ) -> Box< dyn IterTrait< 'a, T > + 'a >
//     // {
//     //   Box::new( self ).clone()
//     // }
//
//   }

//   /// Trait that encapsulates an iterator with specific characteristics, tailored for use with the `syn` crate.
//   ///
//   /// The `IterTrait2` trait is designed to represent iterators that yield references to items (`&'a T`) within the `syn` crate.
//   /// These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
//   /// This combination ensures that the iterator can:
//   /// - Provide an exact size hint (`ExactSizeIterator`),
//   /// - Be traversed from both ends (`DoubleEndedIterator`).
//   ///
//   pub trait IterTrait2< T >
//   where
//     Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
//   {
//   }
//
//   impl< T, I > IterTrait2< T > for I
//   where
//     Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
//   {
//   }
//
//   /// Trait that encapsulates an iterator with specific characteristics, tailored for use with the `syn` crate.
//   ///
//   /// The `IterTrait3` trait is designed to represent iterators that yield references to items (`&'a T`) within the `syn` crate.
//   /// These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
//   /// This combination ensures that the iterator can:
//   /// - Provide an exact size hint (`ExactSizeIterator`),
//   /// - Be traversed from both ends (`DoubleEndedIterator`).
//   ///
//   pub trait IterTrait3< 'a, T : 'a >
//   where
//     Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
//   {
//   }
//
//   impl< 'a, T : 'a, I > IterTrait3< 'a, T > for I
//   where
//     Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
//   {
//   }

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
    IterTraitClonable,
    DynIter,
    // DynIterFrom,
    // IterTrait2,
    // IterTrait3,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
