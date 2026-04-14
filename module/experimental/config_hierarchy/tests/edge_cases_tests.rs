#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;

struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    HashMap::new()
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![]
  }
}

struct TestPaths;
impl ConfigPaths for TestPaths
{
  fn app_name() -> &'static str { "testapp" }
}

struct TestValidator;
impl ConfigValidator for TestValidator
{
  fn validate_parameter( _param : &str, _value : &JsonValue ) -> Result< (), ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError >
  {
    Vec::new()
  }
}

type TestConfig = ConfigManager< TestDefaults, TestPaths, TestValidator >;

#[ test ]
fn test_unicode_parameter_names()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "测试".into(), JsonValue::String( "chinese".into() ) );
  config.insert( "тест".into(), JsonValue::String( "cyrillic".into() ) );
  config.insert( "🔥".into(), JsonValue::String( "emoji".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "测试" ), Some( &JsonValue::String( "chinese".into() ) ) );
  assert_eq!( loaded.get( "тест" ), Some( &JsonValue::String( "cyrillic".into() ) ) );
  assert_eq!( loaded.get( "🔥" ), Some( &JsonValue::String( "emoji".into() ) ) );
}

#[ test ]
fn test_unicode_parameter_values()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "key1".into(), JsonValue::String( "测试值".into() ) );
  config.insert( "key2".into(), JsonValue::String( "Привет мир".into() ) );
  config.insert( "key3".into(), JsonValue::String( "🚀🔥🎉".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key1" ), Some( &JsonValue::String( "测试值".into() ) ) );
  assert_eq!( loaded.get( "key2" ), Some( &JsonValue::String( "Привет мир".into() ) ) );
  assert_eq!( loaded.get( "key3" ), Some( &JsonValue::String( "🚀🔥🎉".into() ) ) );
}

#[ test ]
fn test_very_long_parameter_names()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let long_name = "a".repeat( 1000 );
  let mut config = HashMap::new();
  config.insert( long_name.clone(), JsonValue::String( "value".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( &long_name ), Some( &JsonValue::String( "value".into() ) ) );
}

#[ test ]
fn test_very_long_parameter_values()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let long_value = "x".repeat( 10000 );
  let mut config = HashMap::new();
  config.insert( "key".into(), JsonValue::String( long_value.clone() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "key" ), Some( &JsonValue::String( long_value ) ) );
}

#[ test ]
fn test_special_yaml_characters()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "colon:key".into(), JsonValue::String( "value:with:colons".into() ) );
  config.insert( "quotes\"key".into(), JsonValue::String( "value\"with\"quotes".into() ) );
  config.insert( "brackets[key]".into(), JsonValue::String( "value[with]brackets".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "colon:key" ), Some( &JsonValue::String( "value:with:colons".into() ) ) );
  assert_eq!( loaded.get( "quotes\"key" ), Some( &JsonValue::String( "value\"with\"quotes".into() ) ) );
  assert_eq!( loaded.get( "brackets[key]" ), Some( &JsonValue::String( "value[with]brackets".into() ) ) );
}

#[ test ]
fn test_newlines_in_values()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "multiline".into(), JsonValue::String( "line1\nline2\nline3".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "multiline" ), Some( &JsonValue::String( "line1\nline2\nline3".into() ) ) );
}

#[ test ]
fn test_corrupted_yaml_returns_error()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "corrupted.yaml" );

  std::fs::write( &config_path, "invalid: yaml: [unclosed" ).unwrap();

  let result = TestConfig::load_config_file( &config_path );
  assert!( result.is_err() );
}

#[ test ]
fn test_many_parameters()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  for i in 0..100
  {
    config.insert( format!( "key{i}" ), JsonValue::Number( i.into() ) );
  }

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.len(), 100 );

  for i in 0..100
  {
    assert_eq!( loaded.get( &format!( "key{i}" ) ), Some( &JsonValue::Number( i.into() ) ) );
  }
}

