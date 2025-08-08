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
///
/// This trait establishes a fundamental relationship in the Former pattern by linking
/// an entity type to its complete formation definition. It serves as the bridge between
/// the user's struct/enum and the generated Former ecosystem.
///
/// # Type Parameters
/// - `Context`: The contextual information available during formation
/// - `Formed`: The final type that results from the formation process
/// - `End`: The ending condition or operation for the formation process
///
/// # Associated Types
/// - [`Definition`]: The complete [`FormerDefinition`] that governs this entity's formation
/// - [`Types`]: The type system integration via [`FormerDefinitionTypes`]
///
/// # Usage in Generated Code
/// This trait is automatically implemented by the `#[ derive( Former ) ]` macro and should
/// not typically be implemented manually. It enables the Former pattern to:
/// - Determine the correct storage type for an entity
/// - Link to the appropriate former struct
/// - Apply the correct formation logic
/// - Handle generic parameters and constraints properly
///
/// # Example Context
/// ```rust, ignore
/// // For a struct like this:
/// #[ derive( Former ) ]
/// struct User { name: String, age: u32 }
///
/// // The macro generates an implementation like:
/// impl EntityToDefinition<(), User, former::ReturnPreformed> for User {
///     type Definition = UserDefinition;
///     type Types = UserDefinitionTypes;
/// }
/// ```
pub trait EntityToDefinition<Context, Formed, End> {
  /// The specific [`FormerDefinition`] associated with this entity.
  ///
  /// This definition contains all the information needed to construct instances
  /// of the entity, including storage types, formation logic, and completion handlers.
  type Definition: FormerDefinition;
  
  /// The specific [`FormerDefinitionTypes`] associated with this entity.
  ///
  /// These types provide the type system integration, defining the storage,
  /// formed result, and context types used throughout the formation process.
  type Types: FormerDefinitionTypes;
}

/// Provides a mapping between a type of entity and its associated formation type definitions.
///
/// This trait is a simplified version of [`EntityToDefinition`] that focuses purely on type
/// relationships without requiring end condition specification. It's particularly useful
/// in scenarios where the formation logic needs to understand type relationships without
/// needing complete formation control.
///
/// # Type Parameters
/// - `Context`: The contextual information available during formation
/// - `Formed`: The final type that results from the formation process
///
/// # Purpose and Usage
/// This trait serves as a building block for more complex formation scenarios:
/// - Type system integration for subforms
/// - Generic parameter propagation in nested structures
/// - Context type determination in hierarchical builders
/// - Storage type resolution for complex generic scenarios
///
/// # Relationship to Other Traits
/// - Simpler than [`EntityToDefinition`] as it doesn't specify end conditions
/// - Used internally by the Former macro for type resolution
/// - Enables proper generic parameter handling in complex hierarchies
pub trait EntityToDefinitionTypes<Context, Formed> {
  /// Specifies the `FormerDefinitionTypes` that define the storage, formed entity, and context types used during formation.
  ///
  /// This association is essential for ensuring that the formation process is carried out
  /// with the correct type-specific logic. The types specified here must be consistent
  /// with the entity's actual structure and requirements.
  ///
  /// # Type Requirements
  /// The associated [`Types`] must implement [`FormerDefinitionTypes`] with:
  /// - `Storage` type compatible with the entity's field requirements
  /// - `Formed` type matching the target entity type
  /// - `Context` type appropriate for the formation scenario
  type Types: FormerDefinitionTypes;
}

/// Maps a type of entity to its corresponding former (builder) implementation.
///
/// This trait establishes the connection between an entity and its builder struct,
/// enabling the Former pattern to instantiate the correct builder type for a given entity.
/// It's a crucial part of the type system that ensures type safety across the formation process.
///
/// # Type Parameters
/// - `Definition`: The [`FormerDefinition`] that governs the formation process
///
/// # Purpose and Design
/// This trait enables:
/// - **Type-Safe Builder Resolution**: Ensures the correct builder is used for each entity
/// - **Generic Parameter Preservation**: Maintains generic constraints through builder creation
/// - **Custom Former Support**: Allows for specialized builder implementations
/// - **Subform Integration**: Enables nested builders with proper type relationships
///
/// # Usage in Generated Code
/// The `#[ derive( Former ) ]` macro automatically implements this trait:
/// ```rust, ignore
/// // For a struct like:
/// #[ derive( Former ) ]
/// struct Config { setting: String }
///
/// // The macro generates:
/// impl EntityToFormer<ConfigDefinition> for Config {
///     type Former = ConfigFormer<ConfigDefinition>;
/// }
/// ```
///
/// # Integration Points
/// This trait works with:
/// - [`EntityToDefinition`]: For complete entity-to-formation mapping
/// - [`FormerBegin`]: For initiating the formation process
/// - Generated former structs: For the actual builder implementation
pub trait EntityToFormer<Definition>
where
  Definition: FormerDefinition,
{
  /// The type of the former (builder) used for constructing the entity.
  ///
  /// This type must implement the necessary builder pattern methods and integrate
  /// properly with the Former ecosystem. It typically includes:
  /// - Setter methods for each field
  /// - Subform support for nested structures
  /// - Collection builders for container fields
  /// - Generic parameter preservation
  type Former;

  /// A placeholder function to reference the definition without operational logic.
  ///
  /// This function exists solely to establish a compile-time relationship with the
  /// `Definition` parameter and has no runtime behavior. It helps the compiler
  /// understand the type relationships in complex generic scenarios.
  ///
  /// # Implementation Note
  /// This is a workaround for Rust's type system limitations in expressing phantom
  /// type relationships. It should never be called in actual code.
  fn __f(_: &Definition) {}
}

/// Maps a type of entity to its storage type.
/// This trait defines what storage structure is used to hold the interim state
/// of an entity during its formation.
pub trait EntityToStorage {
  /// The storage type used for forming the entity.
  type Storage;
}

/// Defines the fundamental components involved in the formation of an entity.
/// This trait specifies the types of storage, the formed entity, and the context
/// used during the formation process.
pub trait FormerDefinitionTypes: Sized {
  /// The type of storage used to maintain the state during formation.
  type Storage: Default;

  /// The type of the entity once fully formed.
  type Formed;

  /// The contextual information used during formation, if any.
  type Context;
}

/// Expands on `FormerDefinitionTypes` by incorporating an ending mechanism for the formation process.
/// This trait connects the formation types with a specific endpoint, defining
/// how the formation process concludes, including any necessary transformations
/// or validations.
pub trait FormerDefinition: Sized {
  /// Encapsulates the types related to the formation process including any mutators.
  type Types: crate::FormerDefinitionTypes<Storage = Self::Storage, Formed = Self::Formed, Context = Self::Context>
    + crate::FormerMutator;

  /// Defines the ending condition or operation of the formation process.
  type End: crate::FormingEnd<Self::Types>;

  /// The storage type used during the formation.
  type Storage: Default;

  /// The type of the entity being formed. It is
  /// generally the structure for which the `Former` is derived, representing the fully formed
  /// state of the entity. However, it can differ if a custom `FormingEnd` or a different `Formed` type
  /// is defined to handle specific forming logic or requirements.
  type Formed;

  /// The context used during the formation process.
  type Context;
}
