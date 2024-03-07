//! # SuperFormer Trait and Implementations
//!
//! This module provides the `ToSuperFormer` trait and its implementations, enabling flexible end-of-subforming
//! processing in builder patterns. It facilitates returning the original context or container through customizable
//! handlers, making it versatile for various use cases. The `NoEnd` and `ReturnContainer` structs offer predefined
//! behaviors for common scenarios.

/// Defines a handler for the end of a subforming process, enabling the return of the original context.
///
/// This trait is designed to be flexible, allowing for various end-of-forming behaviors in builder patterns.
/// Implementors can define how to transform or pass through the context during the forming process's completion.
///
/// # Parameters
/// - `T`: The type of the container being processed.
/// - `Context`: The type of the context that might be altered or returned upon completion.
pub trait ToSuperFormer< T, Context >
{
  /// Called at the end of the subforming process to return the modified or original context.
  ///
  /// # Parameters
  /// - `container`: The container being processed.
  /// - `context`: Optional context to be transformed or returned.
  ///
  /// # Returns
  /// Returns the transformed or original context based on the implementation.
  fn call( &self, container : T, context : core::option::Option< Context > ) -> Context;
}

impl< T, Context, F > ToSuperFormer< T, Context > for F
where
  F : Fn( T, core::option::Option< Context > ) -> Context,
{
  #[ inline( always ) ]
  fn call( &self, container : T, context : core::option::Option< Context > ) -> Context
  {
    self( container, context )
  }
}

/// A `ToSuperFormer` implementation that returns the original context without any modifications.
///
/// This struct is used when no end-of-forming processing is needed, and the original context is to be returned as-is.
#[ derive( Debug, Default ) ]
pub struct NoEnd;

impl< T, Context > ToSuperFormer< T, Context >
for NoEnd
{
  #[ inline( always ) ]
  fn call( &self, _container : T, context : core::option::Option< Context > ) -> Context
  {
    context.unwrap()
  }
}

/// A `ToSuperFormer` implementation that returns the container itself instead of the context.
///
/// This struct is useful when the forming process should result in the container being returned directly,
/// bypassing any additional context processing. It simplifies scenarios where the container is the final result.
#[ derive( Debug, Default ) ]
pub struct ReturnContainer;

impl< T > ToSuperFormer< T, T >
for ReturnContainer
{
  #[ inline( always ) ]
  fn call( &self, container : T, _context : core::option::Option< T > ) -> T
  {
    container
  }
}

//
