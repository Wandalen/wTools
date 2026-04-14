use std::{ env, path::PathBuf };
use crate::ConfigPaths;

/// Validate `app_name` to prevent path traversal and empty names
///
/// ## Fix(manual-testing)
/// Added validation to prevent security vulnerabilities from malicious `app_name` values.
///
/// ## Root cause
/// `ConfigPaths::app_name()` is user-provided with no validation. Empty strings create
/// invalid paths (e.g., `./config.yaml`). Path separators allow directory traversal
/// (e.g., `../../etc/passwd`).
///
/// ## Pitfall
/// Always validate user-provided strings used in path construction, even from trait
/// implementations. Never assume trait implementations provide safe values.
#[ inline ]
fn validate_app_name< P : ConfigPaths >() -> Result< (), String >
{
  let app_name = P::app_name();

  // Check for empty app_name
  if app_name.is_empty()
  {
    return Err( "app_name must not be empty".to_string() );
  }

  // Check for path separators (/ or \)
  if app_name.contains( '/' ) || app_name.contains( '\\' )
  {
    return Err( "app_name contains invalid characters (path separators)".to_string() );
  }

  // Check for parent directory reference
  if app_name.contains( ".." )
  {
    return Err( "app_name contains invalid characters (parent directory reference)".to_string() );
  }

  Ok( () )
}

/// Get local config directory name (using `P::local_permanent_prefix()` + `app_name()`)
///
/// # Errors
///
/// Returns error if `app_name` is invalid (empty, contains path separators, or `..`)
#[ inline ]
fn local_config_dir_name< P : ConfigPaths >() -> Result< String, String >
{
  validate_app_name::< P >()?;
  Ok( format!( "{}{}", P::local_permanent_prefix(), P::app_name() ) )
}

/// Get global config subdirectory name (using `P::local_permanent_prefix()` + `app_name()`)
///
/// # Errors
///
/// Returns error if `app_name` is invalid (empty, contains path separators, or `..`)
#[ inline ]
fn global_config_subdir< P : ConfigPaths >() -> Result< String, String >
{
  validate_app_name::< P >()?;
  Ok( format!( "{}{}", P::local_permanent_prefix(), P::app_name() ) )
}

/// Get OS-specific global config directory
/// Priority: `$PRO/.persistent/.{app_name}` > OS-specific config dir
///
/// # Errors
///
/// Returns error if no valid config directory can be determined from environment variables
/// or if `app_name` is invalid
#[ inline ]
pub fn get_global_config_dir< P : ConfigPaths >() -> Result< PathBuf, String >
{
  // Validate app_name first
  validate_app_name::< P >()?;

  let app_name = P::app_name();

  // 1. Try PRO env var first (using P::pro_env_var())
  if let Ok( pro_path ) = env::var( P::pro_env_var() )
  {
    let path = PathBuf::from( pro_path )
      .join( P::global_persistent_dir() )
      .join( global_config_subdir::< P >()? );
    return Ok( path );
  }

  // 2. Fall back to OS-specific paths
  #[ cfg( target_os = "linux" ) ]
  {
    // Try XDG_CONFIG_HOME first (using P::xdg_config_home_var())
    if let Ok( xdg_config ) = env::var( P::xdg_config_home_var() )
    {
      return Ok( PathBuf::from( xdg_config ).join( app_name ) );
    }
    // Fall back to HOME/.config/app (using P::home_env_var() and P::linux_config_base())
    if let Ok( home ) = env::var( P::home_env_var() )
    {
      return Ok( PathBuf::from( home ).join( P::linux_config_base() ).join( app_name ) );
    }
  }

  #[ cfg( target_os = "macos" ) ]
  {
    // Use HOME env var and macOS config base (using P::home_env_var() and P::macos_config_base())
    if let Ok( home ) = env::var( P::home_env_var() )
    {
      return Ok( PathBuf::from( home ).join( P::macos_config_base() ).join( app_name ) );
    }
  }

  #[ cfg( target_os = "windows" ) ]
  {
    // Use APPDATA env var (using P::appdata_var())
    if let Ok( appdata ) = env::var( P::appdata_var() )
    {
      return Ok( PathBuf::from( appdata ).join( app_name ) );
    }
  }

  // Ultimate fallback (using P::home_env_var() and P::linux_config_base())
  if let Ok( home ) = env::var( P::home_env_var() )
  {
    return Ok( PathBuf::from( home ).join( P::linux_config_base() ).join( app_name ) );
  }

  Err( format!( "Cannot determine config directory: no ${}, ${}, ${}, or ${}", P::pro_env_var(), P::home_env_var(), P::xdg_config_home_var(), P::appdata_var() ) )
}

