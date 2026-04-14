//! Common test utilities and helpers
//!
//! This module contains shared test infrastructure to eliminate code duplication
//! across test files following the Anti-Duplication Principle.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

pub mod alignment_helpers;
pub mod test_helpers;

pub use alignment_helpers::visual_position;
pub use test_helpers::sample_data;
