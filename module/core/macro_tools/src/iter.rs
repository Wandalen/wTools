//!
//! Iterators.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use clone_dyn_types::CloneDyn;

  /// Trait that encapsulates an iterator with specific characteristics, tailored for use with the `syn` crate and implemetning `CloneDyn`.
  ///
  /// The `_IterTrait` trait is designed to represent iterators that may yield references to items ( `&'a T` ) within the `syn` crate.
  /// These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
  /// This combination ensures that the iterator can:
  /// - Provide an exact size hint ( `ExactSizeIterator` ),
  /// - Be traversed from both ends ( `DoubleEndedIterator` ).
  ///
  /// Additionally, the iterator must implement the `CloneDyn` trait, which allows cloning of trait objects.
  ///
  /// # Example
  /// ```rust
  /// use assistant::_IterTrait;
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
  /// impl _IterTrait< '_, i32 > for MyIterator {}
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
  /// # Example
  /// ```rust
  /// use assistant::{ _IterTrait, BoxedIter };
  ///
  /// // Example function that returns a BoxedIter.
  /// fn example_iterator() -> BoxedIter< 'static, i32 >
  /// {
  ///   Box::new( MyIterator
  ///   {
  ///     // initialize fields
  ///   })
  /// }
  /// ```
  pub type BoxedIter< 'a, T > = Box< dyn _IterTrait< 'a, T > + 'a >;

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

// xxx : qqq : make command to autogenerate it
//   /// Wrapper around a boxed iterator that implements `_IterTrait`.
//   ///
//   /// The `DynIter` struct provides a way to work with trait objects that implement the `_IterTrait` trait. It acts as a
//   /// wrapper around a boxed iterator and provides methods to interact with the iterator in a type-safe manner.
//   ///
//   /// # Examples
//   ///
//   /// ```rust
//   /// use crate::DynIter;
//   /// use std::vec::Vec;
//   ///
//   /// let v = vec![ 1, 2, 3 ];
//   /// let iter = DynIter::new( v.iter() );
//   /// for val in iter
//   /// {
//   ///   println!( "{}", val );
//   /// }
//   /// ```
//   pub struct DynIter< 'a, T >( Box< dyn _IterTrait< 'a, & 'a T > + 'a > );
//
//   impl< 'a, T > fmt::Debug for DynIter< 'a, T >
//   {
//     fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
//     {
//       f.write_fmt( format_args!( "DynIter" ) )
//     }
//   }
//
//   impl< 'a, T > DynIter< 'a, T >
//   {
//     /// Creates a new `DynIter` from an iterator that implements `_IterTrait`.
//     ///
//     /// # Parameters
//     ///
//     /// - `src`: The source iterator to be wrapped.
//     ///
//     /// # Returns
//     ///
//     /// A new instance of `DynIter`.
//     pub fn new< It >( src : It ) -> Self
//     where
//       It : _IterTrait< 'a, & 'a T > + 'a,
//     {
//       Self( Box::new( src ) )
//     }
//   }
//
//   impl< 'a, T > From< DynIter< 'a, T > > for Box< dyn _IterTrait< 'a, & 'a T > + 'a >
//   {
//     fn from( src : DynIter< 'a, T > ) -> Self
//     {
//       src.0
//     }
//   }
//
//   impl< 'a, T > core::ops::Deref for DynIter< 'a, T >
//   {
//     type Target = Box< dyn _IterTrait< 'a, & 'a T > + 'a >;
//
//     fn deref( & self ) -> & Self::Target
//     {
//       & self.0
//     }
//   }
//
//   impl< 'a, T > core::convert::AsRef< Box< dyn _IterTrait< 'a, & 'a T > + 'a > > for DynIter< 'a, T >
//   {
//     fn as_ref( & self ) -> & Box< dyn _IterTrait< 'a, & 'a T > + 'a >
//     {
//       & self.0
//     }
//   }
//
//   impl< 'a, T > Iterator for DynIter< 'a, T >
//   {
//     type Item = & 'a T;
//
//     fn next( & mut self ) -> Option< Self::Item >
//     {
//       self.0.next()
//     }
//   }
//
//   impl< 'a, T > ExactSizeIterator for DynIter< 'a, T >
//   {
//     fn len( & self ) -> usize
//     {
//       self.0.len()
//     }
//   }
//
//   impl< 'a, T > DoubleEndedIterator for DynIter< 'a, T >
//   {
//     fn next_back( & mut self ) -> Option< Self::Item >
//     {
//       self.0.next_back()
//     }
//   }

  // =

//   trait Cloneable : Clone
//   {
//     fn clone_box( & self ) -> Box< dyn Cloneable >;
//   }
//
//   impl< T > Cloneable for T
//   where
//     T : 'static + Clone,
//   {
//     fn clone_box( & self ) -> Box< dyn Cloneable >
//     {
//       Box::new( self.clone() )
//     }
//   }
//
//   pub fn clone_boxed( t : & dyn Cloneable ) -> Box< dyn Cloneable >
//   {
//     t.clone_box()
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
    _IterTrait,
    IterTrait,
    BoxedIter,
    // DynIter,
    // DynIterFrom,
    // IterTrait2,
    // IterTrait3,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
