//!
//! Commands aggregator library.
//!

pub mod parsing;

mod private {}

crate::mod_interface!
{
  exposed use parsing;
}
