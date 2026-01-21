//! Display formatter tests
//!
//! Tests for table, JSON, and YAML formatters

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
use std::collections::HashMap;

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
use serde_json::Value as JsonValue;

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
use config_hierarchy::
{
  ConfigDefaults, ConfigPaths, ConfigValidator, ConfigManager,
  ValidationError, ConfigSource,
};

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
struct TestDefaults;

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut defaults = HashMap::new();
    defaults.insert( "timeout".to_string(), JsonValue::Number( 30.into() ) );
    defaults.insert( "retries".to_string(), JsonValue::Number( 3.into() ) );
    defaults.insert( "debug".to_string(), JsonValue::Bool( false ) );
    defaults.insert( "name".to_string(), JsonValue::String( "app".to_string() ) );
    defaults
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "retries", "debug", "name" ]
  }
}

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
struct TestPaths;

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
impl ConfigPaths for TestPaths
{
  fn app_name() -> &'static str { "testapp" }
}

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
struct TestValidator;

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
impl ConfigValidator for TestValidator
{
  fn validate_parameter( param_name : &str, value : &JsonValue ) -> Result< (), ValidationError >
  {
    if param_name == "timeout"
    {
      if let Some( n ) = value.as_i64()
      {
        if !( 1..=300 ).contains( &n )
        {
          return Err( ValidationError::new( param_name, "timeout must be 1-300" ) );
        }
      }
    }
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError >
  {
    Vec::new()
  }
}

#[ cfg( any( feature = "display_table", feature = "display_json", feature = "display_yaml" ) ) ]
type TestConfig = ConfigManager< TestDefaults, TestPaths, TestValidator >;

#[ test ]
#[ cfg( feature = "display_table" ) ]
fn test_table_format_basic()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();
  let options = DisplayOptions::default();

  let output = TestConfig::format_config_table( &config, &errors, &options );

  // Check header
  assert!( output.contains( "Parameter" ) );
  assert!( output.contains( "Value" ) );
  assert!( output.contains( "Default" ) );
  assert!( output.contains( "Source" ) );

  // Check parameters present
  assert!( output.contains( "timeout" ) );
  assert!( output.contains( "retries" ) );
  assert!( output.contains( "debug" ) );
  assert!( output.contains( "name" ) );

  // Check sources section
  assert!( output.contains( "Configuration sources:" ) );
  assert!( output.contains( "cli" ) );
  assert!( output.contains( "built-in" ) );
}

#[ test ]
#[ cfg( feature = "display_table" ) ]
fn test_table_format_with_warnings()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );

  // Create validation errors
  let errors = vec![
    ValidationError::new( "timeout", "invalid value" ),
    ValidationError::new( "retries", "out of range" ),
  ];

  let options = DisplayOptions::default();
  let output = TestConfig::format_config_table( &config, &errors, &options );

  // Check warnings displayed
  assert!( output.contains( "⚠️  Configuration warnings:" ) );
  assert!( output.contains( "timeout: invalid value" ) );
  assert!( output.contains( "retries: out of range" ) );
}

#[ test ]
#[ cfg( feature = "display_table" ) ]
fn test_table_format_filtered()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();

  let options = DisplayOptions
  {
    filter_key : Some( "timeout".to_string() ),
    include_sources : false,
    include_warnings : false,
  };

  let output = TestConfig::format_config_table( &config, &errors, &options );

  // Should only show filtered key
  assert!( output.contains( "timeout" ) );
  assert!( output.contains( "30" ) );
  assert!( !output.contains( "retries" ) );
  assert!( !output.contains( "Configuration sources:" ) );
}

#[ test ]
#[ cfg( feature = "display_json" ) ]
fn test_json_format_basic()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();
  let options = DisplayOptions::default();

  let output = TestConfig::format_config_json( &config, &errors, &options );

  // Parse JSON to verify structure
  let parsed : serde_json::Value = serde_json::from_str( &output ).expect( "Invalid JSON" );

  assert!( parsed.is_object() );
  assert!( parsed[ "parameters" ].is_object() );
  assert!( parsed[ "parameters" ][ "timeout" ].is_object() );
  assert!( parsed[ "parameters" ][ "timeout" ][ "value" ] == JsonValue::Number( 30.into() ) );
  assert!( parsed[ "parameters" ][ "timeout" ][ "source" ].is_object() );
}

