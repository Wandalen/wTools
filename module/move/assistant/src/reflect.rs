
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

/// A trait for iterating over all fields convertible into a specified type within an entity.
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `E`: The element type.
pub trait Fields< 'a, K, E >
where
  E : Clone + 'a,
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( K, Option< Cow< 'a, E > > ) >;
}

// /// Return number of fields convertible into a specified type withing an entity.
// ///
// /// # Type Parameters
// ///
// /// - `E`: The element type.
// ///
// pub trait FieldsLen< E >
// {
//   /// Return number of fields convertible into a specified type withing an entity.
//   fn len( &self ) -> usize;
// }

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
