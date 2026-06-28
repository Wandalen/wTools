//! Path naming standards validation tests
//!
//! Verifies that all paths follow the `.{utility_name}` convention

use config_hierarchy::{ ConfigPaths, ConfigManager, ConfigDefaults, ConfigValidator };
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use serial_test::serial;

/// Check if path contains pattern with either Unix or Windows separator
fn contains_with_separator( path : &str, pattern : &str ) -> bool
{
  path.contains( pattern ) ||
  path.contains( &pattern.replace( '/', "\\" ) )
}

/// Check if path ends with pattern using either Unix or Windows separator
fn ends_with_separator( path : &str, pattern : &str ) -> bool
{
  path.ends_with( pattern ) ||
  path.ends_with( &pattern.replace( '/', "\\" ) )
}

struct TestApp;

impl ConfigPaths for TestApp
{
  fn app_name() -> &'static str { "testapp" }
}

struct TestDefaults;

impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut defaults = HashMap::new();
    defaults.insert( "param1".into(), JsonValue::String( "value1".into() ) );
    defaults
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "param1" ]
  }
}

struct TestValidator;

impl ConfigValidator for TestValidator
{
  fn validate_parameter( _param_name : &str, _value : &JsonValue )
    -> Result< (), config_hierarchy::ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, config_hierarchy::ConfigSource ) > )
    -> Vec< config_hierarchy::ValidationError >
  {
    Vec::new()
  }
}

type TestConfig = ConfigManager< TestDefaults, TestApp, TestValidator >;

