use std::{ env, path::PathBuf };
use crate::ConfigPaths;

/// Get local config directory name (always `.{app_name()}`)
#[ inline ]
fn local_config_dir_name< P : ConfigPaths >() -> String
{
  format!( ".{}", P::app_name() )
}

/// Get global config subdirectory name (always `.{app_name()}`)
#[ inline ]
fn global_config_subdir< P : ConfigPaths >() -> String
{
  format!( ".{}", P::app_name() )
}

/// Get OS-specific global config directory
/// Priority: `$PRO/.persistent/.{app_name}` > OS-specific config dir
///
/// # Errors
///
/// Returns error if no valid config directory can be determined from environment variables
#[ inline ]
pub fn get_global_config_dir< P : ConfigPaths >() -> Result< PathBuf, String >
{
  // 1. Try $PRO first
  if let Ok( pro_path ) = env::var( "PRO" )
  {
    let path = PathBuf::from( pro_path )
      .join( ".persistent" )
      .join( global_config_subdir::< P >() );
    return Ok( path );
  }

  // 2. Fall back to OS-specific paths
  #[ cfg( target_os = "linux" ) ]
  {
    // Try XDG_CONFIG_HOME first
    if let Ok( xdg_config ) = env::var( "XDG_CONFIG_HOME" )
    {
      return Ok( PathBuf::from( xdg_config ).join( P::app_name() ) );
    }
    // Fall back to ~/.config/app
    if let Ok( home ) = env::var( "HOME" )
    {
      return Ok( PathBuf::from( home ).join( ".config" ).join( P::app_name() ) );
    }
  }

  #[ cfg( target_os = "macos" ) ]
  {
    if let Ok( home ) = env::var( "HOME" )
    {
      return Ok( PathBuf::from( home ).join( "Library" ).join( "Application Support" ).join( P::app_name() ) );
    }
  }

  #[ cfg( target_os = "windows" ) ]
  {
    if let Ok( appdata ) = env::var( "APPDATA" )
    {
      return Ok( PathBuf::from( appdata ).join( P::app_name() ) );
    }
  }

  // Ultimate fallback
  if let Ok( home ) = env::var( "HOME" )
  {
    return Ok( PathBuf::from( home ).join( ".config" ).join( P::app_name() ) );
  }

  Err( "Cannot determine config directory: no $PRO, $HOME, $XDG_CONFIG_HOME, or $APPDATA".to_string() )
}

/// Get global config file path
///
/// # Errors
///
/// Returns error if global config directory cannot be determined
#[ inline ]
pub fn get_global_config_path< P : ConfigPaths >() -> Result< PathBuf, String >
{
  Ok( get_global_config_dir::< P >()?.join( "config.yaml" ) )
}

/// Get local config file path in current directory
///
/// # Errors
///
/// Returns error if current directory cannot be determined
#[ inline ]
pub fn get_local_config_path< P : ConfigPaths >() -> Result< PathBuf, String >
{
  let current_dir = env::current_dir()
    .map_err( | e | format!( "Cannot determine current directory: {e}" ) )?;

  Ok( current_dir
    .join( local_config_dir_name::< P >() )
    .join( "config.yaml" ) )
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

  let Ok( mut current_dir ) = env::current_dir() else
  {
    return configs;
  };

  let mut depth = 0;

  // Walk up directory tree
  loop
  {
    let app_name = P::app_name();

    // Priority 1: Temporary config -{app}/config.yaml
    let temp_config = current_dir
      .join( format!( "-{app_name}" ) )
      .join( "config.yaml" );

    if temp_config.exists()
    {
      configs.push( ( temp_config, depth ) );
    }

    // Priority 2: Permanent config .{app}/config.yaml
    let perm_config = current_dir
      .join( format!( ".{app_name}" ) )
      .join( "config.yaml" );

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
