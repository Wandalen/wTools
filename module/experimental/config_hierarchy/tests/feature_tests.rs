//! Feature flag validation tests
//!
//! Ensures different feature combinations compile and work correctly

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use config_hierarchy::{ ConfigDefaults, ConfigPaths, ConfigValidator, ConfigManager, ValidationError, ConfigSource };

// Test implementations
struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut defaults = HashMap::new();
    defaults.insert( "timeout".to_string(), JsonValue::Number( 30.into() ) );
    defaults.insert( "debug".to_string(), JsonValue::Bool( false ) );
    defaults
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "debug" ]
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
  fn validate_parameter( _param_name : &str, _value : &JsonValue ) -> Result< (), ValidationError >
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
fn test_core_features_available()
{
  // Core features (always available)
  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  assert!( config.contains_key( "timeout" ) );
  assert!( config.contains_key( "debug" ) );

  // Validate works
  let errors = TestConfig::validate_all_config( &config );
  assert!( errors.is_empty() );
}

#[ test ]
#[ cfg( feature = "file_ops" ) ]
fn test_file_ops_feature_available()
{
  use tempfile::tempdir;

  let temp_dir = tempdir().expect( "Failed to create temp dir" );
  let config_path = temp_dir.path().join( "config.yaml" );

  let mut config = HashMap::new();
  config.insert( "timeout".to_string(), JsonValue::Number( 60.into() ) );

  // Save/load should work
  TestConfig::save_config_file( &config, &config_path ).expect( "Failed to save" );
  assert!( config_path.exists() );

  let loaded = TestConfig::load_config_file( &config_path ).expect( "Failed to load" );
  assert_eq!( loaded.get( "timeout" ), Some( &JsonValue::Number( 60.into() ) ) );

  // Delete should work
  let deleted = TestConfig::delete_config_file( &config_path ).expect( "Failed to delete" );
  assert!( deleted );
  assert!( !config_path.exists() );
}

#[ test ]
#[ cfg( feature = "display_table" ) ]
fn test_display_table_feature_available()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = TestConfig::validate_all_config( &config );
  let options = DisplayOptions::default();

  let table = TestConfig::format_config_table( &config, &errors, &options );
  assert!( table.contains( "Parameter" ) );
  assert!( table.contains( "timeout" ) );
  assert!( table.contains( "debug" ) );
}

#[ test ]
#[ cfg( feature = "display_json" ) ]
fn test_display_json_feature_available()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = TestConfig::validate_all_config( &config );
  let options = DisplayOptions::default();

  let json = TestConfig::format_config_json( &config, &errors, &options );
  assert!( json.contains( "parameters" ) );
  assert!( json.contains( "timeout" ) );
}

#[ test ]
#[ cfg( feature = "display_yaml" ) ]
fn test_display_yaml_feature_available()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = TestConfig::validate_all_config( &config );
  let options = DisplayOptions::default();

  let yaml = TestConfig::format_config_yaml( &config, &errors, &options );
  assert!( yaml.contains( "configuration:" ) );
  assert!( yaml.contains( "timeout" ) );
}

#[ test ]
#[ cfg( feature = "display" ) ]
fn test_display_options_struct()
{
  use config_hierarchy::display::DisplayOptions;

  let options = DisplayOptions
  {
    filter_key : Some( "timeout".to_string() ),
    include_sources : false,
    include_warnings : false,
  };

  assert_eq!( options.filter_key, Some( "timeout".to_string() ) );
  assert!( !options.include_sources );
  assert!( !options.include_warnings );

  let default_options = DisplayOptions::default();
  assert_eq!( default_options.filter_key, None );
  assert!( default_options.include_sources );
  assert!( default_options.include_warnings );
}
