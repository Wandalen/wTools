//!
//! # System of Types for Reflection
//!
//! This crate provides a comprehensive system for runtime type reflection, enabling dynamic type inspection and manipulation. It is designed to facilitate the integration of types into systems that require advanced operations such as serialization, deserialization, object-relational mapping (ORM), and interaction with generic containers and algorithms that operate on heterogeneous collections of entities.
//!
//! ## Features
//!
//! - **Dynamic Type Inspection**: Retrieve detailed type information at runtime, supporting complex scenarios like serialization frameworks that need to dynamically handle different data types.
//! - **Entity Manipulation**: Manipulate entities in a type-safe manner, leveraging Rust's powerful type system to ensure correctness while allowing dynamic behavior.
//! - **Reflection API**: Utilize a rich set of APIs to introspect and manipulate entities based on their runtime type information, enabling patterns that are not possible with static typing alone.
//! - **Support for Primitive and Composite Types**: Handle both primitive types (e.g., integers, floating-point numbers, strings) and composite entities (e.g., structs, arrays, maps) with a unified interface.
//!
//! ## Use Cases
//!
//! - **Serialization/Deserialization**: Automatically convert Rust structs to and from formats like JSON, XML, or binary representations, based on their runtime type information.
//! - **Dynamic ORM**: Map Rust entities to database tables dynamically, enabling flexible schema evolution and complex queries without sacrificing type safety.
//! - **Generic Algorithms**: Implement algorithms that operate on collections of heterogeneous types, performing runtime type checks and conversions as necessary.
//! - **Plugin Architectures**: Build systems that load and interact with plugins or modules of unknown types at compile time, facilitating extensibility and modularity.
//!
//! ## Getting Started
//!
//! To start using the reflection system, define your entities using the provided traits and enums, and then use the `reflect` function to introspect their properties and behavior at runtime. The system is designed to be intuitive for Rust developers familiar with traits and enums, with minimal boilerplate required to make existing types compatible.
//!
//! ## Example
// qqq : for Yulia : no ignore!
//!
//! ```rust, ignore
//! # use reflect_tools::reflect::{ reflect, Entity };
//!
//! // Define an entity that implements the Instance trait.
//! #[ derive( Debug ) ]
//! struct MyEntity
//! {
//!   id : i32,
//!   name : String,
//!   // other fields
//! }
//!
//! // Implement the required traits for MyEntity.
//! // ...
//!
//! // Use the reflection API to inspect `MyEntity`.
//! let entity = MyEntity { id: 1, name: "Entity Name".to_string() /*, other fields*/ };
//! let reflected = reflect( &entity );
//! println!( "{:?}", reflected.type_name() ); // Outputs "MyEntity"
//! ```
//!
//! ## Extending the System
//!
//! Implement additional traits for your types as needed to leverage the full power of the reflection system. The crate is designed to be extensible, allowing custom types to integrate seamlessly with the reflection mechanism.
//!

// qqq : make the example working. use tests for inpsisrations

/// Internal namespace.
pub( crate ) mod private
{
}

pub mod axiomatic;
pub mod entity_array;
pub mod entity_slice;
pub mod entity_vec;
pub mod entity_hashmap;
pub mod entity_hashset;
pub mod primitive;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::axiomatic::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_array::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_slice::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_vec::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_hashmap::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_hashset::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::primitive::orphan::*;
  // pub use super::private::
  // {
  // };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::axiomatic::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_array::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_slice::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_vec::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_hashmap::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_hashset::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::primitive::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::axiomatic::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_array::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_slice::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_vec::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_hashmap::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::entity_hashset::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::primitive::prelude::*;
}
