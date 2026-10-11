//! Smoke testing of the package.
//!
//! Verifies that the crate compiles, its public API has the expected shape,
//! and `ensure_offline_mode_with` degrades safely when no `.sqlx` cache
//! directory exists anywhere.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `public_api_has_the_expected_shape` | Signatures consumers and tests rely on | — | compiles | ✅ |
//! | `missing_cache_dir_degrades_to_a_warning_not_a_panic` | Printing entry point, nothing to check | empty directory | returns normally | ✅ |

#![ cfg( feature = "enabled" ) ]

use std::path::PathBuf;

use sqlx_query::{ build_script_directives, ensure_offline_mode, ensure_offline_mode_with, Options };

/// The simplest variable lookup `Options::resolve` accepts.
type Lookup = fn( &str ) -> Option< String >;

/// Verifies the public API's shape: the zero-config entry point, its
/// explicit-options variant, the dry run, and both ways to build `Options`.
#[ test ]
fn public_api_has_the_expected_shape()
{
  let _ : fn() = ensure_offline_mode;
  let _ : fn( &Options ) = ensure_offline_mode_with;
  let _ : fn( &Options ) -> Vec< String > = build_script_directives;
  let _ : fn() -> Options = Options::from_env;
  let _ : fn( PathBuf, Lookup ) -> Options = Options::resolve;
  let _ = Options::default();
}

/// With no `.sqlx` anywhere and no override, the crate must warn and return
/// without forcing `SQLX_OFFLINE` -- never panic, never bootstrap a cache
/// directory the consumer never asked for.
#[ test ]
fn missing_cache_dir_degrades_to_a_warning_not_a_panic()
{
  let temp = tempfile::tempdir().expect( "tempdir" );
  let opts = Options { manifest_dir : temp.path().to_path_buf(), ..Options::default() };

  ensure_offline_mode_with( &opts );
}
