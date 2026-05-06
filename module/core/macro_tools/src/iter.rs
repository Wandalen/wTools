//!
//! Tailored iterator.
//!

/// Define a private namespace for all its items.
mod private
{
  use ::clone_dyn_types ::CloneDyn;

  /// Trait that encapsulates an iterator with specific characteristics implementing `CloneDyn`.
  ///
  /// Represents iterators that yield references to items `&'a T`, also implementing
  /// `ExactSizeIterator`, `DoubleEndedIterator`, and `CloneDyn`.
  pub trait _IterTrait< 'a, T >
  where
  T : 'a,
  Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
  Self : CloneDyn,
  {
  }

  impl< 'a, T, I > _IterTrait< 'a, T > for I
  where
  T : 'a,
  Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
  Self : CloneDyn,
  {
  }

  /// Trait that encapsulates a clonable iterator with specific characteristics.
  pub trait IterTrait< 'a, T >
  where
  T : 'a,
  Self : _IterTrait< 'a, T > + Clone,
  {
  }

  impl< 'a, T, I > IterTrait< 'a, T > for I
  where
  T : 'a,
  Self : _IterTrait< 'a, T > + Clone,
  {
  }

  /// Implement `Clone` for boxed `_IterTrait` trait objects.
  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      ::clone_dyn_types ::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + Send + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      ::clone_dyn_types ::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + Sync + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      ::clone_dyn_types ::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + Send + Sync + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      ::clone_dyn_types ::clone_into_box( &**self )
    }
  }

  /// Type alias for boxed `_IterTrait` trait objects.
  ///
  /// Prefer `BoxedIter` over `impl _IterTrait` when using trait objects (`dyn _IterTrait`)
  /// because the concrete type in return is less restrictive than `impl _IterTrait`.
  pub type BoxedIter< 'a, T > = Box< dyn _IterTrait< 'a, T > + 'a >;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super :: *;
  #[ doc( inline ) ]
  pub use orphan :: *;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super :: *;

  #[ doc( inline ) ]
  pub use exposed :: *;

  #[ doc( inline ) ]
  pub use ::itertools ::
  {
    all,
    any,
    assert_equal,
    chain,
    cloned,
    concat,
    cons_tuples,
    diff_with,
    enumerate,
    equal,
    fold,
    interleave,
    intersperse,
    intersperse_with,
    iterate,
    join,
    kmerge,
    kmerge_by,
    max,
    merge,
    merge_join_by,
    min,
    multipeek,
    multiunzip,
    multizip,
    partition,
    peek_nth,
    process_results,
    put_back,
    put_back_n,
    rciter,
    repeat_n,
    rev,
    sorted,
    unfold,
    // zip,
    zip_eq,
    Itertools,
  };

  #[ doc( inline ) ]
  pub use core ::iter ::zip;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super :: *;

  #[ doc( inline ) ]
  pub use prelude :: *;

  #[ doc( inline ) ]
  pub use private :: { _IterTrait, IterTrait };

  #[ doc( inline ) ]
  pub use private ::BoxedIter;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super :: *;

  #[ doc( inline ) ]
  pub use ::itertools :: { Diff, Either, EitherOrBoth, FoldWhile, MinMaxResult, Position, Itertools, PeekingNext };
}
