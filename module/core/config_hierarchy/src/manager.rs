use std::{ collections::HashMap, path::PathBuf };
use core::marker::PhantomData;
use serde_json::Value as JsonValue;
use crate::
{
  ConfigDefaults, ConfigPaths, ConfigValidator,
  ConfigSource, ValidationError,
  path_discovery::{ get_global_config_dir, get_local_config_path, discover_local_configs },
  hierarchy::{ resolve_config_value, resolve_all_config },
};

#[ cfg( feature = "file_ops" ) ]
use crate::file_ops::{ load_config_file, save_config_file, delete_config_file };

#[ cfg( all( feature = "file_ops", feature = "migration" ) ) ]
use crate::file_ops::atomic_config_modify;

#[ cfg( feature = "migration" ) ]
use crate::migration::get_global_config_path_with_migration;

/// Generic configuration manager with hierarchical resolution
///
/// # Type Parameters
///
/// - `D`: `ConfigDefaults` implementation
/// - `P`: `ConfigPaths` implementation
/// - `V`: `ConfigValidator` implementation
///
/// # Example
///
/// ```
/// use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator };
/// use std::collections::HashMap;
/// use serde_json::Value as JsonValue;
///
/// struct MyDefaults;
/// impl ConfigDefaults for MyDefaults {
///   fn get_defaults() -> HashMap< String, JsonValue > {
///     let mut map = HashMap::new();
///     map.insert( "timeout".into(), JsonValue::Number( 30.into() ) );
///     map
///   }
///   fn get_parameter_names() -> Vec< &'static str > { vec![ "timeout" ] }
/// }
///
/// struct MyPaths;
/// impl ConfigPaths for MyPaths {
///   fn app_name() -> &'static str { "myapp" }
/// }
///
/// struct MyValidator;
/// impl ConfigValidator for MyValidator {
///   fn validate_parameter( _param : &str, _value : &JsonValue )
///     -> Result< (), config_hierarchy::ValidationError > { Ok( () ) }
///   fn validate_all( _config : &HashMap< String, ( JsonValue, config_hierarchy::ConfigSource ) > )
///     -> Vec< config_hierarchy::ValidationError > { Vec::new() }
/// }
///
/// type MyConfig = ConfigManager< MyDefaults, MyPaths, MyValidator >;
/// ```
#[ derive( Debug, Clone, Copy ) ]
pub struct ConfigManager< D, P, V >
where
  D : ConfigDefaults,
  P : ConfigPaths,
  V : ConfigValidator,
{
  _defaults : PhantomData< D >,
  _paths : PhantomData< P >,
  _validator : PhantomData< V >,
}

