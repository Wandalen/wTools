//! Module `forming`
//!
//! This module defines a collection of traits that are crucial for implementing a structured and extensible builder pattern.
//! The traits provided manage the various stages of the forming process, handling the initiation, mutation, and completion
//! of constructing complex data structures. These traits facilitate the creation of flexible and maintainable formation
//! logic that can accommodate complex construction scenarios, including nested and conditional formations.

/// Provides a mechanism for mutating the context and storage just before the forming process is completed.
///
/// The `FormerMutator` trait allows for the implementation of custom mutation logic on the internal state
/// of an entity (context and storage) just before the final forming operation is completed. This mutation
/// occurs immediately before the `FormingEnd` callback is invoked.
///
/// #### Differences from `FormingEnd`
///
/// Unlike `FormingEnd`, which is responsible for integrating and finalizing the formation process of a field within
/// a parent former, `form_mutation` directly pertains to the entity itself. This method is designed to be independent
/// of whether the forming process is occurring within the context of a superformer or if the structure is a standalone
/// or nested field. This makes `form_mutation` suitable for entity-specific transformations that should not interfere
/// with the hierarchical forming logic managed by `FormingEnd`.
///
/// #### Use Cases
///
/// - Applying last-minute changes to the data being formed.
/// - Setting or modifying properties that depend on the final state of the storage or context.
/// - Storage-specific fields which are not present in formed structure.
///
/// Look example `former_custom_mutator.rs`

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
/// - `Storage`: The type of the collection being processed.
/// - `Context`: The type of the context that might be altered or returned upon completion.

pub trait FormingEnd< Definition : crate::FormerDefinitionTypes >
{
  /// Called at the end of the subforming process to return the modified or original context.
  ///
  /// # Parameters
  /// - `collection`: The collection being processed.
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

/// A `FormingEnd` implementation that directly returns the formed collection as the final product of the forming process.
///
/// This struct is particularly useful when the end result of the forming process is simply the formed collection itself,
/// without needing to integrate or process additional contextual information. It's ideal for scenarios where the final
/// entity is directly derived from the storage state without further transformations or context-dependent adjustments.
#[ derive( Debug, Default ) ]
pub struct ReturnPreformed;

impl< Definition > FormingEnd< Definition >
for ReturnPreformed
where
  Definition::Storage : crate::StoragePreform< Preformed = Definition::Formed >,
  Definition : crate::FormerDefinitionTypes,
{
  /// Transforms the storage into its final formed state and returns it, bypassing context processing.
  #[ inline( always ) ]
  fn call( &self, storage : Definition::Storage, _context : core::option::Option< Definition::Context > ) -> Definition::Formed
  {
    crate::StoragePreform::preform( storage )
  }
}

/// A `FormingEnd` implementation that returns the storage itself as the formed entity, disregarding any contextual data.
///
/// This struct is suited for straightforward forming processes where the storage already represents the final state of the
/// entity, and no additional processing or transformation of the storage is required. It simplifies use cases where the
/// storage does not undergo a transformation into a different type at the end of the forming process.

#[ derive( Debug, Default ) ]
pub struct ReturnStorage;

impl< Definition, T > FormingEnd< Definition >
for ReturnStorage
where
  Definition : crate::FormerDefinitionTypes< Context = (), Storage = T, Formed = T >,
{
  /// Returns the storage as the final product of the forming process, ignoring any additional context.
  #[ inline( always ) ]
  fn call( &self, storage : Definition::Storage, _context : core::option::Option< () > ) -> Definition::Formed
  {
    storage
  }
}

/// A placeholder `FormingEnd` used when no end operation is required or applicable.
///
/// This implementation is useful in generic or templated scenarios where a `FormingEnd` is required by the interface,
/// but no meaningful end operation is applicable. It serves a role similar to `core::marker::PhantomData` by filling
/// generic parameter slots without contributing operational logic.
#[ derive( Debug, Default ) ]
pub struct NoEnd;

impl< Definition > FormingEnd< Definition >
for NoEnd
where
  Definition : crate::FormerDefinitionTypes,
{
  /// Intentionally causes a panic if called, as its use indicates a configuration error.
  #[ inline( always ) ]
  fn call( &self, _storage : Definition::Storage, _context : core::option::Option< Definition::Context > ) -> Definition::Formed
  {
    unreachable!();
  }
}

#[ allow( unused_extern_crates ) ]
#[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
extern crate alloc;
#[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
use alloc::boxed::Box;

/// A wrapper around a closure to be used as a `FormingEnd`.
///
/// This struct allows for dynamic dispatch of a closure that matches the
/// `FormingEnd` trait's `call` method signature. It is useful for cases where
/// a closure needs to be stored or passed around as an object implementing
/// `FormingEnd`.
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ allow( clippy::type_complexity ) ]
pub struct FormingEndClosure< Definition : crate::FormerDefinitionTypes >
{
  closure : Box< dyn Fn( Definition::Storage, Option< Definition::Context > ) -> Definition::Formed >,
  _marker : core::marker::PhantomData< Definition::Storage >,
}

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
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
      _marker : core::marker::PhantomData
    }
  }
}

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
impl< Definition : crate::FormerDefinitionTypes > FormingEndClosure< Definition >
{
  /// Constructs a new `FormingEndClosure` with the provided closure.
  ///
  /// # Parameters
  ///
  /// * `closure` - A closure that matches the expected signature for transforming a collection
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
      _marker : core::marker::PhantomData
    }
  }
}

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
use core::fmt;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
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

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
impl< Definition : crate::FormerDefinitionTypes > FormingEnd< Definition >
for FormingEndClosure< Definition >
{
  fn call( &self, storage : Definition::Storage, context : Option< Definition::Context > ) -> Definition::Formed
  {
    ( self.closure )( storage, context )
  }
}

