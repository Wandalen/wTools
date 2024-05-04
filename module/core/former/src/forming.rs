

/// Provides a mechanism for mutating the context and storage just before the forming process is completed.
///
/// The `FormerMutator` trait allows for the implementation of custom mutation logic on the internal state
/// of an entity (context and storage) just before the final forming operation is completed. This mutation
/// occurs immediately before the `FormingEnd` callback is invoked.
///
/// ## Differences from `FormingEnd`
///
/// Unlike `FormingEnd`, which is responsible for integrating and finalizing the formation process of a field within
/// a parent former, `form_mutation` directly pertains to the entity itself. This method is designed to be independent
/// of whether the forming process is occurring within the context of a superformer or if the structure is a standalone
/// or nested field. This makes `form_mutation` suitable for entity-specific transformations that should not interfere
/// with the hierarchical forming logic managed by `FormingEnd`.
///
/// ## Use Cases
///
/// - Applying last-minute changes to the data being formed.
/// - Setting or modifying properties that depend on the final state of the storage or context.
/// - Storage-specific fields which are not present in formed structure.

// xxx : add example
pub trait FormerMutator
where
  Self : crate::FormerDefinitionTypes,
{
  /// Mutates the context and storage of the entity just before the formation process completes.
  ///
  /// This function is invoked immediately prior to the `FormingEnd` callback during the forming process.
  /// It provides a hook for implementing custom logic that modifies the internal state (storage and context)
  /// of the entity. `form_mutation` is particularly useful for adjustments or updates that need to reflect
  /// in the entity just before it is finalized and returned.
  ///
  #[ inline ]
  fn form_mutation( _storage : &mut Self::Storage, _context : &mut ::core::option::Option< Self::Context > )
  {
  }
}

// impl< Definition > crate::FormerMutator
// for Definition
// where
//   Definition : crate::FormerDefinitionTypes,
// {
// }

/// Defines a handler for the end of a subforming process, enabling the return of the original context.
///
/// This trait is designed to be flexible, allowing for various end-of-forming behaviors in builder patterns.
/// Implementors can define how to transform or pass through the context during the forming process's completion.
///
/// # Parameters
/// - `Storage`: The type of the container being processed.
/// - `Context`: The type of the context that might be altered or returned upon completion.

pub trait FormingEnd< Definition : crate::FormerDefinitionTypes >
{
  /// Called at the end of the subforming process to return the modified or original context.
  ///
  /// # Parameters
  /// - `container`: The container being processed.
  /// - `context`: Optional context to be transformed or returned.
  ///
  /// # Returns
  /// Returns the transformed or original context based on the implementation.
  fn call( &self, storage : Definition::Storage, context : core::option::Option< Definition::Context > ) -> Definition::Formed;
}

impl< Definition, F > FormingEnd< Definition > for F
where
  F : Fn( Definition::Storage, core::option::Option< Definition::Context > ) -> Definition::Formed,
  Definition : crate::FormerDefinitionTypes,
{
  #[ inline( always ) ]
  fn call( &self, storage : Definition::Storage, context : core::option::Option< Definition::Context > ) -> Definition::Formed
  {
    self( storage, context )
  }
}

/// A `FormingEnd` implementation that returns the formed container itself instead of the context.
///
/// This struct is useful when the forming process should result in the formed container being returned directly,
/// bypassing any additional context processing. It simplifies scenarios where the formed container is the final result.
#[ derive( Debug, Default ) ]
pub struct ReturnPreformed;

impl< Definition > FormingEnd< Definition >
for ReturnPreformed
where
  Definition::Storage : crate::StoragePreform< Preformed = Definition::Formed >,
  Definition : crate::FormerDefinitionTypes,
{
  #[ inline( always ) ]
  fn call( &self, storage : Definition::Storage, _context : core::option::Option< Definition::Context > ) -> Definition::Formed
  {
    crate::StoragePreform::preform( storage )
  }
}

/// zzz : update description
#[ derive( Debug, Default ) ]
pub struct ReturnStorage;

