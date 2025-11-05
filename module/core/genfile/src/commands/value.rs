//! Value management commands
//!
//! Commands for managing runtime parameter values

use unilang::registry::CommandRegistry;
use unilang::data::
{
  CommandDefinition,
  ArgumentDefinition,
  Kind,
  CommandName,
  CommandStatus,
  VersionType,
};

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".value.set" ).expect( "valid command name" ),
    "Set runtime value for a template parameter".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "value".to_string(), "set".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".value.set name::host value::\"example.com\"".to_string(),
    ".value.set name::port value::\"8080\"".to_string(),
    ".value.set name::enabled value::\"true\"".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "name", Kind::String )
      .with_description( "Parameter name" ),
    ArgumentDefinition::new( "value", Kind::String )
      .with_description( "Parameter value (as string)" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::value::set_handler ) )?;
  Ok( () )
}

/// Register .value.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".value.list" ).expect( "valid command name" ),
    "List all current parameter values".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "value".to_string(), "list".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".value.list".to_string(),
    ".value.list verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::value::list_handler ) )?;
  Ok( () )
}

/// Register .value.clear command
#[ allow( deprecated ) ]
fn register_clear( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".value.clear" ).expect( "valid command name" ),
    "Clear all parameter values (reset to defaults)".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "value".to_string(), "clear".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".value.clear".to_string(),
    ".value.clear verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::value::clear_handler ) )?;
  Ok( () )
}
