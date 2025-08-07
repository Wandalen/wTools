#![cfg_attr(feature = "no_std", no_std)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/error_tools/latest/error_tools/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Error handling tools and utilities for Rust" ) ]
#![allow(clippy::mod_module_files)]

/// Core error handling utilities.
#[cfg(feature = "enabled")]
pub mod error;

/// Namespace with dependencies.
#[cfg(feature = "enabled")]
pub mod dependency {
  #[doc(inline)]
  #[cfg(feature = "error_typed")]
  pub use ::thiserror;
  #[doc(inline)]
  #[cfg(feature = "error_untyped")]
  pub use ::anyhow;
}

/// Prelude to use essentials: `use error_tools::prelude::*`.
#[cfg(feature = "enabled")]
pub mod prelude {
  #[doc(inline)]
  #[allow(unused_imports)]
  pub use super::error::*;
  #[doc(inline)]
  #[cfg(feature = "error_untyped")]
  pub use super::error::untyped::*;
  #[doc(inline)]
  #[cfg(feature = "error_typed")]
  pub use super::error::typed::*;
}

#[doc(inline)]
#[cfg(feature = "enabled")]
pub use prelude::*;
