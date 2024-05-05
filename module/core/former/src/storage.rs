// storage.rs

/// Defines the storage interface for entities being constructed using a forming pattern.
///
/// This trait is required for any storage type that temporarily holds data during the construction
/// of an entity. It mandates the implementation of `Default`, ensuring that storage can be initialized
/// to a default state at the start of the forming process.
pub trait Storage : ::core::default::Default
{
  /// The type of the fully formed entity that results from the forming process.
  type Formed;
}

/// Provides a mechanism to finalize the forming process by converting storage into its final formed state.
///
/// This trait is crucial for transitioning the mutable, intermediate storage state into the final,
/// immutable state of an entity. The transformation is typically performed once all configurations
/// and modifications are applied to the storage during the forming process. The type `Preformed` and `Formed` is
/// generally the structure for which the `Former` trait is derived, representing the fully formed
/// state of the entity. However, it can differ if a custom `FormingEnd` or a different `Formed` type
/// is defined to handle specific forming logic or requirements.
/// But even if `Formed` is custom `Preformed` is always that structure.
pub trait StoragePreform
{
  /// The type of the entity as it should appear once fully formed.
  type Preformed;

  /// Transforms the storage into the final formed state of the entity.
  ///
  /// This function is called at the conclusion of the forming process to finalize the entity's state,
  /// effectively turning the mutable storage state into the immutable, fully formed entity. This transition
  /// reflects the culmination of the forming process where the temporary, modifiable attributes of the
  /// storage are solidified into the permanent attributes of the formed entity.
  fn preform( self ) -> Self::Preformed;
}
