//! # Former - Advanced Builder Pattern Implementation
//!
//! The Former crate provides a comprehensive derive macro ecosystem for implementing the Builder Pattern
//! in Rust with advanced features like subform support, custom validation, and flexible configuration.
//!
//! ## Core Features
//!
//! - **Fluent Builder API**: Generate clean, ergonomic builder interfaces
//! - **Advanced Generic Support**: Handle complex generic parameters and lifetime constraints
//! - **Subform Integration**: Build nested structures with full type safety
//! - **Collection Builders**: Specialized support for Vec, HashMap, HashSet, and custom collections
//! - **Custom Validation**: Pre-formation validation through custom mutators
//! - **Flexible Configuration**: Extensive attribute system for fine-grained control
//! - **No-std Compatibility**: Full support for no-std environments with optional alloc
//!
//! ## Quick Start
//!
//! ```rust
//! use former::Former;
//!
//! #[derive(Debug, PartialEq, Former)]
//! pub struct UserProfile {
//!     age: i32,
//!     username: String,
//!     bio_optional: Option<String>,
//! }
//!
//! let profile = UserProfile::former()
//!     .age(30)
//!     .username("JohnDoe".to_string())
//!     .bio_optional("Software Developer".to_string())
//!     .form();
//! ```
//!
//! ## Architecture Overview
//!
//! The Former pattern generates several key components:
//! - **Storage Struct**: Holds intermediate state during building (all fields are `Option<T>`)
//! - **Former Struct**: The main builder providing the fluent API
//! - **Definition Types**: Type system integration for advanced scenarios
//! - **Trait Implementations**: Integration with the broader Former ecosystem
//!
//! ## Debug Support
//!
//! The Former derive macro provides comprehensive debugging capabilities through the `#[debug]` attribute,
//! following the design principle that "Proc Macros: Must Implement a 'debug' Attribute".
//!
//! ### Using Debug Attribute
//!
//! ```rust
//! use former::Former;
//!
//! // Standalone debug attribute
//! #[derive(Debug, PartialEq, Former)]
//! // #[debug]  // <-- Commented out - debug attribute only for temporary debugging
//! pub struct Person {
//!     name: String,
//!     age: u32,
//!     email: Option<String>,
//! }
//!
//! // Within #[former(...)] container
//! #[derive(Debug, PartialEq, Former)]
//! // #[former(debug, standalone_constructors)]  // <-- Debug commented out
//! pub struct Config {
//!     host: String,
//!     port: u16,
//! }
//! ```
//!
//! ### Debug Output Categories
//!
//! When `#[debug]` is present and the `former_diagnostics_print_generated` feature is enabled,
//! the macro provides detailed information in four phases:
//!
//! 1. **Input Analysis**: Target type, generic parameters, fields/variants, attribute configuration
//! 2. **Generic Classification**: How generics are categorized and processed
//! 3. **Generated Components**: Complete breakdown of Former ecosystem components
//! 4. **Final Generated Code**: The complete TokenStream output
//!
//! ### Enabling Debug Output
//!
//! ```bash
//! # See debug information during compilation
//! cargo build --features former_diagnostics_print_generated
//!
//! # For examples
//! cargo run --example former_debug --features former_diagnostics_print_generated
//! ```
//!
//! ### Debug Benefits
//!
//! - **Understand Macro Behavior**: See exactly how the macro processes your struct/enum
//! - **Debug Complex Scenarios**: Troubleshoot generic parameters, lifetimes, trait bounds
//! - **Learn Former Pattern**: Understand the complete generated ecosystem
//! - **Verify Configuration**: Confirm attribute parsing and code generation decisions
//!
//! ## Integration Points
//!
//! This crate serves as the main entry point and integrates:
//! - [`former_meta`]: Procedural macro implementation
//! - [`former_types`]: Core traits and type definitions
//! - External collections through [`collection_tools`]
//!
//! For detailed examples and advanced usage patterns, see the module documentation
//! and the comprehensive examples in the repository.

#![cfg_attr(feature = "no_std", no_std)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/former/latest/former/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Code generation and builder patterns" ) ]

// xxx : introduce body( struct/enum ) attribute `standalone_constructors` which create stand-alone, top-level constructors for struct/enum. for struct it's always single function, for enum it's as many functions as enum has vartianys. if there is no `arg_for_constructor` then constructors expect exaclty zero arguments. start from implementations without respect of attribute attribute `arg_for_constructor`. by default `standalone_constructors` is false
// xxx : introduce field attribute to mark an attribute `arg_for_constructor` as an argument which should be used in constructing functions ( either standalone consturcting function or associated with struct ). in case of enums attribute `arg_for_constructor` is attachable only to fields of variant and attempt to attach attribute `arg_for_constructor` to variant must throw understandable error. name standalone constructor of struct the same way struct named, but snake case and for enums the same name variant is named, but snake case. by default it's false.