#[ test ]
fn test_parameter_name_with_dots()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "module.setting.value".into(), JsonValue::String( "dotted".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "module.setting.value" ), Some( &JsonValue::String( "dotted".into() ) ) );
}

#[ test ]
fn test_parameter_name_with_underscores()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "some_long_parameter_name".into(), JsonValue::String( "underscored".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "some_long_parameter_name" ), Some( &JsonValue::String( "underscored".into() ) ) );
}

#[ test ]
fn test_parameter_name_with_hyphens()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "some-hyphenated-param".into(), JsonValue::String( "hyphenated".into() ) );

  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.get( "some-hyphenated-param" ), Some( &JsonValue::String( "hyphenated".into() ) ) );
}

// Bug reproducer: Empty app_name creates invalid paths
//
// ## Root Cause
// ConfigPaths::app_name() can return empty string, which creates paths like
// `/path/./config.yaml` instead of `/path/.appname/config.yaml`. The empty
// string concatenates with prefix (`.` or `-`) to form `./` or `-/`, which
// are valid directory references but not subdirectories.
//
// ## Why Not Caught
// No validation exists for app_name() return value. It's a user-provided trait
// implementation with no constraints.
//
// ## Fix Applied
// Added validation in path construction functions to detect empty app_name
// and return error. Users must provide non-empty application name.
//
// ## Prevention
// Add documentation requirement that app_name() must return non-empty string.
// Consider adding validation helper or compile-time constraint in future.
//
// ## Pitfall
// Empty strings in path construction create valid but semantically wrong paths.
// Always validate user inputs even from trait implementations.
#[ test ]
#[ should_panic( expected = "app_name must not be empty" ) ]
fn test_empty_app_name_rejected()
{
  struct EmptyAppName;
  impl ConfigDefaults for EmptyAppName
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  impl ConfigPaths for EmptyAppName
  {
    fn app_name() -> &'static str { "" }  // EMPTY - should be rejected
  }

  impl ConfigValidator for EmptyAppName
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError > { Vec::new() }
  }

  type EmptyConfig = ConfigManager< EmptyAppName, EmptyAppName, EmptyAppName >;

  // Should panic when trying to get path with empty app_name
  let _path = EmptyConfig::get_local_config_path().unwrap();
}

// Bug reproducer: Path traversal via app_name
//
// ## Root Cause
// ConfigPaths::app_name() is used directly in path construction without
// sanitization. User can include `../` to escape intended config directory,
// potentially accessing files outside application's config scope.
//
// ## Why Not Caught
// No path sanitization or validation of app_name content. Assumed trait
// implementation would provide safe values.
//
// ## Fix Applied
// Added validation to reject app_name containing path separators (`/`, `\`)
// or parent directory references (`..`). Only alphanumeric, hyphens, underscores,
// and dots (not `..`) are allowed.
//
// ## Prevention
// Document security requirements for ConfigPaths trait implementations.
// Add validation helpers for common security checks.
//
// ## Pitfall
// User-provided strings in filesystem paths require sanitization even when
// from trait implementations. Never trust input for path construction.
#[ test ]
#[ should_panic( expected = "app_name contains invalid characters" ) ]
fn test_path_traversal_rejected()
{
  struct PathTraversalAttack;
  impl ConfigDefaults for PathTraversalAttack
  {
    fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
    fn get_parameter_names() -> Vec< &'static str > { vec![] }
  }

  impl ConfigPaths for PathTraversalAttack
  {
    fn app_name() -> &'static str { "../../etc/passwd" }  // PATH TRAVERSAL ATTACK
  }

  impl ConfigValidator for PathTraversalAttack
  {
    fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
    fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError > { Vec::new() }
  }

  type AttackConfig = ConfigManager< PathTraversalAttack, PathTraversalAttack, PathTraversalAttack >;

  // Should panic when detecting path traversal attempt
  let _path = AttackConfig::get_local_config_path().unwrap();
}
