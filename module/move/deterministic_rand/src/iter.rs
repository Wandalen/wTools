
//!
//! Extensions of iterator for determinism.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use std::cmp::Ordering;
  // #[ cfg( not( feature = "determinism" ) ) ]
  // use std::{ ops::Deref, ops::DerefMut };
  // #[ cfg( feature = "determinism" ) ]
  // use std::sync::{ Arc, Mutex, RwLock };
  #[ cfg( feature = "determinism" ) ]
  use std::vec::IntoIter;

  #[ cfg( feature = "determinism" ) ]
  use iter_tools::exposed::Itertools;

  // #[ cfg( feature = "determinism" ) ]
  // use rand_chacha::ChaCha8Rng;
  //
  // pub use rand::{ SeedableRng, Rng, RngCore, seq::SliceRandom };

  /// Extensions of iterator to sort items of the iterator. Replaced by a no-op when determinism is switched off.
  /// Useful, for example, to eliminate non-deterministic iteration of `HashMap` and `HashSet` keys.
  #[ sealed::sealed ]
  pub trait IfDeterminismIteratorExt : Iterator
  {
    /// Sorts the slice. Replaced by a no-op when determinism is switched off.
    /// Useful, for example, to eliminate non-deterministic iteration of `HashMap` and `HashSet` keys.
    #[ cfg( feature = "determinism" ) ]
    #[ inline( always ) ]
    fn if_determinism_then_sort( self ) -> IntoIter< Self::Item >
    where
      Self : Sized,
      Self::Item : Ord,
    {
      self.sorted()
    }

    /// Sorts the slice. Replaced by a no-op when determinism is switched off.
    /// Useful, for example, to eliminate non-deterministic iteration of `HashMap` and `HashSet` keys.
    #[ cfg( not( feature = "determinism" ) ) ]
    #[ inline( always ) ]
    fn if_determinism_then_sort( self ) -> Self
    where
      Self : Sized,
      Self::Item : Ord,
    {
      self
    }

    /// Sorts the slice with a comparator function. Replaced by a no-op when determinism is switched off.
    /// Useful, for example, to eliminate non-deterministic iteration of `HashMap` and `HashSet` keys.
    #[ cfg( feature = "determinism" ) ]
    #[ inline( always ) ]
    fn if_determinism_then_sort_by< F >( self, cmp : F ) -> IntoIter< Self::Item >
    where
      Self : Sized,
      F : FnMut( &Self::Item, &Self::Item ) -> Ordering,
    {
      self.sorted_by( cmp )
    }

    /// Sorts the slice with a comparator function. Replaced by a no-op when determinism is switched off.
    /// Useful, for example, to eliminate non-deterministic iteration of `HashMap` and `HashSet` keys.
    #[ cfg( not( feature = "determinism" ) ) ]
    #[ inline( always ) ]
    fn if_determinism_then_sort_by< F >( self, _ : F ) -> Self
    where
      Self : Sized,
      F : FnMut( &Self::Item, &Self::Item ) -> Ordering,
    {
      self
    }
  }

  #[ sealed::sealed ]
  impl< T : ?Sized > IfDeterminismIteratorExt for T
  where T : Iterator
  {
  }

}

crate::mod_interface!
{
  prelude use IfDeterminismIteratorExt;
}
