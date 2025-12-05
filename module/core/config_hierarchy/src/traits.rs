use std::collections::HashMap;
use serde_json::Value as JsonValue;
use crate::{ ValidationError, ConfigSource };

/// Provides application-specific default configuration values
///
/// # Example
///
/// ```
/// use config_hierarchy::{ ConfigDefaults };
/// use std::collections::HashMap;
/// use serde_json::Value as JsonValue;
///
/// struct MyAppDefaults;
///
/// impl ConfigDefaults for MyAppDefaults
/// {
///   fn get_defaults() -> HashMap< String, JsonValue >
///   {
///     let mut defaults = HashMap::new();
///     defaults.insert( "timeout".to_string(), JsonValue::Number( 30.into() ) );
///     defaults.insert( "debug".to_string(), JsonValue::Bool( false ) );
///     defaults
///   }
///
///   fn get_parameter_names() -> Vec< &'static str >
///   {
///     vec![ "timeout", "debug" ]
///   }
///}
/// ```
pub trait ConfigDefaults
{
  /// Returns default configuration as key-value pairs
  fn get_defaults() -> HashMap< String, JsonValue >;

  /// Returns list of known parameter names
  fn get_parameter_names() -> Vec< &'static str >;
}

/// Provides application-specific path configuration
///
/// All paths are automatically derived from `app_name()` following strict naming standards:
/// - Local config: `./.{app_name()}/config.yaml`
/// - Global config: `$PRO/.persistent/.{app_name()}/config.yaml`
/// - Environment variables: `{APP_NAME}_PARAMETER`
///
/// This design enforces the `.{utility_name}` convention and prevents non-standard configurations.
///
/// # Example
///
/// ```
/// use config_hierarchy::{ ConfigPaths };
///
/// struct MyAppPaths;
///
/// impl ConfigPaths for MyAppPaths
/// {
///   fn app_name() -> &'static str { "myapp" }
/// }
///
/// // All paths automatically derived:
/// // - Local:  ./.myapp/config.yaml
/// // - Global: $PRO/.persistent/.myapp/config.yaml
/// // - Env:    MYAPP_TIMEOUT, MYAPP_DEBUG, etc.
/// ```
pub trait ConfigPaths
{
  /// Application name without dot prefix (e.g., "unikit", "runbox", "myapp")
  ///
  /// All configuration paths are derived from this name:
  /// - Local directory: `.{app_name()}`
  /// - Global directory: `.{app_name()}`
  /// - Environment prefix: `{app_name().to_uppercase()}`
  ///
  /// # Examples
  ///
  /// For `app_name() = "unikit"`:
  /// - Local: `./.unikit/config.yaml`
  /// - Global: `$PRO/.persistent/.unikit/config.yaml`
  /// - Env: `UNIKIT_PARAMETER`
  fn app_name() -> &'static str;
}

/// Provides application-specific validation logic
///
/// # Example
///
/// ```
/// use config_hierarchy::{ ConfigValidator, ValidationError, ConfigSource };
/// use serde_json::Value as JsonValue;
/// use std::collections::HashMap;
///
/// struct MyAppValidator;
///
/// impl ConfigValidator for MyAppValidator
/// {
///   fn validate_parameter( param_name : &str, value : &JsonValue )
///     -> Result< (), ValidationError >
///   {
///     match param_name
///     {
///       "timeout" =>
///       {
///         if let Some( t ) = value.as_i64()
///         {
///           if !( 1..=300 ).contains( &t )
///           {
///             return Err( ValidationError::new( param_name, "must be 1-300" ) );
///           }
///         }
///       },
///       _ => {}
///     }
///     Ok( () )
///   }
///
///   fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > )
///     -> Vec< ValidationError >
///   {
///     Vec::new()
///   }
/// }
/// ```
pub trait ConfigValidator
{
  /// Validate a single parameter value
  ///
  /// # Errors
  ///
  /// Returns `ValidationError` if value is invalid
  fn validate_parameter( param_name : &str, value : &JsonValue )
    -> Result< (), ValidationError >;

  /// Validate entire configuration for cross-parameter constraints
  fn validate_all( config : &HashMap< String, ( JsonValue, ConfigSource ) > )
    -> Vec< ValidationError >;
}
