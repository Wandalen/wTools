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
  #[ allow( dead_code ) ]
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

/// A wrapper around a closure to be used as a `ToSuperFormer`.
///
/// This struct allows for dynamic dispatch of a closure that matches the
/// `ToSuperFormer` trait's `call` method signature. It is useful for cases where
/// a closure needs to be stored or passed around as an object implementing
/// `ToSuperFormer`.
///
/// # Type Parameters
///
/// * `T` - The type of the container being processed. This type is passed to the closure
///         when it's called.
/// * `Context` - The type of the context that may be altered or returned by the closure.
///               This allows for flexible manipulation of context based on the container.
#[ cfg( not( feature = "no_std" ) ) ]
pub struct ToSuperFormerWrapper< T, Context >
{
  closure : Box< dyn Fn( T, Option< Context > ) -> Context >,
  _marker : std::marker::PhantomData< T >,
}

#[ cfg( not( feature = "no_std" ) ) ]
impl< T, Context > ToSuperFormerWrapper< T, Context >
{
  /// Constructs a new `ToSuperFormerWrapper` with the provided closure.
  ///
  /// # Parameters
  ///
  /// * `closure` - A closure that matches the expected signature for transforming a container
  ///               and context into a new context. This closure is stored and called by the
  ///               `call` method of the `ToSuperFormer` trait implementation.
  ///
  /// # Returns
  ///
  /// Returns an instance of `ToSuperFormerWrapper` encapsulating the provided closure.
  pub fn new( closure : impl Fn( T, Option< Context > ) -> Context + 'static ) -> Self
  {
    Self
    {
      closure : Box::new( closure ),
      _marker : std::marker::PhantomData
    }
  }
}

#[ cfg( not( feature = "no_std" ) ) ]
use std::fmt;
#[ cfg( not( feature = "no_std" ) ) ]
impl< T, Context > fmt::Debug for ToSuperFormerWrapper< T, Context >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "ToSuperFormerWrapper" )
    .field( "closure", &format_args!{ "- closure -" } )
    .field( "_marker", &self._marker )
    .finish()
  }
}

#[ cfg( not( feature = "no_std" ) ) ]
impl< T, Context > ToSuperFormer< T, Context >
for ToSuperFormerWrapper< T, Context >
{
  fn call( &self, container : T, context : Option< Context > ) -> Context
  {
    ( self.closure )( container, context )
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
