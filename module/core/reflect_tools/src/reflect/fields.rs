//!
//! Iterator over fields.
//!

/// Internal namespace.
mod private
{

  /// A trait for iterators that are also `ExactSizeIterator`.
  pub trait _IteratorTrait
  where
    Self : core::iter::Iterator + ExactSizeIterator
  {
  }

  impl< T > _IteratorTrait for T
  where
    Self : core::iter::Iterator + ExactSizeIterator
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
    fn fields< 's >( &'s self ) -> impl IteratorTrait< Item = ( Self::Key< 's >, Self::Val< 's > ) >;
    // fn fields( &self ) -> impl IteratorTrait< Item = ( Self::Key< '_ >, Self::Val< '_ > ) >;

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

}

mod vec;
mod hmap;
mod bmap;

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
