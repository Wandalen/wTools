//!
//! Command aggregator library for advanced command parsing and execution.
//!

/// Contains the parsing components for the command aggregator.
pub mod parsing;

mod private {}

crate::mod_interface!
{
  /// Exposes the parsing module.
  exposed use parsing;
}
