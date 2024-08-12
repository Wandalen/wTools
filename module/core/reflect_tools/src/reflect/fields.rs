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
  /// - `Value<'v>`: The type of value yielded by the iterator, parameterized by a lifetime `'v`.
  ///   This ensures the values' lifetimes are tied to the entity being iterated over.
  ///
  /// # Example
  ///
  /// ```rust
  /// use reflect_tools::{ Fields, IteratorTrait };
  ///
  /// struct MyCollection< T >
  /// {
  ///   data : Vec< T >,
  /// }
  ///
  /// impl< T > Fields< usize, & T > for MyCollection< T >
  /// {
  ///   type Value< 'v > = & 'v T where Self : 'v;
  ///
  ///   fn fields( & self ) -> impl IteratorTrait< Item = ( usize, Self::Value< '_ > ) >
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
    /// `Value<'v>`: The type of value yielded by the iterator, parameterized by a lifetime `'v`.
    ///   This ensures the values' lifetimes are tied to the entity being iterated over.
    type Value< 'v > where Self : 'v;

    /// Returns an iterator over fields of the specified type within the entity.
    fn fields( & self ) -> impl IteratorTrait< Item = ( K, Self::Value< '_ > ) >;
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

//   impl< T, Marker > Fields< usize, Marker > for Vec< T >
//   where
//     T : std::borrow::ToOwned,
//     Marker : for< 'a > From< &'a T >,
//   //   T : Clone
//   {
//     type Value< 'v > = Marker
//     where Self : 'v;
//
//     fn fields( &self ) -> impl IteratorTrait< Item = ( usize, Self::Value< '_ > ) >
//     // where
//       // 'a : 'b,
//     {
//       self.iter().enumerate().map( move | ( key, val ) | ( key, Marker::from( val ) ) )
//     }
//   }

  impl< T > Fields< usize, &'_ T > for Vec< T >
  where
    T : std::borrow::ToOwned,
  {
    type Value< 'v > = &'v T
    where Self : 'v, T : 'v;

    fn fields( &self ) -> impl IteratorTrait< Item = ( usize, Self::Value< '_ > ) >
    {
      self.into_iter().enumerate().map( move | ( key, val ) | ( key, val ) )
    }

  }

  impl< T > Fields< usize, Option< Cow< '_, T > > > for Vec< T >
  where
    T : std::borrow::ToOwned,
  {
    type Value< 'v > = Option< Cow< 'v, T > >
    where Self : 'v;

    fn fields( &self ) -> impl IteratorTrait< Item = ( usize, Self::Value< '_ > ) >
    {
      self.iter().enumerate().map( move | ( key, val ) | ( key, Some( Cow::Borrowed( val ) ) ) )
    }
  }

  impl< T, Marker > Fields< usize, crate::MaybeAs< '_, T, Marker > > for Vec< T >
  where
    T : std::borrow::ToOwned,
    Marker : Clone + Copy + 'static,
  {
    type Value< 'v > = crate::MaybeAs< 'v, T, Marker >
    where Self : 'v;

    fn fields( &self ) -> impl IteratorTrait< Item = ( usize, Self::Value< '_ > ) >
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
