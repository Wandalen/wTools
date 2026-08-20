//! Typed error handling, a facade for `thiserror`.
//!
//! **Note: ** `thiserror` 2 derive macros expand to absolute `::thiserror` paths, so every crate using `#[ derive( Error ) ]` needs `thiserror` as a direct dependency in its `Cargo.toml`. A `use error_tools ::dependency ::thiserror;` alias is no longer sufficient.
pub use ::thiserror ::Error;
