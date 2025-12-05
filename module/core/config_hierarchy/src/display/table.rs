//! Table format output for configuration

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
use core::fmt::Write;
use crate::{ ConfigDefaults, ConfigPaths, ConfigSource, ValidationError, json_value_to_display_string };

// Re-export tree_fmt types for customization
pub use tree_fmt::{ RowBuilder as TreeRowBuilder, TableFormatter as TreeTableFormatter, TableConfig as TreeTableConfig };

/// Format configuration as a table
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
/// # #[cfg(feature = "display_table")] {
/// use config_hierarchy::display::{ table::format_config_table, DisplayOptions };
/// use config_hierarchy::{ ConfigSource, ValidationError, ConfigDefaults, ConfigPaths };
/// use serde_json::Value as JsonValue;
/// use std::collections::HashMap;
///
/// struct TestDefaults;
/// impl ConfigDefaults for TestDefaults {
///   fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
///   fn get_parameter_names() -> Vec< &'static str > { vec![] }
/// }
/// struct TestPaths;
/// impl ConfigPaths for TestPaths {
///   fn app_name() -> &'static str { "test" }
/// }
///
/// let mut config = HashMap::new();
/// config.insert( "key".to_string(), ( JsonValue::String( "value".into() ), ConfigSource::Default ) );
/// let errors : Vec< ValidationError > = vec![];
/// let options = DisplayOptions::default();
/// let output = format_config_table::< TestDefaults, TestPaths >( &config, &errors, &options );
/// assert!( output.contains( "key" ) );
/// # }
/// ```
#[ inline ]
#[ allow( clippy::implicit_hasher ) ]
pub fn format_config_table< D, P >(
  config : &HashMap< String, ( JsonValue, ConfigSource ) >,
  validation_errors : &[ ValidationError ],
  options : &super::DisplayOptions,
) -> String
where
  D : ConfigDefaults,
  P : ConfigPaths,
{
  let defaults = D::get_defaults();
  let mut output = String::new();

  // Show validation warnings if enabled and present
  if options.include_warnings && !validation_errors.is_empty()
  {
    let _ = writeln!( output, "⚠️  Configuration warnings:\n" );
    for err in validation_errors
    {
      let _ = writeln!( output, "  - {}: {}", err.parameter, err.message );
    }
    let _ = writeln!( output );
  }

  // Handle filter_key option
  if let Some( ref filter_key ) = options.filter_key
  {
    if let Some( ( value, source ) ) = config.get( filter_key )
    {
      return format!(
        "{}: {} ({})",
        filter_key,
        json_value_to_display_string( value ),
        source.display_name()
      );
    }
    return format!( "Parameter '{filter_key}' not found" );
  }

  let _ = writeln!( output, "Configuration:\n" );

  // Build parameter table
  let mut builder = RowBuilder::new( vec![ "Parameter".into(), "Value".into(), "Default".into(), "Source".into() ] );

  let mut sorted : Vec< _ > = config.iter().collect();
  sorted.sort_by( | a, b | a.0.cmp( b.0 ) );

  for ( key, ( value, source ) ) in sorted
  {
    let value_str = json_value_to_display_string( value );
    let default_str = defaults.get( key.as_str() )
      .map_or_else( || "-".to_string(), json_value_to_display_string );

    builder = builder.add_row( vec![
      key.clone(),
      value_str,
      default_str,
      source.display_name(),
    ] );
  }

  let tree = builder.build();
  let formatter = TableFormatter::with_config( TableConfig::default() );
  let _ = write!( output, "{}", formatter.format( &tree ) );

  // Add sources table if enabled
  if options.include_sources
  {
    let _ = writeln!( output, "\n\n{}", format_sources_table::< P >() );
  }

  output
}

/// Format configuration sources as a table
///
/// Shows which configuration sources are active and their paths
#[ inline ]
#[ must_use ]
pub fn format_sources_table< P : ConfigPaths >() -> String
{
  use crate::path_discovery::{ discover_local_configs };

  #[ cfg( feature = "migration" ) ]
  use crate::migration::get_global_config_path_with_migration;

  let mut output = String::new();
  let _ = writeln!( output, "Configuration sources:\n" );

  let mut sources_builder = RowBuilder::new( vec![ "Source".into(), "Status".into(), "Path".into() ] );

  // CLI parameters (always active)
  sources_builder = sources_builder.add_row( vec![
    "cli".into(),
    "active".into(),
    "(command-line parameters)".into(),
  ] );

  // Local configs
  let local_configs = discover_local_configs::< P >();
  if local_configs.is_empty()
  {
    let local_dir_name = format!( ".{}", P::app_name() );
    sources_builder = sources_builder.add_row( vec![
      "local".into(),
      "not found".into(),
      format!( "(no {}/config.yaml in tree)", local_dir_name ),
    ] );
  }
  else
  {
    for path in &local_configs
    {
      sources_builder = sources_builder.add_row( vec![
        "local".into(),
        "active".into(),
        path.display().to_string(),
      ] );
    }
  }

  // Global config (migration-aware if feature enabled)
  #[ cfg( feature = "migration" ) ]
  {
    match get_global_config_path_with_migration::< P >()
    {
      Ok( ( path, _ ) ) =>
      {
        let status = if path.exists() { "active" } else { "not found" };
        sources_builder = sources_builder.add_row( vec![
          "global".into(),
          status.into(),
          path.display().to_string(),
        ] );
      },
      Err( _ ) =>
      {
        sources_builder = sources_builder.add_row( vec![
          "global".into(),
          "error".into(),
          "(path unavailable)".into(),
        ] );
      },
    }
  }

  #[ cfg( not( feature = "migration" ) ) ]
  {
    use crate::path_discovery::get_global_config_path;
    match get_global_config_path::< P >()
    {
      Ok( path ) =>
      {
        let status = if path.exists() { "active" } else { "not found" };
        sources_builder = sources_builder.add_row( vec![
          "global".into(),
          status.into(),
          path.display().to_string(),
        ] );
      },
      Err( _ ) =>
      {
        sources_builder = sources_builder.add_row( vec![
          "global".into(),
          "error".into(),
          "(path unavailable)".into(),
        ] );
      },
    }
  }

  // Built-in defaults (always active)
  sources_builder = sources_builder.add_row( vec![
    "built-in".into(),
    "active".into(),
    "(hardcoded defaults)".into(),
  ] );

  let sources_tree = sources_builder.build();
  let formatter = TableFormatter::with_config( TableConfig::default() );
  let _ = write!( output, "{}", formatter.format( &sources_tree ) );

  output
}
