//! Smoke testing of the package.
//!
//! Verifies that the proc-macro crate compiles and exports the `mod_interface` macro.
//! This test ensures the crate is functional at the most basic level.

#[ test ]
fn crate_is_importable()
{
  // This test verifies that the crate compiles and the mod_interface macro exists.
  // The import itself is the test - if this compiles, the macro is accessible.
  #[ allow( unused_imports ) ]
  use mod_interface_meta::mod_interface;

  // No runtime assertions needed - compile-time verification is sufficient.
  // A proc-macro crate's primary contract is that it compiles and exports the macro.
}
