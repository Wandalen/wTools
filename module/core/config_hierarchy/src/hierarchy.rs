use std::{ env, collections::HashMap };
use serde_json::Value as JsonValue;
use crate::
{
  ConfigSource, ConfigDefaults, ConfigPaths, EnvVarCasing,
  type_detection::detect_and_convert_value,
};

#[ cfg( feature = "file_ops" ) ]
use crate::
{
  path_discovery::{ get_global_config_path, discover_local_configs },
  file_ops::load_config_file,
};

/// Resolve configuration value using full 6-level hierarchy
/// Returns (value, source) tuple
#[ inline ]
#[ must_use ]
#[ allow( clippy::implicit_hasher ) ]
pub fn resolve_config_value< D, P >(
  param_name : &str,
  runtime_params : &HashMap< String, String >,
) -> ( JsonValue, ConfigSource )
where
  D : ConfigDefaults,
  P : ConfigPaths,
{
  // 1. Runtime parameters (highest priority)
  if let Some( value_str ) = runtime_params.get( param_name )
  {
    return ( detect_and_convert_value( value_str ), ConfigSource::Runtime );
  }

  // 2. Environment variables (using P::env_var_prefix(), P::env_var_separator(), P::env_var_casing())
  let env_prefix = P::env_var_prefix();
  let separator = P::env_var_separator();

  // Apply casing based on P::env_var_casing()
  // Note: UpperCase and PreserveAppName both uppercase parameters, but differ in prefix handling
  // (UpperCase uppercases prefix, PreserveAppName preserves app_name casing in prefix)
  #[allow(clippy::match_same_arms)]
  let param_part = match P::env_var_casing()
  {
    EnvVarCasing::UpperCase => param_name.to_uppercase(),
    EnvVarCasing::LowerCase => param_name.to_lowercase(),
    EnvVarCasing::PreserveAppName => param_name.to_uppercase(),
  };

  let env_name = format!( "{env_prefix}{separator}{param_part}" );
  if let Ok( value_str ) = env::var( &env_name )
  {
    return ( detect_and_convert_value( &value_str ), ConfigSource::Environment );
  }

  // 3-4. Local configs (current and parents) - requires file_ops feature
  #[ cfg( feature = "file_ops" ) ]
  {
    use crate::discover_local_configs_internal;

    let local_configs = discover_local_configs_internal::< P >();
    for ( config_path, depth ) in &local_configs
    {
      if let Ok( config_data ) = load_config_file( config_path )
      {
        if let Some( value ) = config_data.get( param_name )
        {
          let source = if *depth == 0
          {
            ConfigSource::LocalCurrent( config_path.clone() )
          }
          else
          {
            ConfigSource::LocalParent( config_path.clone() )
          };
          return ( value.clone(), source );
        }
      }
    }
  }

  // 5. Global config - requires file_ops feature
  #[ cfg( feature = "file_ops" ) ]
  {
    if let Ok( global_path ) = get_global_config_path::< P >()
    {
      if let Ok( config_data ) = load_config_file( &global_path )
      {
        if let Some( value ) = config_data.get( param_name )
        {
          return ( value.clone(), ConfigSource::Global( global_path ) );
        }
      }
    }
  }

  // 6. Defaults (lowest priority)
  let defaults = D::get_defaults();
  if let Some( value ) = defaults.get( param_name )
  {
    return ( value.clone(), ConfigSource::Default );
  }

  // Fallback: null value with default source
  ( JsonValue::Null, ConfigSource::Default )
}

/// Resolve all configuration parameters using full hierarchy
#[ inline ]
#[ allow( clippy::implicit_hasher ) ]
pub fn resolve_all_config< D, P >(
  runtime_params : &HashMap< String, String >,
) -> HashMap< String, ( JsonValue, ConfigSource ) >
where
  D : ConfigDefaults,
  P : ConfigPaths,
{
  let mut resolved = HashMap::new();
  let param_names = D::get_parameter_names();

  for param_name in param_names
  {
    let ( value, source ) = resolve_config_value::< D, P >( param_name, runtime_params );
    resolved.insert( param_name.to_string(), ( value, source ) );
  }

  // Also check for any additional params in global or local configs not in defaults
  // This ensures custom user parameters are included (requires file_ops feature)

  #[ cfg( feature = "file_ops" ) ]
  {
    // Check global config
    if let Ok( global_path ) = get_global_config_path::< P >()
    {
      if let Ok( config_data ) = load_config_file( &global_path )
      {
        for key in config_data.keys()
        {
          if !resolved.contains_key( key )
          {
            let ( value, source ) = resolve_config_value::< D, P >( key, runtime_params );
            resolved.insert( key.clone(), ( value, source ) );
          }
        }
      }
    }

    // Check local configs
    for config_path in discover_local_configs::< P >()
    {
      if let Ok( config_data ) = load_config_file( &config_path )
      {
        for key in config_data.keys()
        {
          if !resolved.contains_key( key )
          {
            let ( value, source ) = resolve_config_value::< D, P >( key, runtime_params );
            resolved.insert( key.clone(), ( value, source ) );
          }
        }
      }
    }
  }

  resolved
}
