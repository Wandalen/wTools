//! Content management commands
//!
//! Commands for managing content sources: internalize, externalize, and list.

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register all content commands
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_internalize( registry )?;
  register_externalize( registry )?;
  register_list( registry )?;
  Ok( () )
}

/// Register .content.internalize command
#[ allow( deprecated ) ]
fn register_internalize( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".content.internalize".to_string(),
    namespace : String::new(),
    description : "Convert all external content references (file/URL) to inline content for portability".to_string(),
    hint : "Internalize content".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "content".to_string(), "internalize".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".content.internalize".to_string(),
      ".content.internalize verbosity::2".to_string(),
      ".content.internalize dry::true".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "verbosity".to_string(),
        description : "Output verbosity level (0=silent, 1=normal, 2=verbose)".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::content::internalize_handler ) )?;
  Ok( () )
}

/// Register .content.externalize command
#[ allow( deprecated ) ]
fn register_externalize( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".content.externalize".to_string(),
    namespace : String::new(),
    description : "Convert inline content to external file references for lightweight archives".to_string(),
    hint : "Externalize content".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "content".to_string(), "externalize".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".content.externalize base_path::./content".to_string(),
      ".content.externalize base_path::/tmp/archive-content verbosity::2".to_string(),
      ".content.externalize base_path::./content dry::true".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "base_path".to_string(),
        description : "Directory where content files will be written".to_string(),
        kind : Kind::Path,
        hint : "Content directory path".to_string(),
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
        description : "Output verbosity level (0=silent, 1=normal, 2=verbose)".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::content::externalize_handler ) )?;
  Ok( () )
}

/// Register .content.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".content.list".to_string(),
    namespace : String::new(),
    description : "List all content sources in the archive grouped by type".to_string(),
    hint : "List content sources".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "content".to_string(), "list".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".content.list".to_string(),
      ".content.list filter::file".to_string(),
      ".content.list filter::inline verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "filter".to_string(),
        description : "Filter by content type: inline, file, url, or all (default: all)".to_string(),
        kind : Kind::String,
        hint : "Filter type".to_string(),
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
        description : "Output verbosity level (0=silent, 1=normal, 2=verbose)".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::content::list_handler ) )?;
  Ok( () )
}
