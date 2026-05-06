#[ allow( unused_imports ) ]
use super :: *;

// Test file for basic `TempDir` functionality.
//
// ## Test Matrix
//
// | ID   | Aspect Tested        | Expected Behavior                          |
// |------|----------------------|--------------------------------------------|
// | T1.1 | TempDir creation     | `TempDir::new()` returns valid instance    |
// | T1.2 | Path access          | Can access base_path field                 |
// | T1.3 | Default values       | Fields initialize to empty `PathBuf`       |
//
// Note: `TempDir` is available whenever `enabled` feature is set. The lib.rs
// `cfg_attr` ensures the crate stays in std mode when `enabled` is active,
// even if `no_std` feature is also set (e.g. with `--all-features`).

/// Tests that `TempDir` can be created with `new()`.
/// Test Combination: T1.1
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn temp_dir_creation()
{
  let temp_dir = the_module::TempDir::new();
  // Verify the instance was created
  assert!( temp_dir.base_path.as_os_str().is_empty() );
}

/// Tests that `TempDir` fields are accessible.
/// Test Combination: T1.2
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn temp_dir_path_access()
{
  let temp_dir = the_module::TempDir::new();
  let _ = &temp_dir.base_path;
  let _ = &temp_dir.prefix_path;
  let _ = &temp_dir.postfix_path;
  // Test passes if all fields are accessible
}

/// Tests default initialization values.
/// Test Combination: T1.3
#[ test ]
#[ cfg( feature = "enabled" ) ]
fn temp_dir_defaults()
{
  let temp_dir = the_module::TempDir::new();
  assert!( temp_dir.base_path.as_os_str().is_empty() );
  assert!( temp_dir.prefix_path.as_os_str().is_empty() );
  assert!( temp_dir.postfix_path.as_os_str().is_empty() );
}
