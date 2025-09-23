//! Popular type support for component model
//!
//! This module provides built-in implementations of `Assign` trait for commonly used Rust types
//! to eliminate manual implementation boilerplate and improve developer experience.

#[ cfg( feature = "types_component_assign" ) ]
pub mod std_types;

// Feature-gated type support
// TODO: Implement these in Phase 2
// #[ cfg( all( feature = "types_component_assign", feature = "uuid" ) ) ]
// pub mod uuid_support;

// #[ cfg( all( feature = "types_component_assign", feature = "url" ) ) ]  
// pub mod url_support;

// #[ cfg( all( feature = "types_component_assign", feature = "serde" ) ) ]
// pub mod serde_support;

#[ cfg( feature = "types_component_assign" ) ]
pub use std_types :: *;