#[ test ]
#[ cfg( feature = "display_json" ) ]
fn test_json_format_with_warnings()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );

  let errors = vec![
    ValidationError::new( "timeout", "invalid value" ),
  ];

  let options = DisplayOptions::default();
  let output = TestConfig::format_config_json( &config, &errors, &options );

  let parsed : serde_json::Value = serde_json::from_str( &output ).expect( "Invalid JSON" );

  assert!( parsed[ "warnings" ].is_array() );
  assert_eq!( parsed[ "warnings" ].as_array().unwrap().len(), 1 );
  assert_eq!( parsed[ "warnings" ][ 0 ][ "parameter" ], "timeout" );
  assert_eq!( parsed[ "warnings" ][ 0 ][ "message" ], "invalid value" );
}

#[ test ]
#[ cfg( feature = "display_json" ) ]
fn test_json_format_filtered()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();

  let options = DisplayOptions
  {
    filter_key : Some( "timeout".to_string() ),
    include_sources : false,
    include_warnings : false,
  };

  let output = TestConfig::format_config_json( &config, &errors, &options );

  let parsed : serde_json::Value = serde_json::from_str( &output ).expect( "Invalid JSON" );

  assert!( parsed[ "value" ] == JsonValue::Number( 30.into() ) );
  assert!( parsed[ "source" ].is_string() );
}

#[ test ]
#[ cfg( feature = "display_yaml" ) ]
fn test_yaml_format_basic()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();
  let options = DisplayOptions::default();

  let output = TestConfig::format_config_yaml( &config, &errors, &options );

  // Check YAML structure
  assert!( output.contains( "configuration:" ) );
  assert!( output.contains( "timeout:" ) );
  assert!( output.contains( "value: 30" ) );
  assert!( output.contains( "type: default" ) );
  assert!( output.contains( "source: default" ) );
}

#[ test ]
#[ cfg( feature = "display_yaml" ) ]
fn test_yaml_format_with_warnings()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );

  let errors = vec![
    ValidationError::new( "timeout", "invalid value" ),
  ];

  let options = DisplayOptions::default();
  let output = TestConfig::format_config_yaml( &config, &errors, &options );

  assert!( output.contains( "warnings:" ) );
  assert!( output.contains( "parameter: timeout" ) );
  assert!( output.contains( "message: invalid value" ) );
}

#[ test ]
#[ cfg( feature = "display_yaml" ) ]
fn test_yaml_format_string_quoting()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();
  let options = DisplayOptions::default();

  let output = TestConfig::format_config_yaml( &config, &errors, &options );

  // String values should be quoted
  assert!( output.contains( "\"app\"" ) );
  // Numbers should not be quoted
  assert!( output.contains( "value: 30" ) );
  assert!( !output.contains( "\"30\"" ) );
  // Booleans should not be quoted
  assert!( output.contains( "value: false" ) );
  assert!( !output.contains( "\"false\"" ) );
}

#[ test ]
#[ cfg( feature = "display_yaml" ) ]
fn test_yaml_format_filtered()
{
  use config_hierarchy::display::DisplayOptions;

  let runtime_params = HashMap::new();
  let config = TestConfig::resolve_all_config( &runtime_params );
  let errors = Vec::new();

  let options = DisplayOptions
  {
    filter_key : Some( "name".to_string() ),
    include_sources : false,
    include_warnings : false,
  };

  let output = TestConfig::format_config_yaml( &config, &errors, &options );

  assert!( output.contains( "name:" ) );
  assert!( output.contains( "\"app\"" ) );
  assert!( !output.contains( "timeout:" ) );
  assert!( !output.contains( "configuration:" ) );
}

#[ test ]
#[ cfg( feature = "display_table" ) ]
fn test_table_tree_fmt_reexports()
{
  // Verify tree_fmt types are reexported for customization
  use config_hierarchy::display::table::{ TreeRowBuilder, TreeTableFormatter, TreeTableConfig };

  let builder = TreeRowBuilder::new( vec![ "Col1".into(), "Col2".into() ] );
  let builder = builder.add_row( vec![ "val1".into(), "val2".into() ] );
  let tree = builder.build();

  let formatter = TreeTableFormatter::with_config( TreeTableConfig::default() );
  let output = formatter.format( &tree );

  assert!( output.contains( "Col1" ) );
  assert!( output.contains( "val1" ) );
}
