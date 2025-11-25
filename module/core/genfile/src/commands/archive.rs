//! Archive lifecycle commands
//!
//! Commands for creating, loading, and saving template archives.

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".archive.new" ).expect( "valid command name" ),
    "Create new empty template archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "archive".to_string(), "create".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".archive.new name::\"my-template\"".to_string(),
    ".archive.new name::\"api-scaffold\" description::\"REST API template\"".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "name", Kind::String )
      .with_description( "Archive name" ),
    ArgumentDefinition::new( "description", Kind::String )
      .with_description( "Archive description" )
      .with_optional( Some( "" ) ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::new_handler ) )?;
  Ok( () )
}

/// Register .archive.load command
#[ allow( deprecated ) ]
fn register_load( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".archive.load" ).expect( "valid command name" ),
    "Load archive from JSON or YAML file".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "archive".to_string(), "load".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".archive.load path::\"template.json\"".to_string(),
    ".archive.load path::\"./archives/api.yaml\" verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "path", Kind::Path )
      .with_description( "Path to archive file (JSON or YAML)" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::load_handler ) )?;
  Ok( () )
}

/// Register .archive.save command
#[ allow( deprecated ) ]
fn register_save( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".archive.save" ).expect( "valid command name" ),
    "Save current archive to JSON or YAML file".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "archive".to_string(), "save".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".archive.save path::\"output.json\"".to_string(),
    ".archive.save path::\"template.yaml\" format::yaml pretty::0".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "path", Kind::Path )
      .with_description( "Output file path" ),
    ArgumentDefinition::new( "format", Kind::Enum( vec![ "json".to_string(), "yaml".to_string() ] ) )
      .with_description( "Serialization format (json or yaml)" )
      .with_optional( Some( "json" ) ),
    ArgumentDefinition::new( "pretty", Kind::Boolean )
      .with_description( "Pretty-print JSON (0 or 1)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "dry", Kind::Boolean )
      .with_description( "Dry run mode (0 or 1)" )
      .with_optional( Some( "0" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::save_handler ) )?;
  Ok( () )
}

/// Register `.archive.from_directory` command
#[ allow( deprecated ) ]
fn register_from_directory( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".archive.from_directory" ).expect( "valid command name" ),
    "Create archive from filesystem directory".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "archive".to_string(), "create".to_string(), "directory".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".archive.from_directory source::\"./templates\" mode::reference".to_string(),
    ".archive.from_directory source::\"./src\" mode::inline exclude_pattern::\"**/target/**\"".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "source", Kind::Directory )
      .with_description( "Source directory to scan" ),
    ArgumentDefinition::new( "mode", Kind::Enum( vec![ "inline".to_string(), "reference".to_string() ] ) )
      .with_description( "Content mode: inline (embedded) or reference (file refs)" )
      .with_optional( Some( "reference" ) ),
    ArgumentDefinition::new( "recursive", Kind::Boolean )
      .with_description( "Scan subdirectories recursively (0 or 1)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "include_pattern", Kind::String )
      .with_description( "Include files matching glob pattern" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "exclude_pattern", Kind::String )
      .with_description( "Exclude files matching glob pattern" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::archive::from_directory_handler ) )?;
  Ok( () )
}
