//! File operations for configuration management (feature = "`file_ops`")

use std::
{
  collections::HashMap,
  fs::{ self, OpenOptions },
  path::Path,
  io::Read,
};
use serde_json::Value as JsonValue;
use fs2::FileExt;
use crate::conversion::{ yaml_to_json, json_to_yaml };

/// Load config from a single YAML file
///
/// # Errors
///
/// Returns error if file read or YAML parse fails
#[ inline ]
pub fn load_config_file( path : &Path ) -> Result< HashMap< String, JsonValue >, String >
{
  if !path.exists()
  {
    return Ok( HashMap::new() );
  }

  let content = fs::read_to_string( path )
    .map_err( | e | format!( "Failed to read config file {}: {e}", path.display() ) )?;

  if content.trim().is_empty()
  {
    return Ok( HashMap::new() );
  }

  let yaml_value : serde_yaml::Value = serde_yaml::from_str( &content )
    .map_err( | e | format!( "Failed to parse YAML in {}: {e}", path.display() ) )?;

  let mut config = HashMap::new();

  if let serde_yaml::Value::Mapping( map ) = yaml_value
  {
    // Look for parameters section first
    if let Some( params_value ) = map.get( serde_yaml::Value::String( "parameters".to_string() ) )
    {
      if let serde_yaml::Value::Mapping( params_map ) = params_value
      {
        for ( key, value ) in params_map
        {
          if let serde_yaml::Value::String( key_str ) = key
          {
            config.insert( key_str.clone(), yaml_to_json( value.clone() ) );
          }
        }
      }
    }
    else
    {
      // Fallback: old flat format
      for ( key, value ) in map
      {
        if let serde_yaml::Value::String( key_str ) = key
        {
          // Skip metadata fields
          if key_str == "version" || key_str == "last_modified" || key_str == "metadata"
          {
            continue;
          }
          config.insert( key_str.clone(), yaml_to_json( value ) );
        }
      }
    }
  }

  Ok( config )
}

/// Save configuration to file with metadata
///
/// # Errors
///
/// Returns error if file write or YAML serialize fails
#[ inline ]
#[ allow( clippy::implicit_hasher ) ]
pub fn save_config_file( config : &HashMap< String, JsonValue >, config_path : &Path ) -> Result< (), String >
{
  ensure_config_dir_exists( config_path )?;
  build_and_write_config( config, config_path )
}

/// Delete configuration file
///
/// # Errors
///
/// Returns error if file delete fails
#[ inline ]
pub fn delete_config_file( config_path : &Path ) -> Result< bool, String >
{
  if !config_path.exists()
  {
    return Ok( false );
  }

  fs::remove_file( config_path )
    .map_err( | e | format!( "Failed to delete config file: {e}" ) )?;

  Ok( true )
}

/// Atomically modify config file with file locking
///
/// # Errors
///
/// Returns error if file lock fails or `modify_fn` returns error
#[ inline ]
pub fn atomic_config_modify< F >( config_path : &Path, modify_fn : F ) -> Result< (), String >
where
  F : FnOnce( &mut HashMap< String, JsonValue > ) -> Result< (), String >
{
  ensure_config_dir_exists( config_path )?;

  // Open or create config file
  let mut file = OpenOptions::new()
    .read( true )
    .write( true )
    .create( true )
    .truncate( false )
    .open( config_path )
    .map_err( | e | format!( "Failed to open config file for locking: {e}" ) )?;

  // Acquire exclusive lock
  file.lock_exclusive()
    .map_err( | e | format!( "Failed to acquire file lock: {e}" ) )?;

  // Load current config from the already-opened file handle
  let mut content = String::new();
  file.read_to_string( &mut content )
    .map_err( | e | format!( "Failed to read config file {}: {e}", config_path.display() ) )?;

  let mut config = if content.trim().is_empty()
  {
    HashMap::new()
  }
  else
  {
    let yaml_value : serde_yaml::Value = serde_yaml::from_str( &content )
      .map_err( | e | format!( "Failed to parse YAML in {}: {e}", config_path.display() ) )?;

    let mut parsed_config = HashMap::new();

    if let serde_yaml::Value::Mapping( map ) = yaml_value
    {
      // Look for parameters section first
      if let Some( params_value ) = map.get( serde_yaml::Value::String( "parameters".to_string() ) )
      {
        if let serde_yaml::Value::Mapping( params_map ) = params_value
        {
          for ( key, value ) in params_map
          {
            if let serde_yaml::Value::String( key_str ) = key
            {
              parsed_config.insert( key_str.clone(), yaml_to_json( value.clone() ) );
            }
          }
        }
      }
      // Fall back to root level if no parameters section
      else
      {
        for ( key, value ) in &map
        {
          if let serde_yaml::Value::String( key_str ) = key
          {
            parsed_config.insert( key_str.clone(), yaml_to_json( value.clone() ) );
          }
        }
      }
    }

    parsed_config
  };

  // Apply modifications
  modify_fn( &mut config )?;

  // Save while still holding lock
  let result = build_and_write_config( &config, config_path );

  // Release lock (automatic when file dropped)
  drop( file );

  result
}

