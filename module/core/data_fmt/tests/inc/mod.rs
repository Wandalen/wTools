//! Common test utilities and helpers
//!
//! This module contains shared test infrastructure to eliminate code duplication
//! across test files following the Anti-Duplication Principle.
//!
//! ## Re-exports
//!
//! Convenience re-exports so callers can write `use inc::sample_data` instead of
//! `use inc::test_helpers::sample_data`. Each integration test binary independently
//! compiles this module and only uses some re-exports — the `unused_imports` lint fires
//! for the ones not used in that binary. Suppression is permanent for this pattern.

pub mod alignment_helpers;
pub mod test_helpers;

#[ allow( unused_imports ) ] // Rust integration test artifact: each binary uses only some re-exports
pub use alignment_helpers::visual_position;
#[ allow( unused_imports ) ] // Rust integration test artifact: each binary uses only some re-exports
pub use test_helpers::sample_data;
