//! Parameter command handlers
//!
//! Implementation of parameter management operations (add, list, remove)

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use genfile_core::ParameterDescriptor;
/// Use shared state for current archive
use super::shared_state::{ get_current_archive, set_current_archive };

/// Handler for .parameter.add command
pub fn add_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let name = cmd.get_string( "name" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: name" ) )?;
  let mandatory = cmd.get_boolean( "mandatory" ).unwrap_or( false );
  let default_value = cmd.get_string( "default" ).map( std::string::ToString::to_string );
  let description = cmd.get_string( "description" ).map( std::string::ToString::to_string );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Create parameter descriptor
  let param = ParameterDescriptor
  {
    parameter : name.to_string(),
    is_mandatory : mandatory,
    default_value : default_value.clone(),
    description : description.clone(),
  };

  // Add parameter to archive
  archive.add_parameter( param );
  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Added parameter: {name}" ),
    _ =>
    {
      let mut details = vec![ format!( "Added parameter: {}", name ) ];
      details.push( format!( "  Mandatory: {mandatory}" ) );
      if let Some( ref default ) = default_value
      {
        details.push( format!( "  Default: {default}" ) );
      }
      if let Some( ref desc ) = description
      {
        details.push( format!( "  Description: {desc}" ) );
      }
      details.join( "\n" )
    }
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .parameter.list command
pub fn list_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  let params = &archive.parameters.descriptors;

  let output = if params.is_empty()
  {
    "No parameters defined".to_string()
  }
  else
  {
    let mut lines = vec![ format!( "Parameters ({}):", params.len() ) ];

    for param in params
    {
      if verbosity == 1
      {
        let mandatory_marker = if param.is_mandatory { " *" } else { "" };
        lines.push( format!( "  {}{}", param.parameter, mandatory_marker ) );
      }
      else if verbosity >= 2
      {
        lines.push( format!( "  {}", param.parameter ) );
        lines.push( format!( "    Mandatory: {}", param.is_mandatory ) );
        if let Some( ref default ) = param.default_value
        {
          lines.push( format!( "    Default: {default}" ) );
        }
        if let Some( ref desc ) = param.description
        {
          lines.push( format!( "    Description: {desc}" ) );
        }
      }
    }

    lines.join( "\n" )
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .parameter.remove command
pub fn remove_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let name = cmd.get_string( "name" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: name" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Try to remove the parameter
  let removed = archive.remove_parameter( name );

  if removed.is_none()
  {
    return Err( crate::error::file_error( format!( "Parameter not found: {name}" ) ) );
  }

  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    _ => format!( "Removed parameter: {name}" ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}
