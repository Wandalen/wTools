//! Typed error handling, a facade for `thiserror`.
//!
//! **Note: ** When using `#[ derive( Error ) ]` or other `thiserror` macros, `thiserror` must be explicitly present in the namespace. This can be achieved by adding `use error_tools ::dependency ::thiserror;` or `use thiserror;` in your module, depending on your project's setup.
pub use ::thiserror ::Error;
