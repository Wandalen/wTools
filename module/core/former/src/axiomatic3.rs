//! ....

/// xxx
pub trait StoragePerform : ::core::default::Default
{
  type Formed;
  fn preform( self ) -> Self::Formed;
}

/// xxx
pub trait FormerDescriptor
{
  type Storage : StoragePerform< Formed = Self::Formed >;
  type Formed;
  // type Former;
}

// pub trait FormerDefinition
// {
//   type Storage : StoragePerform< Formed = Self::Formed >;
//   type Formed;
//   type Context;
//   type FormerDescriptor : FormerDescriptor< Storage = Self::Storage, Formed = Self::Formed >;
//   type End : FormingEnd< Self::FormerDescriptor, Self::Context >;
// }

/// Defines a handler for the end of a subforming process, enabling the return of the original context.
///
/// This trait is designed to be flexible, allowing for various end-of-forming behaviors in builder patterns.
/// Implementors can define how to transform or pass through the context during the forming process's completion.
///
/// # Parameters
/// - `Storage`: The type of the container being processed.
/// - `Context`: The type of the context that might be altered or returned upon completion.
pub trait FormingEnd< Former : FormerDescriptor, Context >
{
  /// Called at the end of the subforming process to return the modified or original context.
  ///
  /// # Parameters
  /// - `container`: The container being processed.
  /// - `context`: Optional context to be transformed or returned.
  ///
  /// # Returns
  /// Returns the transformed or original context based on the implementation.
  fn call( &self, storage : Former::Storage, context : core::option::Option< Context > ) -> Former::Formed;
}

impl< Former : FormerDescriptor, Context, F > FormingEnd< Former, Context > for F
where
  F : Fn( Former::Storage, core::option::Option< Context > ) -> Former::Formed,
{
  #[ inline( always ) ]
  fn call( &self, storage : Former::Storage, context : core::option::Option< Context > ) -> Former::Formed
  {
    self( storage, context )
  }
}

/// A `FormingEnd` implementation that returns the formed container itself instead of the context.
///
/// This struct is useful when the forming process should result in the formed container being returned directly,
/// bypassing any additional context processing. It simplifies scenarios where the formed container is the final result.
#[ derive( Debug, Default ) ]
pub struct ReturnStorage;

impl< Former : FormerDescriptor > FormingEnd< Former, () >
for ReturnStorage
{
  #[ inline( always ) ]
  fn call( &self, storage : Former::Storage, _context : core::option::Option< () > ) -> Former::Formed
  {
    storage.preform()
  }
}

/// A wrapper around a closure to be used as a `FormingEnd`.
///
/// This struct allows for dynamic dispatch of a closure that matches the
/// `FormingEnd` trait's `call` method signature. It is useful for cases where
/// a closure needs to be stored or passed around as an object implementing
/// `FormingEnd`.
///
/// # Type Parameters
///
/// * `Storage` - The type of the container being processed. This type is passed to the closure
///         when it's called.
/// * `Context` - The type of the context that may be altered or returned by the closure.
///               This allows for flexible manipulation of context based on the container.
#[ cfg( not( feature = "no_std" ) ) ]
pub struct FormingEndWrapper< Former : FormerDescriptor, Context >
{
  closure : Box< dyn Fn( Former::Storage, Option< Context > ) -> Former::Formed >,
  _marker : std::marker::PhantomData< Former::Storage >,
}

#[ cfg( not( feature = "no_std" ) ) ]
impl< Former : FormerDescriptor, Context > FormingEndWrapper< Former, Context >
{
  /// Constructs a new `FormingEndWrapper` with the provided closure.
  ///
  /// # Parameters
  ///
  /// * `closure` - A closure that matches the expected signature for transforming a container
  ///               and context into a new context. This closure is stored and called by the
  ///               `call` method of the `FormingEnd` trait implementation.
  ///
  /// # Returns
  ///
  /// Returns an instance of `FormingEndWrapper` encapsulating the provided closure.
  pub fn new( closure : impl Fn( Former::Storage, Option< Context > ) -> Former::Formed + 'static ) -> Self
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
impl< Former : FormerDescriptor, Context > fmt::Debug for FormingEndWrapper< Former, Context >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "FormingEndWrapper" )
    .field( "closure", &format_args!{ "- closure -" } )
    .field( "_marker", &self._marker )
    .finish()
  }
}

#[ cfg( not( feature = "no_std" ) ) ]
impl< Former : FormerDescriptor, Context > FormingEnd< Former, Context >
for FormingEndWrapper< Former, Context >
{
  fn call( &self, storage : Former::Storage, context : Option< Context > ) -> Former::Formed
  {
    ( self.closure )( storage, context )
  }
}

//

/// A trait for initiating a structured subforming process with contextual and intermediary storage linkage.
///
/// This trait facilitates the creation of a subformer that carries through a builder pattern chain,
/// utilizing intermediary storage for accumulating state or data before finally transforming it into
/// a `Formed` structure. It is designed for scenarios where a multi-step construction or transformation
/// process benefits from maintaining both transient state (`Storage`) and contextual information (`Context`),
/// before concluding with the generation of a final product (`Formed`).
///
/// # Type Parameters
///
/// * `Storage` - Represents a mutable intermediary storage structure used throughout the subforming process
///               to accumulate data, state, or partial computations. This storage is internal to the
///               subformer and is eventually converted into the final `Formed` structure by the subformer,
///               not directly by implementations of this trait.
///
/// * `Formed` - Denotes the final type that results from the subforming process. This is the intended outcome
///              of the builder chain, constructed or transformed from the `Storage` with consideration of
///              the provided `Context`.
///
/// * `Context` - Specifies the contextual backdrop against which the subforming process unfolds. This could
///               encompass references to parent builders, configuration data, or any state influencing how
///               `Storage` transitions into `Formed`.
///
/// # Functions
///
/// * `_begin` - This function launches the subforming process, marking the start of a construction or transformation
///              sequence defined by the implementing type. It establishes the foundational `Storage` and `Context`,
///              alongside specifying an `on_end` completion handler that dictates the final conversion into `Formed`.
///
/// The `FormerBegin` trait, by decoupling `Storage` from `Formed` and introducing a contextual layer, enables
/// sophisticated and flexible construction patterns conducive to complex data transformations or object creation
/// sequences within builder patterns.

// xxx : change sequence
pub trait FormerBegin< Former : FormerDescriptor, Context >
{

  /// * `End` - A trait bound marking the closure or handler invoked upon completing the subforming process. Implementers
  ///           of this trait (`End`) are tasked with applying the final transformations that transition `Storage`
  ///           into `Formed`, optionally utilizing `Context` to guide this transformation. It is crucial that this
  ///           associated type satisfies the `FormingEnd<Formed, Context>` trait, defining the precise mechanics of
  ///           how the subformer concludes its operation.
  type End : FormingEnd< Former, Context >;

  /// Launches the subforming process with an initial storage and context, setting up an `on_end` completion handler.
  ///
  /// # Parameters
  ///
  /// * `storage` - An optional initial state for the intermediary storage structure.
  /// * `context` - An optional initial setting providing contextual information for the subforming process.
  /// * `on_end` - A completion handler responsible for transforming the accumulated `Storage` into the final `Formed` structure.
  fn _begin
  (
    storage : core::option::Option< Former::Storage >,
    context : core::option::Option< Context >,
    on_end : Self::End,
  ) -> Self;

}
