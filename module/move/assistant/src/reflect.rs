

// use core::fmt;

/// A trait for iterating over all fields of a specified type within an entity.
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `E`: The element type.
pub trait Fields< K, E >
{
  /// Returns an iterator over all fields of the specified type within the entity.
  fn fields( &self ) -> impl Iterator< Item = ( K, E ) > + Clone;
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
