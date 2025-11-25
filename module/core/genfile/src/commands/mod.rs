//! Command definitions for genfile CLI
//!
//! This module organizes all command definitions into logical groups.
//! Each submodule registers commands with the unilang `CommandRegistry`.
//!
//! ## YAML-First Command Architecture
//!
//! **Authoritative Specification**: All command metadata is defined in `commands/*.yaml` files:
//! - `commands/archive.yaml` - Archive lifecycle (FR1)
//! - `commands/file.yaml` - File operations (FR2)
//! - `commands/parameter.yaml` - Parameter management (FR3)
//! - `commands/value.yaml` - Value management (FR4)
//! - `commands/content.yaml` - Content management (FR5)
//! - `commands/materialize.yaml` - Materialization (FR6)
//! - `commands/pack.yaml` - Serialization (FR7)
//! - `commands/analysis.yaml` - Analysis (FR8)
//!
//! **Implementation Pattern**: The Rust code in this module duplicates the YAML specifications
//! as `CommandDefinition` structs and registers them with handler routines.
//!
//! **Why Hybrid Approach?** unilang's Multi-YAML Build system is designed for internal use
//! within unilang itself. External consumers like genfile cannot access:
//! - Build-time static registry generation (runs only in unilang, not dependent crates)
//! - `MultiYamlAggregator` build APIs (not exported for external use)
//! - Runtime API to attach routines to pre-loaded YAML commands (`routines` `HashMap` is private)
//!
//! **Maintenance Rule**: When adding or modifying commands:
//! 1. Update the YAML specification file first (single source of truth)
//! 2. Update the corresponding Rust `CommandDefinition` to match exactly
//! 3. Verify consistency between YAML and Rust implementations
//!
//! **Future Migration**: When unilang adds external consumer support for Multi-YAML,
//! this module can be simplified by removing the `CommandDefinition` boilerplate
//! and using direct YAML loading with routine attachment.

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
/// - Archive management (.archive.*) - See commands/archive.yaml
/// - File operations (.file.*) - See commands/file.yaml
/// - Parameter management (.parameter.*) - See commands/parameter.yaml
/// - Value management (.value.*) - See commands/value.yaml
/// - Content management (.content.*) - See commands/content.yaml
/// - Materialization (.materialize, .unpack) - See commands/materialize.yaml
/// - Serialization (.pack) - See commands/pack.yaml
/// - Analysis (.info, .status, .analyze, .discover.*) - See commands/analysis.yaml
#[ allow( deprecated ) ]
pub fn create_registry() -> Result< CommandRegistry, Box< dyn core::error::Error > >
{
  let mut registry = CommandRegistry::new();

  // Register command groups
  // NOTE: These implementations must match the specifications in commands/*.yaml
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
