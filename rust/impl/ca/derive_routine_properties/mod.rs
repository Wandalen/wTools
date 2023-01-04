//!
//! Macro to implement Attributes trait.
//!

/// Internal namespace.
pub( crate ) mod private
{
  pub use derive_routine_properties::Properties;
}

crate::mod_interface!
{
  prelude use Properties;
}