impl< D, P, V > ConfigManager< D, P, V >
where
  D : ConfigDefaults,
  P : ConfigPaths,
  V : ConfigValidator,
{
  /// Get global config directory path
  ///
  /// # Errors
  ///
  /// Returns error if environment variables are unavailable
  #[ inline ]
  pub fn get_global_config_dir() -> Result< PathBuf, String >
  {
    get_global_config_dir::< P >()
  }

  /// Get global config file path with automatic migration support
  ///
  /// If old-style config exists (without dot), it will be automatically migrated to new location
  ///
  /// # Errors
  ///
  /// Returns error if environment variables are unavailable or migration fails
  #[ cfg( feature = "migration" ) ]
  #[ inline ]
  pub fn get_global_config_path() -> Result< PathBuf, String >
  {
    let ( path, _migrated ) = get_global_config_path_with_migration::< P >()?;
    Ok( path )
  }

  /// Get local config file path in current directory
  ///
  /// # Errors
  ///
  /// Returns error if current directory cannot be determined
  #[ inline ]
  pub fn get_local_config_path() -> Result< PathBuf, String >
  {
    get_local_config_path::< P >()
  }

  /// Discover all local config files
  #[ inline ]
  #[ must_use ]
  pub fn discover_local_configs() -> Vec< PathBuf >
  {
    discover_local_configs::< P >()
  }

  /// Load config from file
  ///
  /// # Errors
  ///
  /// Returns error if file read or YAML parse fails
  #[ cfg( feature = "file_ops" ) ]
  #[ inline ]
  pub fn load_config_file( path : &std::path::Path ) -> Result< HashMap< String, JsonValue >, String >
  {
    load_config_file( path )
  }

  /// Save config to file
  ///
  /// # Errors
  ///
  /// Returns error if file write or YAML serialize fails
  #[ cfg( feature = "file_ops" ) ]
  #[ inline ]
  pub fn save_config_file( config : &HashMap< String, JsonValue >, path : &std::path::Path ) -> Result< (), String >
  {
    save_config_file( config, path )
  }

  /// Delete config file
  ///
  /// # Errors
  ///
  /// Returns error if file delete fails
  #[ cfg( feature = "file_ops" ) ]
  #[ inline ]
  pub fn delete_config_file( path : &std::path::Path ) -> Result< bool, String >
  {
    delete_config_file( path )
  }

  /// Save configuration to global config file
  ///
  /// # Errors
  ///
  /// Returns error if config path unavailable or file write fails
  #[ cfg( all( feature = "file_ops", feature = "migration" ) ) ]
  #[ inline ]
  pub fn save_global_config( config : &HashMap< String, JsonValue > ) -> Result< (), String >
  {
    let config_path = Self::get_global_config_path()?;
    save_config_file( config, &config_path )
  }

  /// Save configuration to local config file in current directory
  ///
  /// Dual-pattern behavior:
  /// - If `-{app}/config.yaml` exists: save to temp config
  /// - Else if `.{app}/config.yaml` exists: save to perm config
  /// - Else: create new perm config (not temp)
  ///
  /// # Errors
  ///
  /// Returns error if config path unavailable or file write fails
  #[ cfg( feature = "file_ops" ) ]
  #[ inline ]
  pub fn save_local_config( config : &HashMap< String, JsonValue > ) -> Result< (), String >
  {
    use std::env;

    let current_dir = env::current_dir()
      .map_err( | e | format!( "Cannot determine current directory: {e}" ) )?;

    let app_name = P::app_name();

    // Check for temp config first (-{app}/config.yaml)
    let temp_config_path = current_dir
      .join( format!( "-{app_name}" ) )
      .join( "config.yaml" );

    if temp_config_path.exists()
    {
      return save_config_file( config, &temp_config_path );
    }

    // Check for perm config (.{app}/config.yaml)
    let perm_config_path = current_dir
      .join( format!( ".{app_name}" ) )
      .join( "config.yaml" );

    if perm_config_path.exists()
    {
      return save_config_file( config, &perm_config_path );
    }

    // Neither exists - create perm (not temp)
    save_config_file( config, &perm_config_path )
  }

  /// Delete global configuration file
  ///
  /// # Errors
  ///
  /// Returns error if config path unavailable or file delete fails
  #[ cfg( all( feature = "file_ops", feature = "migration" ) ) ]
  #[ inline ]
  pub fn delete_global_config() -> Result< bool, String >
  {
    let config_path = Self::get_global_config_path()?;
    delete_config_file( &config_path )
  }

  /// Atomically modify global config with file locking
  ///
  /// # Errors
  ///
  /// Returns error if config path unavailable, file lock fails, or `modify_fn` returns error
  #[ cfg( all( feature = "file_ops", feature = "migration" ) ) ]
  #[ inline ]
  pub fn atomic_config_modify< F >( modify_fn : F ) -> Result< (), String >
  where
    F : FnOnce( &mut HashMap< String, JsonValue > ) -> Result< (), String >
  {
    let config_path = Self::get_global_config_path()?;
    atomic_config_modify( &config_path, modify_fn )
  }

  /// Get default parameter values
  #[ inline ]
  #[ must_use ]
  pub fn get_defaults() -> HashMap< String, JsonValue >
  {
    D::get_defaults()
  }

  /// Resolve single config value using full hierarchy
  #[ inline ]
  #[ must_use ]
  pub fn resolve_config_value(
    param_name : &str,
    runtime_params : &HashMap< String, String >,
  ) -> ( JsonValue, ConfigSource )
  {
    resolve_config_value::< D, P >( param_name, runtime_params )
  }

  /// Resolve all config values using full hierarchy
  #[ inline ]
  #[ must_use ]
  pub fn resolve_all_config(
    runtime_params : &HashMap< String, String >,
  ) -> HashMap< String, ( JsonValue, ConfigSource ) >
  {
    resolve_all_config::< D, P >( runtime_params )
  }

  /// Validate single parameter
  ///
  /// # Errors
  ///
  /// Returns error if validation fails
  #[ inline ]
  pub fn validate_parameter( param_name : &str, value : &JsonValue ) -> Result< (), ValidationError >
  {
    V::validate_parameter( param_name, value )
  }

  /// Validate all configuration
  #[ inline ]
  #[ must_use ]
  pub fn validate_all_config( config : &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError >
  {
    V::validate_all( config )
  }

  /// Format configuration as table
  ///
  /// # Arguments
  ///
  /// * `config` - Resolved configuration with sources
  /// * `validation_errors` - Validation errors to display
  /// * `options` - Display options
  #[ cfg( feature = "display_table" ) ]
  #[ inline ]
  #[must_use] 
  pub fn format_config_table(
    config : &HashMap< String, ( JsonValue, ConfigSource ) >,
    validation_errors : &[ ValidationError ],
    options : &crate::display::DisplayOptions,
  ) -> String
  {
    crate::display::table::format_config_table::< D, P >( config, validation_errors, options )
  }

  /// Format configuration as JSON
  ///
  /// # Arguments
  ///
  /// * `config` - Resolved configuration with sources
  /// * `validation_errors` - Validation errors to display
  /// * `options` - Display options
  #[ cfg( feature = "display_json" ) ]
  #[ inline ]
  #[must_use] 
  pub fn format_config_json(
    config : &HashMap< String, ( JsonValue, ConfigSource ) >,
    validation_errors : &[ ValidationError ],
    options : &crate::display::DisplayOptions,
  ) -> String
  {
    crate::display::json::format_config_json( config, validation_errors, options )
  }

  /// Format configuration as YAML
  ///
  /// # Arguments
  ///
  /// * `config` - Resolved configuration with sources
  /// * `validation_errors` - Validation errors to display
  /// * `options` - Display options
  #[ cfg( feature = "display_yaml" ) ]
  #[ inline ]
  #[must_use] 
  pub fn format_config_yaml(
    config : &HashMap< String, ( JsonValue, ConfigSource ) >,
    validation_errors : &[ ValidationError ],
    options : &crate::display::DisplayOptions,
  ) -> String
  {
    crate::display::yaml::format_config_yaml( config, validation_errors, options )
  }
}