// xxx : add to readme example with enums
// xxx : disable and phase out attribute "[ perform( fn method_name<...> () -> OutputType ) ]"
// xxx : split out crate component model
// xxx : fix commented out tests

/// ## Namespace with dependencies
///
/// This module exposes the direct dependencies of the Former crate, providing
/// access to the underlying implementation modules for advanced use cases.
///
/// ### Dependencies
/// - [`former_types`]: Core trait definitions and type system integration
/// - [`former_meta`]: Procedural macro implementation and code generation
///
/// ### Usage
/// Most users should import from the main crate or prelude rather than directly
/// from dependencies. This namespace is primarily for:
/// - Advanced integrations requiring direct access to core traits
/// - Custom implementations extending the Former ecosystem
/// - Library authors building on top of Former's foundation
#[cfg(feature = "enabled")]
pub mod dependency {
  pub use former_types;
  pub use former_meta;
}

#[doc(inline)]
#[allow(unused_imports)]
#[cfg(feature = "enabled")]
pub use own::*;

/// ## Own namespace of the module
///
/// Contains the core public API of the Former crate. This namespace follows
/// the standard wTools namespace pattern, providing organized access to
/// functionality while maintaining clear separation of concerns.
///
/// ### Key Exports
/// - All items from [`orphan`] namespace
/// - [`derive`]: Alias to [`former_meta`] for convenient access to derive macros
///
/// ### Usage Pattern
/// This namespace is typically accessed through `use former::own::*` for
/// explicit imports, or through the main crate exports.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod own {

  use super::*;
  #[doc(inline)]
  pub use orphan::*;
  #[doc(inline)]
  #[allow(unused_imports)]
  pub use former_meta as derive;
}

/// ## Parented namespace of the module
///
/// Intermediate namespace layer in the wTools namespace hierarchy. This namespace
/// provides access to exposed functionality while maintaining the architectural
/// separation between different visibility levels.
///
/// ### Architecture Role
/// In the wTools namespace pattern:
/// - **dependency**: External dependencies
/// - **own**: Complete module interface  
/// - **orphan**: Parented/inherited interface
/// - **exposed**: Public API surface
/// - **prelude**: Essential imports
///
/// This pattern enables fine-grained control over what gets exposed at each level.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod orphan {

  use super::*;
  #[doc(inline)]
  pub use exposed::*;
}

/// ## Exposed namespace of the module
///
/// Contains the main public API surface of the Former crate. This namespace
/// aggregates all functionality that should be available to users of the crate.
///
/// ### Key Exports
/// - **Prelude**: Essential traits and types via [`prelude`]
/// - **Derive Macros**: Complete procedural macro interface via [`former_meta`]
/// - **Core Types**: Fundamental traits and definitions via [`former_types::exposed`]
///
/// ### Usage
/// This namespace contains everything needed for typical Former usage:
/// ```rust
/// use former::exposed::*;
/// // Now you have access to Former derive macro and all supporting traits
/// ```
///
/// Most users will access this through the main crate re-exports rather than directly.
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod exposed {

  use super::*;

  #[doc(inline)]
  pub use prelude::*;

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use former_meta::*;

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use former_types::exposed::*;
}

/// ## Prelude to use essentials
///
/// Contains the most commonly used items from the Former crate ecosystem.
/// This module follows the standard Rust prelude pattern, providing a curated
/// set of imports for typical usage scenarios.
///
/// ### Key Exports
/// - **Essential Traits**: Core traits from [`former_types::prelude`]
/// - **Common Types**: Frequently used type definitions
/// - **Builder Patterns**: Standard builder pattern implementations
///
/// ### Usage
/// Import the prelude to get started quickly with Former:
/// ```rust
/// use former::prelude::*;
/// use former::Former;
/// 
/// // Now you have access to the most common Former functionality
/// #[derive(Former)]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// ### Design Philosophy
/// The prelude is designed to be safe to glob-import (`use former::prelude::*`)
/// and contains only items that are:
/// - Commonly used in typical Former scenarios
/// - Unlikely to cause naming conflicts
/// - Essential for basic functionality
#[cfg(feature = "enabled")]
#[allow(unused_imports)]
pub mod prelude {
  use super::*;

  #[doc(inline)]
  #[allow(unused_imports)]
  pub use former_types::prelude::*;
}
