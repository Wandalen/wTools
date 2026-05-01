//! Smoke testing of the package.
//!
//! Verifies basic crate identity and importability.
//! Note: This is a placeholder crate with no functionality implemented yet.

// test_kind: smoke
#[ test ]
fn local_smoke_test()
{
  // Verify the crate has the expected package identity.
  // Fails if the crate is renamed without updating this test.
  assert_eq!( env!( "CARGO_PKG_NAME" ), "include_md" );
}

// test_kind: smoke
#[ test ]
fn published_smoke_test()
{
  // Verify the crate can be imported as an external dependency and
  // that its declared package name matches the expected value.
  // Fails if the crate identity or import path is broken.
  #[ allow( unused_imports ) ]
  use include_md;
  assert_eq!( env!( "CARGO_PKG_NAME" ), "include_md" );
}
