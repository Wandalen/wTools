//! Module `definition`
//!
//! Provides traits for defining the relationships between entities and their formation mechanisms.
//! These traits are central to the implementation of a flexible and extensible formation system,
//! enabling entities to be constructed using various configurations and complex logic.
//!
//! Key aspects of the module include:
//! - **Entity to Definition Mapping**: Linking entities to their specific formation definitions,
//!   which detail how they are to be constructed.
//! - **Entity to Former Mapping**: Associating entities with formers that handle their construction
//!   process.
//! - **Entity to Storage Mapping**: Defining the storage structures that maintain the state of an
//!   entity during its formation.
//! - **Definition Traits**: Specifying the properties and ending conditions of the formation
//!   process to ensure entities are formed according to specified rules and logic.
//!

/// Maps a type of entity to its corresponding former definition.
/// This trait provides a linkage between the entity and its definition,
/// allowing the formation logic to understand what definition to apply
/// during the formation process.
pub trait EntityToDefinition< Context, Formed, End >
{
  /// The specific [`FormerDefinition`] associated with this entity.
  type Definition : FormerDefinition;
  /// The specific [`FormerDefinitionTypes`] associated with this entity.
  type Types : FormerDefinitionTypes;
}

/// Provides a mapping between a type of entity and its associated formation type definitions.
pub trait EntityToDefinitionTypes< Context, Formed >
{
  /// Specifies the `FormerDefinitionTypes` that define the storage, formed entity, and context types used during formation.
  /// This association is essential for ensuring that the formation process is carried out with the correct type-specific logic.
  type Types : FormerDefinitionTypes;
}

/// Maps a type of entity to its corresponding former.
/// This trait binds an entity type to a specific former, facilitating the use
/// of custom formers in complex formation scenarios.
pub trait EntityToFormer< Definition >
where
  Definition : FormerDefinition,
{
  /// The type of the former used for building the entity.
  type Former;

  /// A placeholder function to reference the definition without operational logic to calm compiler.
  fn __f(_: &Definition) {}
}

/// Maps a type of entity to its storage type.
/// This trait defines what storage structure is used to hold the interim state
/// of an entity during its formation.
pub trait EntityToStorage
{
  /// The storage type used for forming the entity.
  type Storage;
}

/// Defines the fundamental components involved in the formation of an entity.
/// This trait specifies the types of storage, the formed entity, and the context
/// used during the formation process.
pub trait FormerDefinitionTypes : Sized
{
  /// The type of storage used to maintain the state during formation.
  type Storage : Default;

  /// The type of the entity once fully formed.
  type Formed;

  /// The contextual information used during formation, if any.
  type Context;
}

/// Expands on `FormerDefinitionTypes` by incorporating an ending mechanism for the formation process.
/// This trait connects the formation types with a specific endpoint, defining
/// how the formation process concludes, including any necessary transformations
/// or validations.
pub trait FormerDefinition : Sized
{
  /// Encapsulates the types related to the formation process including any mutators.
  type Types : crate::FormerDefinitionTypes< Storage = Self::Storage, Formed = Self::Formed, Context = Self::Context >
  + crate::FormerMutator;

  /// Defines the ending condition or operation of the formation process.
  type End: crate::FormingEnd< Self::Types >;

  /// The storage type used during the formation.
  type Storage : Default;

  /// The type of the entity being formed. It is
  /// generally the structure for which the `Former` is derived, representing the fully formed
  /// state of the entity. However, it can differ if a custom `FormingEnd` or a different `Formed` type
  /// is defined to handle specific forming logic or requirements.
  type Formed;

  /// The context used during the formation process.
  type Context;
}
