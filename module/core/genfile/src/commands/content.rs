//! Content management commands
//!
//! Commands for managing content sources: internalize, externalize, and list.

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
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".content.internalize" ).expect( "valid command name" ),
    "Convert all external content references (file/URL) to inline content for portability".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "content".to_string(), "internalize".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".content.internalize".to_string(),
    ".content.internalize verbosity::2".to_string(),
    ".content.internalize dry::true".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "dry", Kind::Boolean )
      .with_description( "Dry run mode (0 or 1)" )
      .with_optional( Some( "0" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::content::internalize_handler ) )?;
  Ok( () )
}

/// Register .content.externalize command
#[ allow( deprecated ) ]
fn register_externalize( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".content.externalize" ).expect( "valid command name" ),
    "Convert inline content to external file references for lightweight archives".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "content".to_string(), "externalize".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( false )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".content.externalize base_path::./content".to_string(),
    ".content.externalize base_path::/tmp/archive-content verbosity::2".to_string(),
    ".content.externalize base_path::./content dry::true".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "base_path", Kind::Path )
      .with_description( "Directory where content files will be written" ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
    ArgumentDefinition::new( "dry", Kind::Boolean )
      .with_description( "Dry run mode (0 or 1)" )
      .with_optional( Some( "0" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::content::externalize_handler ) )?;
  Ok( () )
}

/// Register .content.list command
#[ allow( deprecated ) ]
fn register_list( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".content.list" ).expect( "valid command name" ),
    "List all content sources in the archive grouped by type".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "content".to_string(), "list".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".content.list".to_string(),
    ".content.list filter::file".to_string(),
    ".content.list filter::inline verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "filter", Kind::String )
      .with_description( "Filter by content type: inline, file, url, or all (default: all)" )
      .with_optional( None::< &str > ),
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0=silent, 1=normal, 2=verbose)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::content::list_handler ) )?;
  Ok( () )
}
