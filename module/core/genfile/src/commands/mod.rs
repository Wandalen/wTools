//! Command registration for genfile CLI
//!
//! Each submodule builds `CommandDefinition` structs and wires handler functions
//! into the unilang `CommandRegistry`.
//!
//! ## Command Domains
//!
//! - `archive`     — Archive lifecycle (.archive.*) — FR1
//! - `file`        — File operations (.file.*) — FR2
//! - `parameter`   — Parameter management (.parameter.*) — FR3
//! - `value`       — Value management (.value.*) — FR4
//! - `content`     — Content management (.content.*) — FR5
//! - `materialize` — Materialization (.materialize, .unpack) — FR6
//! - `pack`        — Serialization (.pack) — FR7
//! - `info`        — Analysis (.info, .status, .analyze, .discover.*) — FR8

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
/// - Analysis (.info, .status, .analyze, .discover.*)
///
/// # Errors
/// Returns an error if any command module registration fails.
pub fn create_registry() -> Result< CommandRegistry, Box< dyn core::error::Error > >
{
  let mut registry = CommandRegistry::new();

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
