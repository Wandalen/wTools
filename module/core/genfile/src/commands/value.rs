//! Value management commands
//!
//! Commands for managing runtime parameter values

use unilang::registry::CommandRegistry;
use unilang::data::{ CommandDefinition, ArgumentDefinition, Kind, ArgumentAttributes };

/// Register all value commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_set( registry )?;
  register_list( registry )?;
  register_clear( registry )?;
  Ok( () )
}

/// Register .value.set command
#[ allow( deprecated ) ]
fn register_set( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".value.set".to_string(),
    namespace : String::new(),
    description : "Set runtime value for a template parameter".to_string(),
    hint : "Set parameter value".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "value".to_string(), "set".to_string() ],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".value.set name::host value::\"example.com\"".to_string(),
      ".value.set name::port value::\"8080\"".to_string(),
      ".value.set name::enabled value::\"true\"".to_string(),
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
        name : "value".to_string(),
        description : "Parameter value (as string)".to_string(),
        kind : Kind::String,
        hint : "Parameter value".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::value::set_handler ) )?;
  Ok( () )
}

/// Register .value.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".value.list".to_string(),
    namespace : String::new(),
    description : "List all current parameter values".to_string(),
    hint : "List values".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "value".to_string(), "list".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".value.list".to_string(),
      ".value.list verbosity::2".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::value::list_handler ) )?;
  Ok( () )
}

/// Register .value.clear command
#[ allow( deprecated ) ]
fn register_clear( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition
  {
    name : ".value.clear".to_string(),
    namespace : String::new(),
    description : "Clear all parameter values (reset to defaults)".to_string(),
    hint : "Clear values".to_string(),
    status : "stable".to_string(),
    version : "0.1.0".to_string(),
    aliases : vec![],
    tags : vec![ "value".to_string(), "clear".to_string() ],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec!
    [
      ".value.clear".to_string(),
      ".value.clear verbosity::2".to_string(),
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

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::value::clear_handler ) )?;
  Ok( () )
}
