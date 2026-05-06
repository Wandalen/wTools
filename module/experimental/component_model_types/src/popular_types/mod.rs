//! Popular type support for component model
//!
//! This module provides built-in implementations of `Assign` trait for commonly used Rust types
//! to eliminate manual implementation boilerplate and improve developer experience.

#[ cfg( feature = "types_component_assign" ) ]
pub mod std_types;

#[ cfg( feature = "types_component_assign" ) ]
pub use std_types :: *;