impl< Definition, T > FormingEnd< Definition >
for ReturnStorage
where
  Definition : crate::FormerDefinitionTypes< Context = (), Storage = T, Formed = T >,
{
  #[ inline( always ) ]
  fn call( &self, storage : Definition::Storage, _context : core::option::Option< () > ) -> Definition::Formed
  {
    storage
  }
}

// zzz : improve description
/// Use `NoEnd` to fill parameter FormingEnd in struct where parameter exists, but it is not needed.
/// It might be needed if the same struct is used as `FormerDefinitionTypes` and as `FormerDefinition`, because the first one does not have information aboud End function.
/// Similar logic which `std::marker::PhantomData` has behind.
#[ derive( Debug, Default ) ]
pub struct NoEnd;

impl< Definition > FormingEnd< Definition >
for NoEnd
where
  Definition : crate::FormerDefinitionTypes,
{
  #[ inline( always ) ]
  fn call( &self, _storage : Definition::Storage, _context : core::option::Option< Definition::Context > ) -> Definition::Formed
  {
    unreachable!();
  }
}

/// A wrapper around a closure to be used as a `FormingEnd`.
///
/// This struct allows for dynamic dispatch of a closure that matches the
/// `FormingEnd` trait's `call` method signature. It is useful for cases where
/// a closure needs to be stored or passed around as an object implementing
/// `FormingEnd`.
#[ cfg( not( feature = "no_std" ) ) ]
pub struct FormingEndClosure< Definition : crate::FormerDefinitionTypes >
{
  closure : Box< dyn Fn( Definition::Storage, Option< Definition::Context > ) -> Definition::Formed >,
  _marker : std::marker::PhantomData< Definition::Storage >,
}

impl< T, Definition > From< T > for FormingEndClosure< Definition >
where
  T : Fn( Definition::Storage, Option< Definition::Context > ) -> Definition::Formed + 'static,
  Definition : crate::FormerDefinitionTypes,
{
  #[ inline( always ) ]
  fn from( closure : T ) -> Self
  {
    Self
    {
      closure : Box::new( closure ),
      _marker : std::marker::PhantomData
    }
  }
}

#[ cfg( not( feature = "no_std" ) ) ]
impl< Definition : crate::FormerDefinitionTypes > FormingEndClosure< Definition >
{
  /// Constructs a new `FormingEndClosure` with the provided closure.
  ///
  /// # Parameters
  ///
  /// * `closure` - A closure that matches the expected signature for transforming a container
  ///               and context into a new context. This closure is stored and called by the
  ///               `call` method of the `FormingEnd` trait implementation.
  ///
  /// # Returns
  ///
  /// Returns an instance of `FormingEndClosure` encapsulating the provided closure.
  pub fn new( closure : impl Fn( Definition::Storage, Option< Definition::Context > ) -> Definition::Formed + 'static ) -> Self
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
impl< Definition : crate::FormerDefinitionTypes > fmt::Debug for FormingEndClosure< Definition >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "FormingEndClosure" )
    .field( "closure", &format_args!{ "- closure -" } )
    .field( "_marker", &self._marker )
    .finish()
  }
}

#[ cfg( not( feature = "no_std" ) ) ]
impl< Definition : crate::FormerDefinitionTypes > FormingEnd< Definition >
for FormingEndClosure< Definition >
{
  fn call( &self, storage : Definition::Storage, context : Option< Definition::Context > ) -> Definition::Formed
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
/// The `FormerBegin` trait, by decoupling `Storage` from `Formed` and introducing a contextual layer, enables
/// sophisticated and flexible construction patterns conducive to complex data transformations or object creation
/// sequences within builder patterns.

// zzz : update description
pub trait FormerBegin< Definition : crate::FormerDefinition >
{

  /// Launches the subforming process with an initial storage and context, setting up an `on_end` completion handler.
  ///
  /// # Parameters
  ///
  /// * `storage` - An optional initial state for the intermediary storage structure.
  /// * `context` - An optional initial setting providing contextual information for the subforming process.
  /// * `on_end` - A completion handler responsible for transforming the accumulated `Storage` into the final `Formed` structure.
  fn former_begin
  (
    storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : Definition::End,
  ) -> Self;

}
