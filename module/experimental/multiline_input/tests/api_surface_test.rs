//! Public API surface tests
//!
//! ## Domain
//!
//! Tests for public API:
//! - API types are exposed correctly
//! - Builder pattern works
//!
//! ## Organization
//!
//! Tests migrated from `src/lib.rs` module tests.

use multiline_input::Builder;

#[ test ]
fn test_api_exists()
{
  // Compile-time check that API is correctly exposed
  let _builder = Builder::new();
}
