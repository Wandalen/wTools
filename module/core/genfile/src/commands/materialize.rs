//! Materialize command - Render templates to destination directory
//!
//! Implements FR6: Template Materialization
//! Transforms template archives into actual files with parameter substitution.

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register materialize commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_materialize( registry )?;
  register_unpack( registry )?;
  Ok( () )
}

/// Register .materialize command
#[ allow( deprecated ) ]
fn register_materialize( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".materialize".to_string(),
    namespace : String::new(),
    description : "Render template archive to destination directory with parameter substitution".to_string(),
    hint : "Materialize templates to files".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "materialize".to_string(), "render".to_string(), "template".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".materialize destination::\"./output\"".to_string(),
      ".materialize destination::\"./my-project\" verbosity::2".to_string(),
      ".materialize destination::\"./preview\" dry::1".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "destination".to_string(),
        description : "Output directory for materialized files".to_string(),
        kind : Kind::Path,
        hint : "Destination directory path".to_string(),
        attributes : ArgumentAttributes
        {
          optional : false,
          default : None,
          sensitive : false,
          interactive : false,
          multiple : false,
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
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
      ArgumentDefinition
      {
        name : "dry".to_string(),
        description : "Dry run mode (0 or 1)".to_string(),
        kind : Kind::Boolean,
        hint : "Dry run flag".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "0".to_string() ),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::materialize::materialize_handler ) )?;
  Ok( () )
}

/// Register .unpack command
#[ allow( deprecated ) ]
fn register_unpack( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".unpack".to_string(),
    namespace : String::new(),
    description : "Unpack raw template files to destination without rendering".to_string(),
    hint : "Unpack templates without substitution".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "unpack".to_string(), "extract".to_string(), "template".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".unpack destination::\"./template-files\"".to_string(),
      ".unpack destination::\"./output\" verbosity::2".to_string(),
      ".unpack destination::\"./preview\" dry::1".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "destination".to_string(),
        description : "Output directory for unpacked template files".to_string(),
        kind : Kind::Path,
        hint : "Destination directory path".to_string(),
        attributes : ArgumentAttributes
        {
          optional : false,
          default : None,
          sensitive : false,
          interactive : false,
          multiple : false,
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
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
      ArgumentDefinition
      {
        name : "dry".to_string(),
        description : "Dry run mode (0 or 1)".to_string(),
        kind : Kind::Boolean,
        hint : "Dry run flag".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "0".to_string() ),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::materialize::unpack_handler ) )?;
  Ok( () )
}
