#![ allow( missing_docs ) ]

use config_hierarchy::*;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tempfile::TempDir;
use std::env;

struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "param1".into(), JsonValue::String( "default".into() ) );
    map.insert( "param2".into(), JsonValue::Number( 100.into() ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "param1", "param2" ]
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
fn test_default_source()
{
  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "default".into() ) );
  assert!( matches!( source, ConfigSource::Default ) );
}

#[ test ]
fn test_runtime_overrides_default()
{
  let mut runtime_params = HashMap::new();
  runtime_params.insert( "param1".into(), "runtime_value".into() );

  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "runtime_value".into() ) );
  assert!( matches!( source, ConfigSource::Runtime ) );
}

#[ test ]
fn test_env_overrides_default()
{
  env::set_var( "TESTAPP_ENVPARAM", "env_value" );

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "envparam", &runtime_params );

  assert_eq!( value, JsonValue::String( "env_value".into() ) );
  assert!( matches!( source, ConfigSource::Environment ) );

  env::remove_var( "TESTAPP_ENVPARAM" );
}

#[ test ]
fn test_runtime_overrides_env()
{
  env::set_var( "TESTAPP_TESTPARAM", "env_value" );

  let mut runtime_params = HashMap::new();
  runtime_params.insert( "testparam".into(), "runtime_value".into() );

  let ( value, source ) = TestConfig::resolve_config_value( "testparam", &runtime_params );

  assert_eq!( value, JsonValue::String( "runtime_value".into() ) );
  assert!( matches!( source, ConfigSource::Runtime ) );

  env::remove_var( "TESTAPP_TESTPARAM" );
}

#[ test ]
fn test_global_config_overrides_default()
{
  let temp_dir = TempDir::new().unwrap();
  let original_pro = env::var( "PRO" );

  env::set_var( "PRO", temp_dir.path() );

  let mut config = HashMap::new();
  config.insert( "param1".into(), JsonValue::String( "global_value".into() ) );

  TestConfig::save_global_config( &config ).unwrap();

  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "param1", &runtime_params );

  assert_eq!( value, JsonValue::String( "global_value".into() ) );
  assert!( matches!( source, ConfigSource::Global( _ ) ) );

  if let Ok( original ) = original_pro
  {
    env::set_var( "PRO", original );
  }
  else
  {
    env::remove_var( "PRO" );
  }
}

#[ test ]
fn test_resolve_all_config()
{
  let runtime_params = HashMap::new();
  let all_config = TestConfig::resolve_all_config( &runtime_params );

  assert!( all_config.contains_key( "param1" ) );
  assert!( all_config.contains_key( "param2" ) );

  let ( value1, source1 ) = &all_config[ "param1" ];
  assert_eq!( value1, &JsonValue::String( "default".into() ) );
  assert!( matches!( source1, ConfigSource::Default ) );
}

#[ test ]
fn test_unknown_parameter_returns_null()
{
  let runtime_params = HashMap::new();
  let ( value, source ) = TestConfig::resolve_config_value( "unknown_param", &runtime_params );

  assert_eq!( value, JsonValue::Null );
  assert!( matches!( source, ConfigSource::Default ) );
}

#[ test ]
fn test_case_sensitive_env_var()
{
  env::set_var( "TESTAPP_PARAM_WITH_CASE", "value" );

  let runtime_params = HashMap::new();
  let ( value, _ ) = TestConfig::resolve_config_value( "param_with_case", &runtime_params );

  assert_eq!( value, JsonValue::String( "value".into() ) );

  env::remove_var( "TESTAPP_PARAM_WITH_CASE" );
}

#[ test ]
fn test_type_detection_in_hierarchy()
{
  env::set_var( "TESTAPP_BOOL_PARAM", "true" );

  let runtime_params = HashMap::new();
  let ( value, _ ) = TestConfig::resolve_config_value( "bool_param", &runtime_params );

  assert_eq!( value, JsonValue::Bool( true ) );

  env::remove_var( "TESTAPP_BOOL_PARAM" );
}
