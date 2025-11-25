//! Pack command - Create portable archives with inlined content
//!
//! Implements FR7: Archive Serialization
//! Creates self-contained portable archives by internalizing all file content.

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".pack" ).expect( "valid command name" ),
    "Create portable archive from directory with inline content".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "pack".to_string(), "serialize".to_string(), "portable".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".pack input::\"./my-template\" output::\"template.json\"".to_string(),
    ".pack input::\"./src\" output::\"backup.yaml\" verbosity::2".to_string(),
    ".pack input::\"./templates\" output::\"archive.json\" dry::1".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "input", Kind::Directory )
      .with_description( "Source directory to pack" ),
    ArgumentDefinition::new( "output", Kind::Path )
      .with_description( "Output file path (JSON or YAML)" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "dry", Kind::Boolean )
      .with_description( "Dry run mode (0 or 1)" )
      .with_optional( Some( "0" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::pack::pack_handler ) )?;
  Ok( () )
}