/// A trait for initiating a structured subforming process with contextual and intermediary storage linkage.
///
/// This trait is crucial for the `derive(Former)` macro implementation, where it facilitates the creation
/// of a subformer that integrates seamlessly within a builder pattern chain. It handles intermediary storage
/// to accumulate state or data before finally transforming it into the final `Formed` structure.
///
/// `FormerBegin` is particularly important in scenarios where complex, hierarchical structures are formed,
/// allowing a former to be reused within another former. This reusability and the ability to maintain both transient
/// state (`Storage`) and contextual information (`Context`) are essential for multi-step construction or transformation
/// processes that culminate in the generation of a final product (`Formed`).
///
/// During code generation via the `derive(Former)` macro, `FormerBegin` provides the necessary scaffolding to
/// initiate the subforming process. This setup is critical for ensuring that all elements involved in the formation
/// are aligned from the onset, particularly when one former is nested within another, facilitating the creation
/// of complex hierarchical data structures.
///

pub trait FormerBegin< Definition :  >
where
  Definition : crate::FormerDefinition,
{

  /// Launches the subforming process with an initial storage and context, setting up an `on_end` completion handler.
  ///
  /// This method initializes the formation process by providing the foundational elements necessary for
  /// building the entity. It allows for the configuration of initial states and contextual parameters, which
  /// are critical for accurately reflecting the intended final state of the entity.
  ///
  /// # Parameters
  ///
  /// * `storage` - An optional initial state for the intermediary storage structure. This parameter allows
  ///   for the pre-configuration of storage, which can be crucial for entities requiring specific initial states.
  /// * `context` - An optional initial setting providing contextual information for the subforming process.
  ///   This context can influence how the formation process progresses, especially in complex forming scenarios.
  /// * `on_end` - A completion handler responsible for transforming the accumulated `Storage` into the final `Formed` structure.
  ///   This parameter is vital for ensuring that the transition from `Storage` to `Formed` is handled correctly,
  ///   incorporating any last-minute adjustments or validations necessary for the entity's integrity.
  ///
  /// # Returns
  ///
  /// Returns an instance of Former.
  ///
  fn former_begin
  (
    storage : core::option::Option< Definition::Storage >,
    context : core::option::Option< Definition::Context >,
    on_end : Definition::End,
  ) -> Self;

}
