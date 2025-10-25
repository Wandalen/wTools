//! Pack command - Create portable archives with inlined content
//!
//! Implements FR7: Archive Serialization
//! Creates self-contained portable archives by internalizing all file content.

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register pack commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_pack( registry )?;
  Ok( () )
}

/// Register .pack command
#[ allow( deprecated ) ]
fn register_pack( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".pack".to_string(),
    namespace : String::new(),
    description : "Create portable archive from directory with inline content".to_string(),
    hint : "Pack directory to portable archive".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "pack".to_string(), "serialize".to_string(), "portable".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".pack input::\"./my-template\" output::\"template.json\"".to_string(),
      ".pack input::\"./src\" output::\"backup.yaml\" verbosity::2".to_string(),
      ".pack input::\"./templates\" output::\"archive.json\" dry::1".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "input".to_string(),
        description : "Source directory to pack".to_string(),
        kind : Kind::Directory,
        hint : "Input directory path".to_string(),
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
        name : "output".to_string(),
        description : "Output file path (JSON or YAML)".to_string(),
        kind : Kind::Path,
        hint : "Output file path".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::pack::pack_handler ) )?;
  Ok( () )
}
