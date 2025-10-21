//! Archive lifecycle commands
//!
//! Commands for creating, loading, and saving template archives.

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register archive commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_new( registry )?;
  register_load( registry )?;
  register_save( registry )?;
  register_from_directory( registry )?;
  Ok( () )
}

/// Register .archive.new command
#[ allow( deprecated ) ]
fn register_new( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".archive.new".to_string(),
    namespace : String::new(),
    description : "Create new empty template archive".to_string(),
    hint : "Initialize new archive".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "archive".to_string(), "create".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".archive.new name::\"my-template\"".to_string(),
      ".archive.new name::\"api-scaffold\" description::\"REST API template\"".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "name".to_string(),
        description : "Archive name".to_string(),
        kind : Kind::String,
        hint : "Name for the archive".to_string(),
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
        name : "description".to_string(),
        description : "Archive description".to_string(),
        kind : Kind::String,
        hint : "Optional description".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( String::new() ),
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
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::new_handler ) )?;
  Ok( () )
}

/// Register .archive.load command
#[ allow( deprecated ) ]
fn register_load( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".archive.load".to_string(),
    namespace : String::new(),
    description : "Load archive from JSON or YAML file".to_string(),
    hint : "Load existing archive".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "archive".to_string(), "load".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".archive.load path::\"template.json\"".to_string(),
      ".archive.load path::\"./archives/api.yaml\" verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "path".to_string(),
        description : "Path to archive file (JSON or YAML)".to_string(),
        kind : Kind::Path,
        hint : "Archive file path".to_string(),
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
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::load_handler ) )?;
  Ok( () )
}

/// Register .archive.save command
#[ allow( deprecated ) ]
fn register_save( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".archive.save".to_string(),
    namespace : String::new(),
    description : "Save current archive to JSON or YAML file".to_string(),
    hint : "Save archive to file".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "archive".to_string(), "save".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".archive.save path::\"output.json\"".to_string(),
      ".archive.save path::\"template.yaml\" format::yaml pretty::0".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "path".to_string(),
        description : "Output file path".to_string(),
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
        name : "format".to_string(),
        description : "Serialization format (json or yaml)".to_string(),
        kind : Kind::Enum( vec![ "json".to_string(), "yaml".to_string() ] ),
        hint : "Format (auto-detect from extension)".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "json".to_string() ),
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
        name : "pretty".to_string(),
        description : "Pretty-print JSON (0 or 1)".to_string(),
        kind : Kind::Boolean,
        hint : "Pretty-print flag".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::save_handler ) )?;
  Ok( () )
}

/// Register .`archive.from_directory` command
#[ allow( deprecated ) ]
fn register_from_directory( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".archive.from_directory".to_string(),
    namespace : String::new(),
    description : "Create archive from filesystem directory".to_string(),
    hint : "Create from directory".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "archive".to_string(), "create".to_string(), "directory".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".archive.from_directory source::\"./templates\" mode::reference".to_string(),
      ".archive.from_directory source::\"./src\" mode::inline exclude_pattern::\"**/target/**\"".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "source".to_string(),
        description : "Source directory to scan".to_string(),
        kind : Kind::Directory,
        hint : "Source directory path".to_string(),
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
        name : "mode".to_string(),
        description : "Content mode: inline (embedded) or reference (file refs)".to_string(),
        kind : Kind::Enum( vec![ "inline".to_string(), "reference".to_string() ] ),
        hint : "Content mode".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "reference".to_string() ),
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
        name : "recursive".to_string(),
        description : "Scan subdirectories recursively (0 or 1)".to_string(),
        kind : Kind::Boolean,
        hint : "Recursive scan flag".to_string(),
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
        name : "include_pattern".to_string(),
        description : "Include files matching glob pattern".to_string(),
        kind : Kind::String,
        hint : "Include pattern".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
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
        name : "exclude_pattern".to_string(),
        description : "Exclude files matching glob pattern".to_string(),
        kind : Kind::String,
        hint : "Exclude pattern".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
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
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::from_directory_handler ) )?;
  Ok( () )
}
