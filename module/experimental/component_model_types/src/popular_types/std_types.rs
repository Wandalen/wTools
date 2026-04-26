//! Standard library type support
//!
//! This module provides markers and utilities for standard library types that should receive
//! special treatment in `ComponentModel` derive macro generation.

/// Marker trait to identify types that should get popular type support
pub trait PopularType {}
