//!
//! Prelude namespace for commonly-used re-exports.
//!
//! Currently empty as meta_tools is a facade crate that aggregates meta-programming
//! utilities. Users should import specific features via the main crate root:
//!
//! ```rust
//! use meta_tools::{ for_each, index }; // Recommended
//! ```
//!
//! This namespace exists as a placeholder following the mod_interface pattern
//! established in the crate's architecture. It may be populated in the future with
//! commonly-used macro re-exports if usage patterns indicate a stable set of
//! frequently-imported items.
//!

/// Internal namespace.
mod private
{
}

/// Exposed namespace of the module.
#[ allow( clippy::module_inception ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::private ::
  {
 };
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed :: *;