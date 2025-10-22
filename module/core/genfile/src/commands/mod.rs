//! Command definitions for genfile CLI
//!
//! This module organizes all command definitions into logical groups.
//! Each submodule registers commands with the unilang `CommandRegistry`.

use unilang::registry::CommandRegistry;

pub mod archive;
pub mod file;
pub mod parameter;
pub mod value;
pub mod content;
pub mod materialize;
pub mod pack;
pub mod info;

/// Create complete genfile command registry
///
/// Registers all command groups with the registry:
/// - Archive management (.archive.*)
/// - File operations (.file.*)
/// - Parameter management (.parameter.*)
/// - Value management (.value.*)
/// - Content management (.content.*)
/// - Materialization (.materialize, .unpack)
/// - Serialization (.pack)
/// - Information (.status, .info, .analyze, .discover.*)
#[ allow( deprecated ) ]
pub fn create_registry() -> Result< CommandRegistry, Box< dyn core::error::Error > >
{
  let mut registry = CommandRegistry::new();

  // Register command groups
  archive::register( &mut registry )?;
  file::register( &mut registry )?;
  parameter::register( &mut registry )?;
  value::register( &mut registry )?;
  content::register( &mut registry )?;
  materialize::register( &mut registry )?;
  pack::register( &mut registry )?;
  info::register( &mut registry )?;

  Ok( registry )
}
