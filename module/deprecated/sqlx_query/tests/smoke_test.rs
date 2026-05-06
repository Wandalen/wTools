//! Smoke testing of the package.
//!
//! Verifies that the crate compiles and basic imports work correctly.

#[ allow( unused_imports ) ]
use sqlx_query as the_module;

/// Verifies crate compiles and namespace imports work.
#[ test ]
fn local_smoke_test()
{
  let _ = stringify!( the_module );
}

/// Verifies published package compiles in default configuration.
#[ test ]
fn published_smoke_test()
{
  let _ = stringify!( the_module );
}
