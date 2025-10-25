//! Analysis and information command definitions - FR8: Archive Analysis
//!
//! Commands for inspecting and understanding template archives:
//! - `.info` - Archive metadata and statistics
//! - `.discover.parameters` - Auto-detect template parameters
//! - `.status` - Archive readiness check
//! - `.analyze` - Comprehensive analysis

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register analysis commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_info( registry )?;
  register_discover_parameters( registry )?;
  register_status( registry )?;
  register_analyze( registry )?;
  Ok( () )
}

/// Register .info command
#[ allow( deprecated ) ]
fn register_info( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".info".to_string(),
    namespace : String::new(),
    description : "Display archive metadata and statistics".to_string(),
    hint : "Show archive information".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "info".to_string(), "metadata".to_string(), "inspect".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".info".to_string(),
      ".info verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "verbosity".to_string(),
        description : "Output verbosity level (0-5)".to_string(),
        kind : Kind::Integer,
        hint : "Verbosity level".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "1".to_string() ),
          sensitive : false,
          interactive : false,
          multiple : false,
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::info_handler ) )?;
  Ok( () )
}

/// Register .discover.parameters command
#[ allow( deprecated ) ]
fn register_discover_parameters( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".discover.parameters".to_string(),
    namespace : String::new(),
    description : "Auto-detect template parameters in archive files".to_string(),
    hint : "Discover template variables".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "discover".to_string(), "parameters".to_string(), "detect".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".discover.parameters".to_string(),
      ".discover.parameters verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "verbosity".to_string(),
        description : "Output verbosity level (0-5)".to_string(),
        kind : Kind::Integer,
        hint : "Verbosity level".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "1".to_string() ),
          sensitive : false,
          interactive : false,
          multiple : false,
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::discover_parameters_handler ) )?;
  Ok( () )
}

/// Register .status command
#[ allow( deprecated ) ]
fn register_status( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".status".to_string(),
    namespace : String::new(),
    description : "Show archive readiness and completeness status".to_string(),
    hint : "Check archive status".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "status".to_string(), "readiness".to_string(), "check".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".status".to_string(),
      ".status verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "verbosity".to_string(),
        description : "Output verbosity level (0-5)".to_string(),
        kind : Kind::Integer,
        hint : "Verbosity level".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "1".to_string() ),
          sensitive : false,
          interactive : false,
          multiple : false,
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::status_handler ) )?;
  Ok( () )
}

/// Register .analyze command
#[ allow( deprecated ) ]
fn register_analyze( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".analyze".to_string(),
    namespace : String::new(),
    description : "Comprehensive archive analysis including all insights".to_string(),
    hint : "Analyze archive".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "analyze".to_string(), "inspect".to_string(), "comprehensive".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".analyze".to_string(),
      ".analyze verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "verbosity".to_string(),
        description : "Output verbosity level (0-5)".to_string(),
        kind : Kind::Integer,
        hint : "Verbosity level".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "1".to_string() ),
          sensitive : false,
          interactive : false,
          multiple : false,
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::analyze_handler ) )?;
  Ok( () )
}
