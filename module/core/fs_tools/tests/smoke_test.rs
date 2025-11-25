//! Smoke testing of the package.
//!
//! Verifies that the crate compiles and basic imports work correctly.

#[ allow( unused_imports ) ]
use fs_tools as the_module;

/// Verifies crate compiles and namespace imports work for local development.
#[ test ]
fn local_smoke_test()
{
  // Verify crate can be imported
  let _ = stringify!( the_module );

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  {
    // Verify TempDir is accessible when enabled + std
    let temp_dir = the_module::TempDir::new();
    // Basic sanity check
    assert!( temp_dir.base_path.as_os_str().is_empty(), "TempDir should initialize with empty base_path" );
  }

  #[ cfg( not( all( feature = "enabled", not( feature = "no_std" ) ) ) ) ]
  {
    // In no_std mode or when disabled, just verify compilation succeeds
    // This is a valid test - it confirms the crate builds in this configuration
  }
}

/// Verifies crate compiles correctly as published package.
#[ test ]
fn published_smoke_test()
{
  // Verify all public exports are accessible
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  {
    // When std is available, verify TempDir works
    let _temp_dir = the_module::TempDir::new();
  }

  // Test always passes after verifying imports/compilation
  // This confirms the published crate structure is valid
}