/// Ensure config directory exists
#[ inline ]
fn ensure_config_dir_exists( config_path : &Path ) -> Result< (), String >
{
  if let Some( parent ) = config_path.parent()
  {
    if !parent.exists()
    {
      fs::create_dir_all( parent )
        .map_err( | e | format!( "Failed to create config directory: {e}" ) )?;
    }
  }
  Ok( () )
}

/// Build YAML structure and write to file
#[ inline ]
fn build_and_write_config( config : &HashMap< String, JsonValue >, config_path : &Path ) -> Result< (), String >
{
  // Try to preserve created_at timestamp
  let created_at = if config_path.exists()
  {
    extract_created_at_timestamp( config_path )
  }
  else
  {
    None
  };

  // Create YAML structure
  let mut root_map = serde_yaml::Mapping::new();

  // Metadata section
  let mut metadata_map = serde_yaml::Mapping::new();
  metadata_map.insert(
    serde_yaml::Value::String( "version".to_string() ),
    serde_yaml::Value::String( "1.0".to_string() )
  );

  let timestamp = chrono::Utc::now().to_rfc3339();
  metadata_map.insert(
    serde_yaml::Value::String( "created_at".to_string() ),
    serde_yaml::Value::String( created_at.unwrap_or_else( || timestamp.clone() ) )
  );
  metadata_map.insert(
    serde_yaml::Value::String( "last_modified".to_string() ),
    serde_yaml::Value::String( timestamp )
  );

  root_map.insert(
    serde_yaml::Value::String( "metadata".to_string() ),
    serde_yaml::Value::Mapping( metadata_map )
  );

  // Parameters section
  let mut params_map = serde_yaml::Mapping::new();
  for ( key, value ) in config
  {
    params_map.insert(
      serde_yaml::Value::String( key.clone() ),
      json_to_yaml( value.clone() )
    );
  }

  root_map.insert(
    serde_yaml::Value::String( "parameters".to_string() ),
    serde_yaml::Value::Mapping( params_map )
  );

  let yaml_value = serde_yaml::Value::Mapping( root_map );
  let yaml_string = serde_yaml::to_string( &yaml_value )
    .map_err( | e | format!( "Failed to serialize YAML: {e}" ) )?;

  fs::write( config_path, yaml_string )
    .map_err( | e | format!( "Failed to write config file: {e}" ) )?;

  Ok( () )
}

/// Extract `created_at` timestamp from existing config file
#[ inline ]
fn extract_created_at_timestamp( config_path : &Path ) -> Option< String >
{
  let content = fs::read_to_string( config_path ).ok()?;
  let yaml_value : serde_yaml::Value = serde_yaml::from_str( &content ).ok()?;

  if let serde_yaml::Value::Mapping( map ) = yaml_value
  {
    if let Some( serde_yaml::Value::Mapping( metadata_map ) ) =
      map.get( serde_yaml::Value::String( "metadata".to_string() ) )
    {
      if let Some( serde_yaml::Value::String( created_str ) ) =
        metadata_map.get( serde_yaml::Value::String( "created_at".to_string() ) )
      {
        return Some( created_str.clone() );
      }
    }
  }

  None
}
