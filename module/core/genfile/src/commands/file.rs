//! File management commands
//!
//! Commands for adding, removing, and inspecting files in archives.

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".file.add" ).expect( "valid command name" ),
    "Add file to current archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "file".to_string(), "add".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".file.add path::src/main.rs content::\"fn main() {}\"".to_string(),
    ".file.add path::readme.md from_file::/tmp/template.md".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "path", Kind::Path )
      .with_description( "File path within archive" ),
    ArgumentDefinition::new( "content", Kind::String )
      .with_description( "File content (text)" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "from_file", Kind::Path )
      .with_description( "Source file to read content from" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "write_mode", Kind::Enum( vec![ "rewrite".to_string(), "append".to_string(), "skip".to_string() ] ) )
      .with_description( "Write mode (rewrite, append, skip)" )
      .with_optional( Some( "rewrite" ) ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::add_handler ) )?;
  Ok( () )
}

/// Register .file.remove command
#[ allow( deprecated ) ]
fn register_remove( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".file.remove" ).expect( "valid command name" ),
    "Remove file from current archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "file".to_string(), "remove".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".file.remove path::src/old.rs".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "path", Kind::Path )
      .with_description( "File path to remove" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::remove_handler ) )?;
  Ok( () )
}

/// Register .file.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".file.list" ).expect( "valid command name" ),
    "List all files in current archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "file".to_string(), "list".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".file.list".to_string(),
    ".file.list verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::list_handler ) )?;
  Ok( () )
}

/// Register .file.show command
#[ allow( deprecated ) ]
fn register_show( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".file.show" ).expect( "valid command name" ),
    "Show file content from archive".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "file".to_string(), "show".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".file.show path::src/main.rs".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "path", Kind::Path )
      .with_description( "File path to show" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::file::show_handler ) )?;
  Ok( () )
}
