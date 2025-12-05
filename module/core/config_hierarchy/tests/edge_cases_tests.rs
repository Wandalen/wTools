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
