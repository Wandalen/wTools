//! Path naming standards validation tests
//!
//! Verifies that all paths follow the `.{utility_name}` convention

use config_hierarchy::{ ConfigPaths, ConfigManager, ConfigDefaults, ConfigValidator };
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use serial_test::serial;

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
  assert!( path_str.contains( "/.testapp/" ), "Global path must use .testapp (got: {path_str})" );
  assert!( !path_str.contains( "/testapp/" ), "Global path must not use testapp without dot (got: {path_str})" );

  // Should be in .persistent/.testapp
  assert!( path_str.contains( ".persistent/.testapp" ), "Global path must be in .persistent/.testapp (got: {path_str})" );

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
  assert!( dir_str.ends_with( ".persistent/.testapp" ), "Global dir must be .persistent/.testapp (got: {dir_str})" );

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
  let _old_dir = std::env::current_dir().expect( "Should get current dir" );
  std::env::set_current_dir( temp_dir.path() ).expect( "Should change to temp dir" );

  // Discover configs
  let configs = config_hierarchy::discover_local_configs::< TestApp >();

  // Should find the config
  assert!( !configs.is_empty(), "Should discover local config" );

  // All discovered paths should use .testapp
  for path in &configs
  {
    let path_str = path.to_string_lossy();
    assert!( path_str.contains( "/.testapp/" ), "Local path must use .testapp (got: {path_str})" );
    assert!( !path_str.contains( "/testapp/" ), "Local path must not use testapp without dot (got: {path_str})" );
  }
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
  assert!( global_path_str.contains( "/.testapp/" ), "Global must use .testapp" );

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
  assert!( path_str.contains( "/.anotherapp/" ), "Should derive .anotherapp from app_name" );

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
  assert!( path_str.contains( "/.testapp/" ), "ConfigManager must use .testapp (got: {path_str})" );
  assert!( path_str.contains( ".persistent/.testapp" ), "ConfigManager must use .persistent/.testapp" );

  std::env::remove_var( "PRO" );
}
