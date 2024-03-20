//! ....

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

/// A trait defining the initialization process for a subformer with contextual linkage.
///
/// This trait is designed for types that need to initiate a subforming process,
/// passing themselves as the context and specifying a closure or handler (`on_end`) to be
/// called upon completion. It facilitates the construction of builder pattern chains
/// that maintain stateful context through each step of the process.
///
/// # Type Parameters
///
/// * `Struct` - Represents the type that is being constructed or transformed by the subformer.
/// * `Context` - Denotes the contextual information or the environment in which `Struct` is being formed.
///               This could be a reference to a parent builder, configuration settings, or any other
///               relevant state.
///
/// # Associated Types
///
/// * `End` - Specifies the trait bound for the closure or handler that gets called at the completion
///           of the subforming process. This type must implement the `ToSuperFormer<Struct, Context>`
///           trait, which defines how the final transformation or construction of `Struct` is handled,
///           potentially using the provided `Context`.
///

pub trait FormerBegin< Struct, Context >
{

  /// * `End` - Specifies the trait bound for the closure or handler that gets called at the completion
  ///           of the subforming process. This type must implement the `ToSuperFormer<Struct, Context>`
  ///           trait, which defines how the final transformation or construction of `Struct` is handled,
  ///           potentially using the provided `Context`.
  type End : ToSuperFormer< Struct, Context >;

  /// Initializes the subforming process by setting the context and specifying an `on_end` completion handler.
  ///
  /// This function is the entry point for initiating a subforming sequence, allowing the caller
  /// to establish initial contextual information and define how the process concludes.
  ///
  /// # Parameters
  ///
  /// * `context` - An optional parameter providing initial context for the subforming process. This
  ///               might include configuration data, references to parent structures, or any state
  ///               relevant to the formation of `Struct`.
  ///
  /// * `on_end` - A closure or handler of type `Self::End` that is invoked at the completion of
  ///              the subforming process. This handler is responsible for applying any final transformations
  ///              to `Struct` and potentially utilizing `Context` to influence the outcome.
  ///
  fn _begin
  (
    context : core::option::Option< Context >,
    on_end : Self::End,
  ) -> Self;

}
