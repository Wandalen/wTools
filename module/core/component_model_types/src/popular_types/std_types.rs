//! Standard library type support
//!
//! This module provides markers and utilities for standard library types that should receive
//! special treatment in `ComponentModel` derive macro generation.

// Standard library types are used for Default implementations

/// Marker trait to identify types that should get popular type support
pub trait PopularType {}

// Note: We cannot implement foreign traits for foreign types due to orphan rules
// The actual implementations will be generated in the derive macro

// TODO: SocketAddr doesn't implement Default by default, so structs using it need
// to provide their own Default implementation or use #[derive(Default)] won't work