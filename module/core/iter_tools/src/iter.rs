
#[ cfg( not( feature = "no_std" ) ) ]
pub( crate ) mod private
{
  use ::itertools::process_results;

  use clone_dyn_types::CloneDyn;

  /// Trait that encapsulates an iterator with specific characteristics and implemetning `CloneDyn`.
  ///
  /// The `_IterTrait` trait is designed to represent iterators that may yield references to items ( `&'a T` ).
  /// These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
  /// This combination ensures that the iterator can:
  /// - Provide an exact size hint ( `ExactSizeIterator` ),
  /// - Be traversed from both ends ( `DoubleEndedIterator` ).
  ///
  /// Additionally, the iterator must implement the `CloneDyn` trait, which allows cloning of trait objects.
  ///
  /// # Example
  /// ```rust
  /// use iter_tools::_IterTrait;
  ///
  /// // Example struct that implements Iterator, ExactSizeIterator, DoubleEndedIterator, and CloneDyn.
  /// #[ derive( Clone ) ]
  /// struct MyIterator
  /// {
  ///   // internal fields
  /// }
  ///
  /// impl Iterator for MyIterator
  /// {
  ///   type Item = i32;
  ///
  ///   fn next( &mut self ) -> Option< Self::Item >
  ///   {
  ///     // implementation
  ///     Some( 1 )
  ///   }
  /// }
  ///
  /// impl ExactSizeIterator for MyIterator
  /// {
  ///   fn len( &self ) -> usize
  ///   {
  ///     // implementation
  ///     1
  ///   }
  /// }
  ///
  /// impl DoubleEndedIterator for MyIterator
  /// {
  ///   fn next_back( &mut self ) -> Option< Self::Item >
  ///   {
  ///     // implementation
  ///     Some( 1 )
  ///   }
  /// }
  ///
  /// ```

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

  /// Trait that encapsulates a clonable iterator with specific characteristics, tailored for use with the `syn` crate.
  ///
  /// The `IterTrait` trait is designed to represent iterators that may yield references to items ( `&'a T` ) within the `syn` crate.
  /// These iterators must also implement the `ExactSizeIterator`, `DoubleEndedIterator`, and `Clone` traits.
  /// This combination ensures that the iterator can:
  /// - Provide an exact size hint ( `ExactSizeIterator` ),
  /// - Be traversed from both ends ( `DoubleEndedIterator` ),
  /// - Be clonable ( `Clone` ).
  ///
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
  ///
  /// This allows cloning of boxed iterators that implement `_IterTrait`.
  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      clone_dyn_types::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + Send + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      clone_dyn_types::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + Sync + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      clone_dyn_types::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn _IterTrait< 'c, T > + Send + Sync + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      clone_dyn_types::clone_into_box( &**self )
    }
  }

  /// Type alias for boxed `_IterTrait` trait objects.
  ///
  /// Prefer `BoxedIter` over `impl _IterTrait` when using trait objects ( `dyn _IterTrait` ) because the concrete type in return is less restrictive than `impl _IterTrait`.
  ///
  pub type BoxedIter< 'a, T > = Box< dyn _IterTrait< 'a, T > + 'a >;

  /// Extension of iterator.

  // xxx : review
  pub trait IterExt
  where
    Self : core::iter::Iterator,
  {
    /// Iterate each element and return `core::Result::Err` if any element is error.
    fn map_result< F, RE, El >( self, f : F ) -> core::result::Result< Vec< El >, RE >
    where
      Self : Sized + Clone,
      F : FnMut( < Self as core::iter::Iterator >::Item ) -> core::result::Result< El, RE >,
      RE : core::fmt::Debug,
    ;
  }

  impl< Iterator > IterExt for Iterator
  where
    Iterator : core::iter::Iterator,
  {
    fn map_result< F, RE, El >( self, f : F ) -> core::result::Result< Vec< El >, RE >
    where
      Self : Sized + Clone,
      F : FnMut( < Self as core::iter::Iterator >::Item ) -> core::result::Result< El, RE >,
      RE : core::fmt::Debug,
    {
      let vars_maybe = self.map( f );
      let vars : Vec< _ > = process_results( vars_maybe, | iter | iter.collect() )?;
      Ok( vars )
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;

}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;


  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::itertools::
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

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use std::iter::zip;

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    _IterTrait,
    IterTrait,
    BoxedIter,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::itertools::
  {
    Diff,
    Either,
    EitherOrBoth,
    FoldWhile,
    MinMaxResult,
    Position,
    Itertools,
    PeekingNext,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  pub use super::private::IterExt;

}
