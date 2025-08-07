//! # Former Types - Core Trait Definitions and Type System Integration
//!
//! This crate provides the foundational trait definitions and type system integration
//! for the Former builder pattern ecosystem. It defines the core abstractions that
//! enable flexible and extensible builder pattern implementations.
//!
//! ## Core Abstractions
//!
//! ### Formation Process Management
//! The crate defines several key traits that manage the formation process:
//!
//! - **[`FormerDefinition`]**: Links entities to their formation definitions
//! - **[`FormerDefinitionTypes`]**: Specifies storage, formed, and context types
//! - **[`FormingEnd`]**: Handles completion of the formation process
//! - **[`FormerMutator`]**: Enables pre-formation data validation and manipulation
//! - **[`FormerBegin`]**: Initiates subform creation with proper context
//!
//! ### Storage Management
//! - **[`Storage`]**: Defines the interface for temporary state during formation
//! - **[`StoragePreform`]**: Handles transition from storage to final formed state
//!
//! ### Collection Integration
//! Specialized support for collection types when the `types_former` feature is enabled:
//! - Automatic trait implementations for standard collections
//! - Custom collection support through extensible trait system
//! - Type-safe collection builders with proper generic handling
//!
//! ## Architecture Design
//!
//! ### Type Safety and Generics
//! The trait system is designed to handle complex generic scenarios:
//! - **Lifetime Parameters**: Full support for complex lifetime relationships
//! - **Generic Constraints**: Proper constraint propagation through the type system
//! - **Associated Types**: Clean separation of concerns through associated types
//!
//! ### Builder Pattern Integration
//! The traits work together to enable:
//! - **Fluent Interfaces**: Method chaining with compile-time validation
//! - **Subform Support**: Nested builders with proper context preservation
//! - **Custom Validation**: Pre-formation validation and transformation
//! - **Flexible End Conditions**: Customizable formation completion logic
//!
//! ## Feature Gates
//!
//! - **`types_former`**: Enables core Former trait definitions
//! - **`use_alloc`**: Enables allocation-dependent features in no-std environments
//! - **`no_std`**: Full no-std compatibility when used without std-dependent features
//!
//! ## Integration with Former Ecosystem
//!
//! This crate serves as the foundation for:
//! - **[`former`]**: Main user-facing crate with derive macro
//! - **[`former_meta`]**: Procedural macro implementation
//! - **Collection Tools**: Integration with external collection libraries
//!
//! ## Usage Patterns
//!
//! Most users will not interact with this crate directly, but will instead use
//! the higher-level [`former`] crate. However, this crate is essential for:
//! - Custom Former implementations
//! - Integration with external libraries
//! - Advanced builder pattern scenarios

#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/former_types/latest/former_types/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Former pattern types" ) ]

/// ## Formation Definition System
///
/// Core trait definitions that establish the relationship between entities and their
/// formation processes. Defines how types are linked to their builders, storage
/// mechanisms, and completion handlers.
///
/// Key traits: [`FormerDefinition`], [`FormerDefinitionTypes`], entity mapping traits.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
pub mod definition;

/// ## Formation Process Management
///
/// Traits and types that manage the formation lifecycle, including process initiation,
/// mutation, and completion. Provides the foundational abstractions for controlling
/// how entities are constructed through the builder pattern.
///
/// Key traits: [`FormingEnd`], [`FormerMutator`], [`FormerBegin`], completion handlers.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
pub mod forming;

/// ## Storage Interface System
///
/// Defines the storage mechanisms that maintain intermediate state during entity
/// formation. Provides traits for managing temporary data and transforming it
/// into final formed structures.
///
/// Key traits: [`Storage`], [`StoragePreform`], storage lifecycle management.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
pub mod storage;

/// ## Collection Interface System
///
/// Provides specialized support for collection types within the Former pattern.
/// Defines traits and implementations that enable seamless integration with
/// standard collections like Vec, HashMap, HashSet, and custom collection types.
///
/// ### Key Features
/// - Entry-to-value conversion abstractions
/// - Value-to-entry transformation support
/// - Collection-specific builder patterns
/// - Type-safe collection manipulation
///
/// This module is only available with std or when the `use_alloc` feature is enabled.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "types_former" ) ]
mod collection;

/// ## Namespace with dependencies
///
/// Exposes the external dependencies used by former_types for advanced integration
/// scenarios and custom implementations.
///
/// ### Dependencies
/// - [`collection_tools`]: Comprehensive collection manipulation utilities
///
/// ### Usage
/// This namespace is primarily intended for library authors and advanced users
/// who need direct access to the underlying collection tools for custom Former
/// implementations or specialized collection handling.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::collection_tools;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{

  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{

  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "types_former" ) ]
  pub use collection::orphan::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{

  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ cfg( feature = "types_former" ) ]
  pub use super::{ definition::*, forming::*, storage::* };

  #[ doc( inline ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "types_former" ) ]
  pub use collection::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{

  use super::*;

  #[ doc( inline ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "types_former" ) ]
  pub use collection::prelude::*;
}
