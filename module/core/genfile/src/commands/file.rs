//! File management commands
//!
//! Commands for adding, removing, and inspecting files in archives.

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register file commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_add( registry )?;
  register_remove( registry )?;
  register_list( registry )?;
  register_show( registry )?;
  Ok( () )
}

/// Register .file.add command
#[ allow( deprecated ) ]
fn register_add( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".file.add".to_string(),
    namespace : String::new(),
    description : "Add file to current archive".to_string(),
    hint : "Add file".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "file".to_string(), "add".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".file.add path::src/main.rs content::\"fn main() {}\"".to_string(),
      ".file.add path::readme.md from_file::/tmp/template.md".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "path".to_string(),
        description : "File path within archive".to_string(),
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
        name : "content".to_string(),
        description : "File content (text)".to_string(),
        kind : Kind::String,
        hint : "Text content".to_string(),
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
        name : "from_file".to_string(),
        description : "Source file to read content from".to_string(),
        kind : Kind::Path,
        hint : "Source file path".to_string(),
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
        name : "write_mode".to_string(),
        description : "Write mode (rewrite, append, skip)".to_string(),
        kind : Kind::Enum( vec![ "rewrite".to_string(), "append".to_string(), "skip".to_string() ] ),
        hint : "Write mode".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "rewrite".to_string() ),
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
        hint : "Verbosity".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::add_handler ) )?;
  Ok( () )
}

/// Register .file.remove command
#[ allow( deprecated ) ]
fn register_remove( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".file.remove".to_string(),
    namespace : String::new(),
    description : "Remove file from current archive".to_string(),
    hint : "Remove file".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "file".to_string(), "remove".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".file.remove path::src/old.rs".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "path".to_string(),
        description : "File path to remove".to_string(),
        kind : Kind::Path,
        hint : "File path".to_string(),
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
        hint : "Verbosity".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::remove_handler ) )?;
  Ok( () )
}

/// Register .file.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".file.list".to_string(),
    namespace : String::new(),
    description : "List all files in current archive".to_string(),
    hint : "List files".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "file".to_string(), "list".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".file.list".to_string(),
      ".file.list verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "verbosity".to_string(),
        description : "Output verbosity level (0-5)".to_string(),
        kind : Kind::Integer,
        hint : "Verbosity".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::list_handler ) )?;
  Ok( () )
}

/// Register .file.show command
#[ allow( deprecated ) ]
fn register_show( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".file.show".to_string(),
    namespace : String::new(),
    description : "Show file content from archive".to_string(),
    hint : "Show file content".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "file".to_string(), "show".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![
      ".file.show path::src/main.rs".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec![
      ArgumentDefinition
      {
        name : "path".to_string(),
        description : "File path to show".to_string(),
        kind : Kind::Path,
        hint : "File path".to_string(),
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
        hint : "Verbosity".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::show_handler ) )?;
  Ok( () )
}
