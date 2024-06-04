

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
