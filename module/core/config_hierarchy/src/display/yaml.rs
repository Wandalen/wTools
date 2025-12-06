//! YAML format output for configuration

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use core::fmt::Write;
use crate::{ ConfigSource, ValidationError, json_value_to_display_string };

/// Format configuration as YAML
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
/// use config_hierarchy::display::{ yaml::format_config_yaml, DisplayOptions };
/// use config_hierarchy::{ ConfigSource, ValidationError };
/// use serde_json::Value as JsonValue;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// config.insert( "key".to_string(), ( JsonValue::String( "value".into() ), ConfigSource::Default ) );
/// let errors : Vec< ValidationError > = vec![];
/// let options = DisplayOptions::default();
/// let output = format_config_yaml( &config, &errors, &options );
/// assert!( output.contains( "key" ) );
/// ```
#[ inline ]
#[ must_use ]
#[ allow( clippy::implicit_hasher ) ]
pub fn format_config_yaml(
  config : &HashMap< String, ( JsonValue, ConfigSource ) >,
  validation_errors : &[ ValidationError ],
  options : &super::DisplayOptions,
) -> String
{
  let mut yaml = String::new();

  // Add validation warnings if enabled and present
  if options.include_warnings && !validation_errors.is_empty()
  {
    let _ = writeln!( yaml, "warnings:" );
    for err in validation_errors
    {
      let _ = writeln!( yaml, "  - parameter: {}", err.parameter );
      let _ = writeln!( yaml, "    message: {}", err.message );
    }
    let _ = writeln!( yaml );
  }

  // Handle filter_key option
  if let Some( ref filter_key ) = options.filter_key
  {
    if let Some( ( value, source ) ) = config.get( filter_key )
    {
      let _ = writeln!( yaml, "{filter_key}:" );
      let value_str = match value
      {
        JsonValue::String( s ) => format!( "\"{s}\"" ),
        _ => json_value_to_display_string( value ),
      };
      let _ = writeln!( yaml, "  value: {value_str}" );
      let _ = writeln!( yaml, "  source: {}", source.display_name() );
      return yaml;
    }
    return format!( "error: Parameter '{filter_key}' not found\n" );
  }

  let _ = writeln!( yaml, "configuration:" );

  // Sort keys for consistent output
  let mut sorted : Vec< _ > = config.iter().collect();
  sorted.sort_by( | a, b | a.0.cmp( b.0 ) );

  for ( key, ( value, source ) ) in sorted
  {
    let _ = writeln!( yaml, "  {key}:" );

    // Quote string values, don't quote numbers/booleans
    let value_str = match value
    {
      JsonValue::String( s ) => format!( "\"{s}\"" ),
      _ => json_value_to_display_string( value ),
    };
    let _ = writeln!( yaml, "    value: {value_str}" );

    // Add both type and source fields for compatibility
    let source_type = match source
    {
      ConfigSource::Runtime => "runtime",
      ConfigSource::Environment => "env",
      ConfigSource::LocalCurrent( _ ) | ConfigSource::LocalParent( _ ) | ConfigSource::Global( _ ) => "file",
      ConfigSource::Default => "default",
    };
    let _ = writeln!( yaml, "    type: {source_type}" );
    let _ = writeln!( yaml, "    source: {}", source.display_name() );
  }

  yaml
}
