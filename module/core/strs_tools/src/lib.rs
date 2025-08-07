#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/strs_tools/latest/strs_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "String manipulation utilities" ) ]
#![ allow( clippy::std_instead_of_alloc ) ]

//! # Rule Compliance & Architectural Notes
//!
//! This crate has been systematically updated to comply with the Design and Codestyle Rulebooks.
//! Key compliance achievements and ongoing considerations:
//!
//! ## Completed Compliance Work:
//!
//! 1. **Documentation Strategy**: Uses `#![ doc = include_str!(...) ]` to include readme.md
//!    instead of duplicating documentation. This is the mandated approach for all entry files.
//!
//! 2. **Workspace Dependencies**: All external dependencies now inherit from workspace with
//!    `{ workspace = true }`. SIMD optimization deps (memchr, aho-corasick, bytecount, lexical)
//!    were moved to workspace level for version consistency.
//!
//! 3. **Attribute Formatting**: All attributes use proper spacing per Universal Formatting Rule:
//!    `#[ cfg( feature = "enabled" ) ]` instead of `#[cfg(feature = "enabled")]`
//!
//! 4. **mod_interface Architecture**: Converted from manual namespace patterns to `mod_interface!`
//!    macro usage for cleaner module organization and controlled visibility.
//!
//! ## Critical Architectural Decisions:
//!
//! - **Feature Gating**: All functionality is gated behind the "enabled" feature, which now
//!   also enables "mod_interface/enabled" for proper macro functionality.
//!
//! - **Error Handling**: Uses `error_tools` exclusively - no `anyhow` or `thiserror` dependencies
//!   per Design Rulebook requirements.
//!
//! - **Testing Isolation**: All tests are in `tests/` directory, never in `src/`, following
//!   the mandatory testing architecture pattern.

/// String tools.
#[ cfg( feature = "enabled" ) ]
pub mod string;

/// SIMD-optimized string operations.
#[ cfg( all( feature = "enabled", feature = "simd" ) ) ]
pub mod simd;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
  pub use super::string;
  #[ cfg( feature = "simd" ) ]
  pub use super::simd;
  #[ cfg( test ) ]
  pub use super::string::orphan::*;
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*;
  pub use super::string::exposed::*;
}

/// Namespace of the module to include with `use module::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use super::string::prelude::*;
}