#[ test ]
#[ serial ]
fn test_global_path_has_dot_prefix()
{
  std::env::set_var( "PRO", "/tmp/test_pro_path_standards" );

  let path = config_hierarchy::get_global_config_path::< TestApp >()
    .expect( "Should get global config path" );

  // Path should be /tmp/test_pro_path_standards/.persistent/.testapp/config.yaml
  let path_str = path.to_string_lossy();

  // Should contain .testapp not testapp
  assert!( contains_with_separator( &path_str, "/.testapp/" ), "Global path must use .testapp (got: {path_str})" );
  assert!( !contains_with_separator( &path_str, "/testapp/" ), "Global path must not use testapp without dot (got: {path_str})" );

  // Should be in .persistent/.testapp
  assert!( contains_with_separator( &path_str, ".persistent/.testapp" ), "Global path must be in .persistent/.testapp (got: {path_str})" );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_global_dir_structure()
{
  std::env::set_var( "PRO", "/tmp/test_pro_path_standards" );

  let dir = config_hierarchy::get_global_config_dir::< TestApp >()
    .expect( "Should get global config dir" );

  let dir_str = dir.to_string_lossy();

  // Should end with .testapp
  assert!( dir_str.ends_with( ".testapp" ), "Global dir must end with .testapp (got: {dir_str})" );

  // Should be .persistent/.testapp
  assert!( ends_with_separator( &dir_str, ".persistent/.testapp" ), "Global dir must be .persistent/.testapp (got: {dir_str})" );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_local_configs_use_dot_prefix()
{
  use std::fs;
  use tempfile::TempDir;

  // Create temp directory structure
  let temp_dir = TempDir::new().expect( "Should create temp dir" );
  let config_dir = temp_dir.path().join( ".testapp" );
  fs::create_dir( &config_dir ).expect( "Should create .testapp dir" );

  let config_path = config_dir.join( "config.yaml" );
  fs::write( &config_path, "metadata:\n  version: '1.0'\nparameters: {}\n" )
    .expect( "Should write config" );

  // Change to temp dir
  std::env::set_current_dir( temp_dir.path() ).expect( "Should change to temp dir" );

  // Discover configs
  let configs = config_hierarchy::discover_local_configs::< TestApp >();

  // Should find the config
  assert!( !configs.is_empty(), "Should discover local config" );

  // All discovered paths should use .testapp
  for path in &configs
  {
    let path_str = path.to_string_lossy();
    assert!( contains_with_separator( &path_str, "/.testapp/" ), "Local path must use .testapp (got: {path_str})" );
    assert!( !contains_with_separator( &path_str, "/testapp/" ), "Local path must not use testapp without dot (got: {path_str})" );
  }

  // Restore cwd before TempDir drops — otherwise cwd points to a deleted directory
  // and any concurrent test calling current_dir() gets "No such file or directory".
  std::env::set_current_dir( "/tmp" ).expect( "Should restore cwd" );
}

#[ test ]
#[ serial ]
fn test_derived_paths_consistency()
{
  std::env::set_var( "PRO", "/tmp/test_pro_path_standards" );

  let global_path = config_hierarchy::get_global_config_path::< TestApp >()
    .expect( "Should get global config path" );

  let global_path_str = global_path.to_string_lossy();

  // Both should use .testapp
  assert!( contains_with_separator( &global_path_str, "/.testapp/" ), "Global must use .testapp" );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_no_flexibility_all_paths_derived()
{
  // Verify that only app_name() is needed
  struct AnotherApp;

  impl ConfigPaths for AnotherApp
  {
    fn app_name() -> &'static str { "anotherapp" }
  }

  std::env::set_var( "PRO", "/tmp/test_pro_another" );

  let path = config_hierarchy::get_global_config_path::< AnotherApp >()
    .expect( "Should get path" );

  let path_str = path.to_string_lossy();

  // Should automatically use .anotherapp
  assert!( contains_with_separator( &path_str, "/.anotherapp/" ), "Should derive .anotherapp from app_name" );

  std::env::remove_var( "PRO" );
}

#[ test ]
#[ serial ]
fn test_config_manager_uses_standard_paths()
{
  std::env::set_var( "PRO", "/tmp/test_pro_manager" );

  let path = TestConfig::get_global_config_path()
    .expect( "Should get path through ConfigManager" );

  let path_str = path.to_string_lossy();

  // Should use .testapp
  assert!( contains_with_separator( &path_str, "/.testapp/" ), "ConfigManager must use .testapp (got: {path_str})" );
  assert!( contains_with_separator( &path_str, ".persistent/.testapp" ), "ConfigManager must use .persistent/.testapp" );

  std::env::remove_var( "PRO" );
}

// AN-03: Backslash in app_name is rejected (Windows path separator)
//
// ## Root Cause
// validate_app_name checks for '/' and '\' independently. A name like "my\app"
// would create ".my\app/config.yaml" on Linux (valid filename!) but escape the
// intended directory on Windows. Rejected on all platforms for portability.
//
// ## Fix Applied
// validate_app_name returns Err when app_name.contains('\\') — same as '/' check.
//
// ## Pitfall
// On Linux, '\' is not a path separator, so the OS would accept "my\app" as a
// directory name. Validation must be stricter than the OS to ensure portability.
#[ test ]
#[ should_panic( expected = "app_name contains invalid characters" ) ]
fn test_backslash_in_app_name_rejected()
{
  struct BackslashAppName;
  impl ConfigDefaults for BackslashAppName
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  impl ConfigPaths for BackslashAppName
  {
    fn app_name() -> &'static str { "my\\app" }  // BACKSLASH — should be rejected
  }

  impl ConfigValidator for BackslashAppName
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), config_hierarchy::ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, config_hierarchy::ConfigSource ) > ) -> Vec< config_hierarchy::ValidationError > { Vec::new() }
  }

  type BackslashConfig = ConfigManager< BackslashAppName, BackslashAppName, BackslashAppName >;
  let _path = BackslashConfig::get_local_config_path().unwrap();
}

// AN-06: Valid app_name with hyphens and underscores is accepted
//
// ## Root Cause
// validate_app_name rejects '/', '\', and '..'. Hyphens and underscores are common
// in application names (e.g., "my-app_v2") and must NOT be rejected.
//
// ## Fix Applied
// No code change needed — validation already allows hyphens and underscores.
// This test verifies the constraint is appropriately narrow (not over-blocking).
//
// ## Pitfall
// Do not widen the rejection set beyond the documented forbidden characters.
// Over-validation rejects valid names and breaks legitimate use cases.
#[ test ]
#[ serial ]
fn test_valid_hyphen_underscore_name_accepted()
{
  struct HyphenUnderscoreApp;
  impl ConfigPaths for HyphenUnderscoreApp
  {
    fn app_name() -> &'static str { "my-app_v2" }
  }

  let result = config_hierarchy::get_local_config_path::< HyphenUnderscoreApp >();
  assert!( result.is_ok(), "app_name with hyphens and underscores must be accepted, got: {:?}", result.err() );

  let path_str = result.unwrap().to_string_lossy().to_string();
  assert!( path_str.contains( ".my-app_v2" ), "Path must use dotted app_name, got: {path_str}" );
}
