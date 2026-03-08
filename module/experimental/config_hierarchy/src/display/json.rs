//! JSON format output for configuration

use std::collections::HashMap;
use serde_json::{ Value as JsonValue, Map, json };
use crate::{ ConfigSource, ValidationError };

/// Format configuration as JSON
///
/// # Arguments
///
/// * `config` - Resolved configuration with sources
/// * `validation_errors` - Validation errors to display
/// * `options` - Display options
///
/// # Example
///
/// ```
/// use config_hierarchy::display::{ json::format_config_json, DisplayOptions };
/// use config_hierarchy::{ ConfigSource, ValidationError };
/// use serde_json::Value as JsonValue;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// config.insert( "key".to_string(), ( JsonValue::String( "value".into() ), ConfigSource::Default ) );
/// let errors : Vec< ValidationError > = vec![];
/// let options = DisplayOptions::default();
/// let output = format_config_json( &config, &errors, &options );
/// assert!( output.contains( "key" ) );
/// ```
#[ inline ]
#[ must_use ]
#[ allow( clippy::implicit_hasher ) ]
pub fn format_config_json(
  config : &HashMap< String, ( JsonValue, ConfigSource ) >,
  validation_errors : &[ ValidationError ],
  options : &super::DisplayOptions,
) -> String
{
  let mut json_obj = Map::new();

  // Add validation warnings if enabled and present
  if options.include_warnings && !validation_errors.is_empty()
  {
    let warnings : Vec< JsonValue > = validation_errors.iter().map( | err | {
      json!({ "parameter": err.parameter, "message": err.message })
    }).collect();
    json_obj.insert( "warnings".to_string(), JsonValue::Array( warnings ) );
  }

  // Handle filter_key option
  if let Some( ref filter_key ) = options.filter_key
  {
    if let Some( ( value, source ) ) = config.get( filter_key )
    {
      let mut param_obj = Map::new();
      param_obj.insert( "value".to_string(), value.clone() );
      param_obj.insert( "source".to_string(), JsonValue::String( source.display_name() ) );
      return serde_json::to_string_pretty( &JsonValue::Object( param_obj ) )
        .unwrap_or_else( | e | format!( "[ERROR] JSON formatting failed: {e}" ) );
    }
    return json!({ "error": format!( "Parameter '{}' not found", filter_key ) }).to_string();
  }

  let mut params_obj = Map::new();
  for ( key, ( value, source ) ) in config
  {
    let mut parameter_obj = Map::new();
    parameter_obj.insert( "value".to_string(), value.clone() );

    // Create source object with type field
    let mut source_map = Map::new();
    let source_type = match source
    {
      ConfigSource::Runtime => "runtime",
      ConfigSource::Environment => "env",
      ConfigSource::LocalCurrent( _ ) | ConfigSource::LocalParent( _ ) | ConfigSource::Global( _ ) => "file",
      ConfigSource::Default => "default",
    };
    source_map.insert( "type".to_string(), JsonValue::String( source_type.to_string() ) );

    // Add path for file-based sources
    match source
    {
      ConfigSource::LocalCurrent( path ) | ConfigSource::LocalParent( path ) | ConfigSource::Global( path ) =>
      {
        source_map.insert( "path".to_string(), JsonValue::String( path.display().to_string() ) );
      },
      _ => {},
    }

    parameter_obj.insert( "source".to_string(), JsonValue::Object( source_map ) );
    params_obj.insert( key.clone(), JsonValue::Object( parameter_obj ) );
  }
  json_obj.insert( "parameters".to_string(), JsonValue::Object( params_obj ) );

  serde_json::to_string_pretty( &JsonValue::Object( json_obj ) )
    .unwrap_or_else( | e | format!( "[ERROR] JSON formatting failed: {e}" ) )
}
