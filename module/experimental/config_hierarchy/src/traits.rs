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

/// Environment variable casing strategy
///
/// Controls how environment variable names are formatted:
/// - `UpperCase`: `MYAPP_TIMEOUT` (default, standard convention)
/// - `LowerCase`: `myapp_timeout` (some environments prefer lowercase)
/// - `PreserveAppName`: `myapp_TIMEOUT` (preserve `app_name` casing, uppercase parameters)
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum EnvVarCasing
{
  /// Uppercase prefix and parameters: `MYAPP_TIMEOUT` (default)
  UpperCase,
  /// Lowercase prefix and parameters: `myapp_timeout`
  LowerCase,
  /// Preserve app name casing, uppercase parameters: `myapp_TIMEOUT`
  PreserveAppName,
}

/// Provides application-specific path configuration
///
/// By default, all paths derive from `app_name()` following standard conventions:
/// - Local config: `./.{app_name()}/config.yaml`
/// - Global config: `$PRO/.persistent/.{app_name()}/config.yaml`
/// - Environment variables: `{APP_NAME}_PARAMETER`
///
/// # Minimal Usage (Zero Configuration)
///
/// Implement only `app_name()` to use all defaults:
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
///
/// # Advanced Usage (Custom Configuration)
///
/// Override any method to customize behavior:
///
/// ```
/// use config_hierarchy::{ ConfigPaths };
///
/// struct CustomPaths;
///
/// impl ConfigPaths for CustomPaths
/// {
///   fn app_name() -> &'static str { "myapp" }
///
///   // Custom local directory prefix (default: ".")
///   fn local_permanent_prefix() -> &'static str { "_" }
///
///   // Custom config filename (default: "config.yaml")
///   fn local_config_filename() -> &'static str { "settings.toml" }
/// }
///
/// // Results in:
/// // - Local:  ./_myapp/settings.toml
/// // - Global: $PRO/.persistent/.myapp/settings.toml (uses default)
/// // - Env:    MYAPP_TIMEOUT (uses default)
/// ```
pub trait ConfigPaths
{
  /// Application name without prefix (e.g., "unikit", "runbox", "myapp")
  ///
  /// This is the only required method. All other configuration derives from this name.
  ///
  /// **Required**: No default provided - every implementation must specify app name
  fn app_name() -> &'static str;

  // ============================================================================
  // Environment Variable Configuration
  // ============================================================================

  /// Environment variable prefix (default: uppercase app name)
  ///
  /// Controls the prefix used for environment variables.
  ///
  /// **Default**: `app_name().to_uppercase()` → `"MYAPP"`
  ///
  /// # Example
  ///
  /// ```
  /// use config_hierarchy::{ ConfigPaths };
  ///
  /// struct CustomPrefix;
  ///
  /// impl ConfigPaths for CustomPrefix
  /// {
  ///   fn app_name() -> &'static str { "myapp" }
  ///   fn env_var_prefix() -> &'static str { "MY_CUSTOM_APP" }
  /// }
  ///
  /// // Environment variables: MY_CUSTOM_APP_TIMEOUT, MY_CUSTOM_APP_DEBUG
  /// ```
  #[must_use]
  fn env_var_prefix() -> &'static str
  {
    Box::leak( Self::app_name().to_uppercase().into_boxed_str() )
  }

  /// Environment variable separator (default: "_")
  ///
  /// Character(s) separating prefix and parameter name.
  ///
  /// **Default**: `"_"` → `MYAPP_TIMEOUT`
  ///
  /// # Example
  ///
  /// ```
  /// use config_hierarchy::{ ConfigPaths };
  ///
  /// struct DotSeparator;
  ///
  /// impl ConfigPaths for DotSeparator
  /// {
  ///   fn app_name() -> &'static str { "myapp" }
  ///   fn env_var_separator() -> &'static str { "." }
  /// }
  ///
  /// // Environment variables: MYAPP.TIMEOUT, MYAPP.DEBUG
  /// ```
  #[must_use]
  fn env_var_separator() -> &'static str { "_" }

  /// Environment variable casing strategy (default: uppercase)
  ///
  /// Controls how variable names are cased.
  ///
  /// **Default**: `EnvVarCasing::UpperCase` → `MYAPP_TIMEOUT`
  ///
  /// # Example
  ///
  /// ```
  /// use config_hierarchy::{ ConfigPaths, EnvVarCasing };
  ///
  /// struct LowercaseVars;
  ///
  /// impl ConfigPaths for LowercaseVars
  /// {
  ///   fn app_name() -> &'static str { "myapp" }
  ///   fn env_var_casing() -> EnvVarCasing { EnvVarCasing::LowerCase }
  /// }
  ///
  /// // Environment variables: myapp_timeout, myapp_debug
  /// ```
  #[must_use]
  fn env_var_casing() -> EnvVarCasing { EnvVarCasing::UpperCase }

