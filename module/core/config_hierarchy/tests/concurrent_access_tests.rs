#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;
use std::{ thread, sync::Arc };

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
fn test_concurrent_writes_with_locking()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = Arc::new( temp_dir.path().join( "config.yaml" ) );

  let mut handles = vec![];

  for i in 0..10
  {
    let path = Arc::clone( &config_path );
    let handle = thread::spawn( move ||
    {
      use config_hierarchy::atomic_config_modify;

      atomic_config_modify( &path, | config |
      {
        let key = format!( "key{i}" );
        let value = JsonValue::Number( i.into() );
        config.insert( key, value );
        Ok( () )
      } ).unwrap();
    } );
    handles.push( handle );
  }

  for handle in handles
  {
    handle.join().unwrap();
  }

  let loaded = TestConfig::load_config_file( &config_path ).unwrap();
  assert_eq!( loaded.len(), 10 );

  for i in 0..10
  {
    let key = format!( "key{i}" );
    assert_eq!( loaded.get( &key ), Some( &JsonValue::Number( i.into() ) ) );
  }
}

#[ test ]
fn test_concurrent_reads()
{
  let temp_dir = TempDir::new().unwrap();
  let config_path = Arc::new( temp_dir.path().join( "config.yaml" ) );

  let mut config = HashMap::new();
  config.insert( "shared_key".into(), JsonValue::String( "shared_value".into() ) );
  TestConfig::save_config_file( &config, &config_path ).unwrap();

  let mut handles = vec![];

  for _ in 0..10
  {
    let path = Arc::clone( &config_path );
    let handle = thread::spawn( move ||
    {
      let loaded = TestConfig::load_config_file( &path ).unwrap();
      assert_eq!( loaded.get( "shared_key" ), Some( &JsonValue::String( "shared_value".into() ) ) );
    } );
    handles.push( handle );
  }

  for handle in handles
  {
    handle.join().unwrap();
  }
}