/// Get global config file path
///
/// # Errors
///
/// Returns error if global config directory cannot be determined
#[ inline ]
pub fn get_global_config_path< P : ConfigPaths >() -> Result< PathBuf, String >
{
  Ok( get_global_config_dir::< P >()?.join( P::global_config_filename() ) )
}

/// Get local config file path in current directory
///
/// # Errors
///
/// Returns error if current directory cannot be determined or if `app_name` is invalid
#[ inline ]
pub fn get_local_config_path< P : ConfigPaths >() -> Result< PathBuf, String >
{
  let current_dir = env::current_dir()
    .map_err( | e | format!( "Cannot determine current directory: {e}" ) )?;

  Ok( current_dir
    .join( local_config_dir_name::< P >()? )
    .join( P::local_config_filename() ) )
}

/// Internal: Discover local configs with directory depth tracking
/// Returns Vec<(`PathBuf`, depth)> where depth=0 is current directory
///
/// Supports dual-pattern discovery:
/// - `-{app}/config.yaml` (temporary, higher priority within same directory)
/// - `.{app}/config.yaml` (permanent, lower priority within same directory)
///
/// Priority rule: Directory depth takes precedence over pattern type
#[ inline ]
#[ must_use ]
pub( crate ) fn discover_local_configs_internal< P : ConfigPaths >() -> Vec< ( PathBuf, usize ) >
{
  let mut configs = Vec::new();

  // Validate app_name first
  if validate_app_name::< P >().is_err()
  {
    // If app_name is invalid, return empty vec (fail silently for discovery)
    return configs;
  }

  let Ok( mut current_dir ) = env::current_dir() else
  {
    return configs;
  };

  let mut depth = 0;

  // Walk up directory tree
  loop
  {
    let app_name = P::app_name();

    // Priority 1: Temporary config using P::local_temporary_prefix()
    let temp_config = current_dir
      .join( format!( "{}{app_name}", P::local_temporary_prefix() ) )
      .join( P::local_config_filename() );

    if temp_config.exists()
    {
      configs.push( ( temp_config, depth ) );
    }

    // Priority 2: Permanent config using P::local_permanent_prefix()
    let perm_config = current_dir
      .join( format!( "{}{app_name}", P::local_permanent_prefix() ) )
      .join( P::local_config_filename() );

    if perm_config.exists()
    {
      configs.push( ( perm_config, depth ) );
    }

    // Try to move to parent directory
    if !current_dir.pop()
    {
      break; // Reached root
    }

    depth += 1;
  }

  configs
}

/// Discover all local config files from current directory up to root
/// Returns paths in priority order (current dir first, root last)
///
/// Supports dual-pattern discovery:
/// - `-{app}/config.yaml` (temporary, higher priority within same directory)
/// - `.{app}/config.yaml` (permanent, lower priority within same directory)
#[ inline ]
#[ must_use ]
pub fn discover_local_configs< P : ConfigPaths >() -> Vec< PathBuf >
{
  discover_local_configs_internal::< P >()
    .into_iter()
    .map( |( path, _depth )| path )
    .collect()
}
