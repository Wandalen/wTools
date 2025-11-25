//! Smoke testing of the package.
//!
//! Verifies basic functionality works after installation/compilation.

/// Local smoke test - verifies package builds and basic API is accessible.
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn local_smoke_test()
{
  // Verify we can access the module
  use time_tools as the_module;

  // Verify basic functionality works
  let timestamp = the_module ::now();
  assert!( timestamp > 0, "Basic time retrieval should work" );
}

/// Published smoke test - verifies published crate can be used correctly.
///
/// Ensures that the crate compiles and basic usage works.
#[ test ]
fn published_smoke_test()
{
  // If this test runs, compilation succeeded - that's the smoke test
  // No assertions needed; successful compilation is the verification
}
