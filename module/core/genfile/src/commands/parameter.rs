//! Parameter management commands
//!
//! Commands for managing template parameter definitions

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".parameter.add" ).expect( "valid command name" ),
    "Add a parameter definition to the archive with metadata".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "parameter".to_string(), "add".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".parameter.add name::host mandatory::true description::\"Server hostname\"".to_string(),
    ".parameter.add name::port mandatory::false default::\"8080\"".to_string(),
    ".parameter.add name::name description::\"Project name\"".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "name", Kind::String )
      .with_description( "Parameter name" ),
    ArgumentDefinition::new( "mandatory", Kind::Boolean )
      .with_description( "Whether this parameter is required (0 or 1)" )
      .with_optional( Some( "0" ) ),
    ArgumentDefinition::new( "default", Kind::String )
      .with_description( "Default value for the parameter" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "description", Kind::String )
      .with_description( "Description of the parameter" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::parameter::add_handler ) )?;
  Ok( () )
}

/// Register .parameter.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".parameter.list" ).expect( "valid command name" ),
    "List all parameter definitions in the archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "parameter".to_string(), "list".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".parameter.list".to_string(),
    ".parameter.list verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::parameter::list_handler ) )?;
  Ok( () )
}

/// Register .parameter.remove command
#[ allow( deprecated ) ]
fn register_remove( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".parameter.remove" ).expect( "valid command name" ),
    "Remove a parameter definition from the archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "parameter".to_string(), "remove".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".parameter.remove name::host".to_string(),
    ".parameter.remove name::port verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "name", Kind::String )
      .with_description( "Parameter name to remove" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::parameter::remove_handler ) )?;
  Ok( () )
}
