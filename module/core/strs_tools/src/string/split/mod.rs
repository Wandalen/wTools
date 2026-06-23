//! Provides tools for splitting strings with advanced options including quoting.
//!
//! # Architecture & Rule Compliance Notes
//!
//! ## Critical Design Insights:
//!
//! - **Lifetime Management**: All functions with references MUST use explicit lifetime parameters
//!   per Design Rulebook. The `unescape_str` function was corrected from `fn(input: &str)`
//!   to `fn<'a>(input: &'a str)` - this is non-negotiable for maintainability.
//!
//! - **Clippy Conflict Resolution**: The explicit lifetime requirement conflicts with clippy's
//!   `elidable_lifetime_names` warning. Design Rulebook takes precedence, so we use
//!   `#[ allow( clippy::elidable_lifetime_names ) ]` to suppress the warning while maintaining
//!   explicit lifetimes for architectural consistency.
//!
//! - **Namespace Pattern**: This module uses manual namespace blocks (own/orphan/exposed/prelude).
//!   Public items are re-exported through explicit `pub use` statements in each namespace tier.
//!
//! - **SIMD Optimization Dependencies**: memchr, aho-corasick, bytecount are optional
//!   dependencies for performance optimization. They MUST be declared in workspace Cargo.toml
//!   and inherited, not declared locally.
//!
//! ## Performance Pitfalls:
//!
//! - **Cow<'_, str> Usage**: The `unescape_str` function returns `Cow::Borrowed` when no
//!   unescaping is needed, avoiding unnecessary allocations. This is critical for performance
//!   when processing large text with minimal escaping.
//!
//! - **Iterator State Management**: `SplitFastIterator` maintains internal state. All tests
//!   live in the `tests/` directory (per l2_imp rulebook).
//!
//! ## Security Considerations:
//!
//! - **Consumer Owns Unescaping**: This module does NOT interpret escape sequences for security.
//!   Raw string slices are returned, and the consumer must handle unescaping safely.
//!   This prevents injection attacks through malformed escape sequences.

mod types;
mod iterator;
mod builder;

mod split_behavior;
pub use split_behavior::SplitFlags;

#[ cfg( feature = "simd" ) ]
mod simd;
#[ cfg( feature = "simd" ) ]
pub use simd::{ SIMDSplitIterator, simd_split_cached, get_or_create_cached_patterns };

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
  pub use super::types::{ Split, SplitType, Searcher };
  pub use super::iterator::SplitIterator;
  pub use super::builder::{ BasicSplitBuilder, split };
  #[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
  pub use super::builder::{ split_advanced, SplitOptionsFormer };
  #[ cfg( feature = "simd" ) ]
  pub use super::{ SIMDSplitIterator, simd_split_cached, get_or_create_cached_patterns };
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*;
  pub use super::own::{ Split, SplitType, SplitIterator, Searcher, BasicSplitBuilder, split };
  #[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
  pub use super::own::{ split_advanced, SplitOptionsFormer };
  #[ cfg( feature = "simd" ) ]
  pub use super::own::{ SIMDSplitIterator, simd_split_cached, get_or_create_cached_patterns };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use super::types::Searcher;
  pub use super::builder::{ BasicSplitBuilder, split };
  #[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
  pub use super::builder::{ SplitOptionsFormer, split_advanced };
}
