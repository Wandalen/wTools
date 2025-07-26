//! Untyped error handling, a facade for `anyhow`.
#![allow(clippy::wildcard_imports)]
pub use ::anyhow::{anyhow, bail, ensure, format_err, Context, Error, Ok, Result};
