//! Parameter management commands
//!
//! Commands for managing template parameter definitions

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register all parameter commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_add( registry )?;
  register_list( registry )?;
  register_remove( registry )?;
  Ok( () )
}

/// Register .parameter.add command
#[ allow( deprecated ) ]
fn register_add( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".parameter.add".to_string(),
    namespace : String::new(),
    description : "Add a parameter definition to the archive with metadata".to_string(),
    hint : "Add parameter".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "parameter".to_string(), "add".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".parameter.add name::host mandatory::true description::\"Server hostname\"".to_string(),
      ".parameter.add name::port mandatory::false default::\"8080\"".to_string(),
      ".parameter.add name::name description::\"Project name\"".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "name".to_string(),
        description : "Parameter name".to_string(),
        kind : Kind::String,
        hint : "Parameter name".to_string(),
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
        name : "mandatory".to_string(),
        description : "Whether this parameter is required (0 or 1)".to_string(),
        kind : Kind::Boolean,
        hint : "Is mandatory".to_string(),
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
      ArgumentDefinition
      {
        name : "default".to_string(),
        description : "Default value for the parameter".to_string(),
        kind : Kind::String,
        hint : "Default value".to_string(),
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
        name : "description".to_string(),
        description : "Description of the parameter".to_string(),
        kind : Kind::String,
        hint : "Parameter description".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::parameter::add_handler ) )?;
  Ok( () )
}

/// Register .parameter.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".parameter.list".to_string(),
    namespace : String::new(),
    description : "List all parameter definitions in the archive".to_string(),
    hint : "List parameters".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "parameter".to_string(), "list".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".parameter.list".to_string(),
      ".parameter.list verbosity::2".to_string(),
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
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::parameter::list_handler ) )?;
  Ok( () )
}

/// Register .parameter.remove command
#[ allow( deprecated ) ]
fn register_remove( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".parameter.remove".to_string(),
    namespace : String::new(),
    description : "Remove a parameter definition from the archive".to_string(),
    hint : "Remove parameter".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "parameter".to_string(), "remove".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".parameter.remove name::host".to_string(),
      ".parameter.remove name::port verbosity::2".to_string(),
    ],
    routine_link : None,
    auto_help_enabled : true,
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "name".to_string(),
        description : "Parameter name to remove".to_string(),
        kind : Kind::String,
        hint : "Parameter name".to_string(),
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
    ],
  };

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::parameter::remove_handler ) )?;
  Ok( () )
}
