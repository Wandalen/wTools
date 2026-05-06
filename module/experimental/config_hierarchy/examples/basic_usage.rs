//! Demonstrates basic `config_hierarchy` usage with the three required traits.
//!
//! Run with:
//! ```bash
//! cargo run --example basic_usage --features "enabled,file_ops"
//! ```

use config_hierarchy::
{
  ConfigDefaults, ConfigPaths, ConfigValidator, ConfigManager,
  ValidationError, ConfigSource,
};
use std::collections::HashMap;
use serde_json::Value as JsonValue;

// 1. Implement ConfigDefaults — application's built-in fallback values

struct AppDefaults;

impl ConfigDefaults for AppDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "timeout".to_string(), JsonValue::Number( 30.into() ) );
    map.insert( "retries".to_string(), JsonValue::Number( 3.into() ) );
    map.insert( "debug".to_string(),   JsonValue::Bool( false ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "retries", "debug" ]
  }
}

// 2. Implement ConfigPaths — only app_name() is required

struct AppPaths;

impl ConfigPaths for AppPaths
{
  fn app_name() -> &'static str { "myapp" }
  // Override env_var_prefix to avoid Box::leak on every call
  fn env_var_prefix() -> &'static str { "MYAPP" }
}

// 3. Implement ConfigValidator — optional; use a no-op impl when not needed

struct AppValidator;

impl ConfigValidator for AppValidator
{
  fn validate_parameter( param_name : &str, value : &JsonValue )
    -> Result< (), ValidationError >
  {
    if param_name == "timeout"
    {
      if let Some( t ) = value.as_i64()
      {
        if !( 1..=300 ).contains( &t )
        {
          return Err( ValidationError::new( param_name, "must be between 1 and 300 seconds" ) );
        }
      }
    }
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > )
    -> Vec< ValidationError >
  {
    Vec::new()
  }
}

// 4. Compose the manager type

type AppConfig = ConfigManager< AppDefaults, AppPaths, AppValidator >;

fn main()
{
  // Resolve all config parameters from the 6-level hierarchy
  let runtime_params = HashMap::new();
  let config = AppConfig::resolve_all_config( &runtime_params );

  println!( "Resolved configuration:" );
  let mut keys : Vec< _ > = config.keys().collect();
  keys.sort();
  for key in keys
  {
    let ( value, source ) = &config[ key ];
    println!( "  {key:12} = {value:<20} (from {source:?})" );
  }

  // Validate all resolved values
  let errors = AppConfig::validate_all_config( &config );
  if errors.is_empty()
  {
    println!( "\nAll parameters valid." );
  }
  else
  {
    println!( "\nValidation errors:" );
    for err in &errors
    {
      println!( "  {} — {}", err.parameter, err.message );
    }
  }

  // Show resolved global config path
  match AppConfig::get_global_config_path()
  {
    Ok( path ) =>  println!( "\nGlobal config path: {}", path.display() ),
    Err( err ) =>  println!( "\nGlobal config path unavailable: {err}" ),
  }
}
