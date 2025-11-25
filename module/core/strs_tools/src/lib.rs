#![ cfg_attr( all( feature = "no_std", not( feature = "std" ) ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/strs_tools/latest/strs_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "String manipulation utilities" ) ]
#![ allow( clippy::std_instead_of_alloc ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::elidable_lifetime_names ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::manual_strip ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::new_without_default ) ]
#![ allow( clippy::clone_on_copy ) ]
#![ allow( clippy::single_match_else ) ]
#![ allow( clippy::return_self_not_must_use ) ]
#![ allow( clippy::match_same_arms ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::iter_cloned_collect ) ]
#![ allow( clippy::redundant_closure ) ]
#![ allow( clippy::uninlined_format_args ) ]

//! # Rule Compliance & Architectural Notes
//!
//! This crate has been systematically updated to comply with the Design and Codestyle Rulebooks.
//! Key compliance achievements and ongoing considerations :
//!
//! ## Completed Compliance Work :
//!
//! 1. **Documentation Strategy** : Uses `#![ doc = include_str!(...) ]` to include readme.md
//!    instead of duplicating documentation. This is the mandated approach for all entry files.
//!
//! 2. **Workspace Dependencies** : All external dependencies now inherit from workspace with
//!    `{ workspace = true }`. SIMD optimization deps (memchr, aho-corasick, bytecount, lexical)
//!    were moved to workspace level for version consistency.
//!
//! 3. **Attribute Formatting** : All attributes use proper spacing per Universal Formatting Rule :
//!    `#[ cfg( feature = "enabled" ) ]` instead of `#[ cfg( feature = "enabled" ) ]`
//!
//! 4. **Manual Namespace Architecture** : Uses the standard wTools manual namespace pattern
//!    (private/own/orphan/exposed/prelude) for precise API control and stable public interfaces.
//!
//! ## Critical Architectural Decisions :
//!
//! - **Feature Gating** : All functionality is gated behind the "enabled" feature for
//!   granular control over compilation and dependencies.
//!
//! - **Error Handling** : Uses `error_tools` exclusively - no `anyhow` or `thiserror` dependencies
//!   per Design Rulebook requirements.
//!
//! - **Testing Isolation** : All tests are in `tests/` directory, never in `src/`, following
//!   the mandatory testing architecture pattern.

/// String tools.
#[ cfg( feature = "enabled" ) ]
pub mod string;

/// SIMD-optimized string operations.
#[ cfg( all( feature = "enabled", feature = "simd" ) ) ]
pub mod simd;

/// ANSI escape sequence handling utilities.
#[ cfg( all( feature = "enabled", feature = "ansi" ) ) ]
pub mod ansi;

/// Re-export compile-time optimization macros.
#[ cfg( all( feature = "enabled", feature = "compile_time_optimizations" ) ) ]
#[ allow( unused_imports ) ]
pub use strs_tools_meta::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
  pub use super::string;
  #[ cfg( feature = "simd" ) ]
  pub use super::simd;
  #[ cfg( feature = "ansi" ) ]
  pub use super::ansi;
  #[ cfg( test ) ]
  pub use super::string::orphan::*;
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*;
  pub use super::string::exposed::*;
}

/// Namespace of the module to include with `use module::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( unused_imports ) ]
  use super::*;
  pub use super::string::prelude::*;
}
