//! Configuration migration support for path standard changes
//!
//! Handles automatic migration from old-style paths (without dot) to new standard paths (with dot)

use std::{ env, path::{ Path, PathBuf }, fs };
use crate::{ ConfigPaths, file_ops::{ load_config_file, save_config_file } };

/// Find legacy global config (old style without dot)
///
/// Checks for config at `$PRO/.persistent/{app_name}/config.yaml` (old style)
/// instead of `$PRO/.persistent/.{app_name}/config.yaml` (new standard)
#[ inline ]
#[ must_use ]
pub fn find_legacy_global_config< P : ConfigPaths >() -> Option< PathBuf >
{
  // Check $PRO/.persistent/{name}/config.yaml (old style without dot)
  if let Ok( pro_path ) = env::var( "PRO" )
  {
    let old_path = PathBuf::from( pro_path )
      .join( ".persistent" )
      .join( P::app_name() )  // Without dot!
      .join( "config.yaml" );

    if old_path.exists()
    {
      return Some( old_path );
    }
  }

  None
}

/// Migrate config from old location to new
///
/// Reads config from old path, writes to new path, and deletes old file
///
/// # Errors
///
/// Returns error if reading, writing, or cleanup fails
#[ inline ]
pub fn migrate_global_config(
  old_path : &Path,
  new_path : &Path
) -> Result< (), String >
{
  // Read from old location
  let config = load_config_file( old_path )
    .map_err( | e | format!( "Failed to read old config: {e}" ) )?;

  // Create parent directory for new location if needed
  if let Some( parent ) = new_path.parent()
  {
    fs::create_dir_all( parent )
      .map_err( | e | format!( "Failed to create config directory: {e}" ) )?;
  }

  // Write to new location
  save_config_file( &config, new_path )
    .map_err( | e | format!( "Failed to write new config: {e}" ) )?;

  // Delete old file
  fs::remove_file( old_path )
    .map_err( | e | format!( "Failed to remove old config: {e}" ) )?;

  // Try to remove old directory if empty
  if let Some( old_dir ) = old_path.parent()
  {
    let _ = fs::remove_dir( old_dir );  // Ignore errors (dir may not be empty)
  }

  Ok( () )
}

/// Get global config path with automatic migration
///
/// Returns `(path, migrated)` where `migrated` is `true` if migration occurred
///
/// # Migration Logic
///
/// 1. If new path exists, return it (no migration needed)
/// 2. If old path exists, migrate to new path and return new path
/// 3. Otherwise return new path (no config exists yet)
///
/// # Errors
///
/// Returns error if path determination or migration fails
#[ inline ]
pub fn get_global_config_path_with_migration< P : ConfigPaths >()
  -> Result< ( PathBuf, bool ), String >
{
  let new_path = crate::path_discovery::get_global_config_path::< P >()?;

  // If new path exists, use it (no migration needed)
  if new_path.exists()
  {
    return Ok( ( new_path, false ) );
  }

  // Check for legacy config
  if let Some( old_path ) = find_legacy_global_config::< P >()
  {
    // Perform migration
    eprintln!( "ℹ️  Migrating configuration to new location:" );
    eprintln!( "   From: {}", old_path.display() );
    eprintln!( "   To:   {}", new_path.display() );

    migrate_global_config( &old_path, &new_path )?;

    eprintln!( "✅ Configuration migrated successfully\n" );
    return Ok( ( new_path, true ) );
  }

  // No config exists yet
  Ok( ( new_path, false ) )
}
