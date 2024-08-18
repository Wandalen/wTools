//!
//! Iterator over fields.
//!

/// Internal namespace.
pub( crate ) mod private
{

  // use core::fmt;
  use std::borrow::Cow;

  /// A trait for iterators that are also `ExactSizeIterator`.
  pub trait _IteratorTrait
  where
    Self : core::iter::Iterator + ExactSizeIterator + DoubleEndedIterator
  {
  }

  impl< T > _IteratorTrait for T
  where
    Self : core::iter::Iterator + ExactSizeIterator + DoubleEndedIterator
  {
  }

  /// A trait for iterators that implement `_IteratorTrait` and `Clone`.
  pub trait IteratorTrait
  where
    Self : _IteratorTrait + Clone
  {
  }

  impl< T > IteratorTrait for T
  where
    Self : _IteratorTrait + Clone
  {
  }

  ///
  /// A trait for iterating over fields convertible to a specified type within an entity.
  ///
  /// This trait provides a mechanism for accessing fields in collections or entities, converting
  /// them into a desired type for iteration.
  ///
  /// # Type Parameters
  ///
  /// - `K`: The key type, typically representing the index or identifier of each field.
  /// - `V`: The value type that fields are converted into during iteration.
  ///
  /// # Associated Types
  ///
  /// - `Val<'v>`: The type of value yielded by the iterator, parameterized by a lifetime `'v`.
  ///   This ensures the values' lifetimes are tied to the entity being iterated over.
  ///
  /// # Example
  ///
  /// ```rust
  /// use reflect_tools::{ Fields, IteratorTrait };
  ///
  /// struct MyCollection< V >
  /// {
  ///   data : Vec< V >,
  /// }
  ///
  /// impl< V > Fields< usize, &V > for MyCollection< V >
  /// {
  ///   type Key< 'k > = usize where V : 'k;
  ///   type Val< 'v > = & 'v V where Self : 'v;
  ///
  ///   fn fields( & self ) -> impl IteratorTrait< Item = ( usize, Self::Val< '_ > ) >
  ///   {
  ///     self.data.iter().enumerate()
  ///   }
  /// }
  /// ```
  ///
  /// This example shows `MyCollection` implementing `Fields`, allowing iteration over its elements
  /// with both index and value.
  pub trait Fields< K, V >
  {

    /// The type of key yielded by the iterator, parameterized by a lifetime `'k`.
    ///   This ensures the values' lifetimes are tied to the entity being iterated over.
    type Key< 'k > where Self : 'k;

    /// The type of value yielded by the iterator, parameterized by a lifetime `'v`.
    ///   This ensures the values' lifetimes are tied to the entity being iterated over.
    type Val< 'v > where Self : 'v;

    /// Returns an iterator over fields of the specified type within the entity.
    fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >;

  }

  /// Trait returning name of type of variable.
  pub trait TypeName
  {
    /// Return name of type of variable.
    fn type_name( &self ) -> &'static str;
  }

  impl< T > TypeName for T
  where
    T : ?Sized,
  {
    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      ::core::any::type_name_of_val( self )
    }
  }

  // == implementations for collections

  impl< V > Fields< usize, &'_ V > for Vec< V >
  where
    V : std::borrow::ToOwned,
  {

    type Key< 'k > = usize
    where Self : 'k, usize : 'k;

    type Val< 'v > = &'v V
    where Self : 'v, V : 'v;

    fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
    {
      self.into_iter().enumerate().map( move | ( key, val ) | ( key, val ) )
    }

  }

  impl< V > Fields< usize, Option< Cow< '_, V > > > for Vec< V >
  where
    V : std::borrow::ToOwned,
  {

    type Key< 'k > = usize
    where Self : 'k, usize : 'k;

    type Val< 'v > = Option< Cow< 'v, V > >
    where Self : 'v;

    fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
    {
      self.iter().enumerate().map( move | ( key, val ) | ( key, Some( Cow::Borrowed( val ) ) ) )
    }
  }

  impl< V, Marker > Fields< usize, crate::MaybeAs< '_, V, Marker > > for Vec< V >
  where
    V : std::borrow::ToOwned,
    Marker : Clone + Copy + 'static,
  {

    type Key< 'k > = usize
    where Self : 'k, usize : 'k;

    type Val< 'v > = crate::MaybeAs< 'v, V, Marker >
    where Self : 'v;

    fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >
    {
      self.iter().enumerate().map( move | ( key, val ) | ( key, crate::MaybeAs::from( Cow::Borrowed( val ) ) ) )
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use private::
  {
    _IteratorTrait,
    IteratorTrait,
    Fields,
    TypeName,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
