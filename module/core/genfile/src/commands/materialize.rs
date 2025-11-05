//! Materialize command - Render templates to destination directory
//!
//! Implements FR6: Template Materialization
//! Transforms template archives into actual files with parameter substitution.

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".materialize" ).expect( "valid command name" ),
    "Render template archive to destination directory with parameter substitution".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "materialize".to_string(), "render".to_string(), "template".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".materialize destination::\"./output\"".to_string(),
    ".materialize destination::\"./my-project\" verbosity::2".to_string(),
    ".materialize destination::\"./preview\" dry::1".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "destination", Kind::Path )
      .with_description( "Output directory for materialized files" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "dry", Kind::Boolean )
      .with_description( "Dry run mode (0 or 1)" )
      .with_optional( Some( "0" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::materialize::materialize_handler ) )?;
  Ok( () )
}

/// Register .unpack command
#[ allow( deprecated ) ]
fn register_unpack( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".unpack" ).expect( "valid command name" ),
    "Unpack raw template files to destination without rendering".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "unpack".to_string(), "extract".to_string(), "template".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".unpack destination::\"./template-files\"".to_string(),
    ".unpack destination::\"./output\" verbosity::2".to_string(),
    ".unpack destination::\"./preview\" dry::1".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "destination", Kind::Path )
      .with_description( "Output directory for unpacked template files" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "dry", Kind::Boolean )
      .with_description( "Dry run mode (0 or 1)" )
      .with_optional( Some( "0" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::materialize::unpack_handler ) )?;
  Ok( () )
}