  // ============================================================================
  // Local Path Configuration
  // ============================================================================

  /// Local permanent directory prefix (default: ".")
  ///
  /// Prefix for permanent local config directories.
  ///
  /// **Default**: `"."` → `./.myapp/`
  ///
  /// # Example
  ///
  /// ```
  /// use config_hierarchy::{ ConfigPaths };
  ///
  /// struct UnderscorePrefix;
  ///
  /// impl ConfigPaths for UnderscorePrefix
  /// {
  ///   fn app_name() -> &'static str { "myapp" }
  ///   fn local_permanent_prefix() -> &'static str { "_" }
  /// }
  ///
  /// // Local config: ./_myapp/config.yaml
  /// ```
  #[must_use]
  fn local_permanent_prefix() -> &'static str { "." }

  /// Local temporary directory prefix (default: "-")
  ///
  /// Prefix for temporary local config directories.
  ///
  /// **Default**: `"-"` → `./-myapp/`
  #[must_use]
  fn local_temporary_prefix() -> &'static str { "-" }

  /// Local config filename (default: "config.yaml")
  ///
  /// Filename for local configuration files.
  ///
  /// **Default**: `"config.yaml"`
  ///
  /// # Example
  ///
  /// ```
  /// use config_hierarchy::{ ConfigPaths };
  ///
  /// struct TomlConfig;
  ///
  /// impl ConfigPaths for TomlConfig
  /// {
  ///   fn app_name() -> &'static str { "myapp" }
  ///   fn local_config_filename() -> &'static str { "settings.toml" }
  /// }
  ///
  /// // Local config: ./.myapp/settings.toml
  /// ```
  #[must_use]
  fn local_config_filename() -> &'static str { "config.yaml" }

  // ============================================================================
  // Global Path Configuration
  // ============================================================================

  /// Global persistent subdirectory (default: ".persistent")
  ///
  /// Subdirectory under `$PRO` for persistent configurations.
  ///
  /// **Default**: `".persistent"` → `$PRO/.persistent/.myapp/`
  #[must_use]
  fn global_persistent_dir() -> &'static str { ".persistent" }

  /// Global config filename (default: "config.yaml")
  ///
  /// Filename for global configuration files.
  ///
  /// **Default**: `"config.yaml"` → `$PRO/.persistent/.myapp/config.yaml`
  #[must_use]
  fn global_config_filename() -> &'static str { "config.yaml" }

  // ============================================================================
  // Environment Variable Names
  // ============================================================================

  /// PRO environment variable name (default: "PRO")
  ///
  /// Variable pointing to the workspace root directory.
  ///
  /// **Default**: `"PRO"` → `$PRO/.persistent/.myapp/`
  #[must_use]
  fn pro_env_var() -> &'static str { "PRO" }

  /// HOME environment variable name (default: "HOME")
  ///
  /// Variable pointing to user home directory.
  ///
  /// **Default**: `"HOME"` → Used for OS-specific fallback paths
  #[must_use]
  fn home_env_var() -> &'static str { "HOME" }

  /// `XDG_CONFIG_HOME` variable name (default: `"XDG_CONFIG_HOME"`)
  ///
  /// Linux XDG Base Directory specification variable.
  ///
  /// **Default**: `"XDG_CONFIG_HOME"` → Linux config path discovery
  #[must_use]
  fn xdg_config_home_var() -> &'static str { "XDG_CONFIG_HOME" }

  /// APPDATA variable name (default: "APPDATA")
  ///
  /// Windows application data directory variable.
  ///
  /// **Default**: `"APPDATA"` → Windows config path discovery
  #[must_use]
  fn appdata_var() -> &'static str { "APPDATA" }

  // ============================================================================
  // OS-Specific Path Bases
  // ============================================================================

  /// Linux config base directory (default: ".config")
  ///
  /// Base directory for Linux user configuration.
  ///
  /// **Default**: `".config"` → `$HOME/.config/myapp/`
  #[must_use]
  fn linux_config_base() -> &'static str { ".config" }

  /// macOS config base path (default: "Library/Application Support")
  ///
  /// Base path for macOS application support.
  ///
  /// **Default**: `"Library/Application Support"` → `$HOME/Library/Application Support/myapp/`
  #[must_use]
  fn macos_config_base() -> &'static str { "Library/Application Support" }
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
