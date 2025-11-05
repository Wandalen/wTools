//! Analysis and information command definitions - FR8: Archive Analysis
//!
//! Commands for inspecting and understanding template archives:
//! - `.info` - Archive metadata and statistics
//! - `.discover.parameters` - Auto-detect template parameters
//! - `.status` - Archive readiness check
//! - `.analyze` - Comprehensive analysis

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

/// Register analysis commands
#[ allow( deprecated ) ]
pub fn register( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  register_info( registry )?;
  register_discover_parameters( registry )?;
  register_status( registry )?;
  register_analyze( registry )?;
  Ok( () )
}

/// Register .info command
#[ allow( deprecated ) ]
fn register_info( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".info" ).expect( "valid command name" ),
    "Display archive metadata and statistics".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "info".to_string(), "metadata".to_string(), "inspect".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".info".to_string(),
    ".info verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::info_handler ) )?;
  Ok( () )
}

/// Register .discover.parameters command
#[ allow( deprecated ) ]
fn register_discover_parameters( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".discover.parameters" ).expect( "valid command name" ),
    "Auto-detect template parameters in archive files".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "discover".to_string(), "parameters".to_string(), "detect".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".discover.parameters".to_string(),
    ".discover.parameters verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::discover_parameters_handler ) )?;
  Ok( () )
}

/// Register .status command
#[ allow( deprecated ) ]
fn register_status( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".status" ).expect( "valid command name" ),
    "Show archive readiness and completeness status".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "status".to_string(), "readiness".to_string(), "check".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".status".to_string(),
    ".status verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::status_handler ) )?;
  Ok( () )
}

/// Register .analyze command
#[ allow( deprecated ) ]
fn register_analyze( registry : &mut CommandRegistry ) -> Result< (), Box< dyn core::error::Error > >
{
  let cmd = CommandDefinition::new
  (
    CommandName::new( ".analyze" ).expect( "valid command name" ),
    "Comprehensive archive analysis including all insights".to_string(),
  )
  .with_namespace( String::new() )
  .with_status( CommandStatus::Active )
  .with_version( VersionType::new( "0.1.0" ).expect( "valid version" ) )
  .with_tags( vec![ "analyze".to_string(), "inspect".to_string(), "comprehensive".to_string() ] )
  .with_aliases( vec![] )
  .with_permissions( vec![] )
  .with_idempotent( true )
  .with_deprecation_message( "" )
  .with_http_method_hint( "" )
  .with_examples( vec!
  [
    ".analyze".to_string(),
    ".analyze verbosity::2".to_string(),
  ] )
  .with_auto_help( true )
  .with_arguments( vec!
  [
    ArgumentDefinition::new( "verbosity", Kind::Integer )
      .with_description( "Output verbosity level (0-5)" )
      .with_optional( Some( "1" ) ),
  ] );

  registry.command_add_runtime( &cmd, Box::new( crate::handlers::analysis::analyze_handler ) )?;
  Ok( () )
}
