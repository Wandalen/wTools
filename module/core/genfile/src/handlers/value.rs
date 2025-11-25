//! Value command handlers
//!
//! Implementation of value management operations (set, list, clear)

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use genfile_core::Value;
/// Use shared state for current archive
use super::shared_state::{ get_current_archive, set_current_archive };

/// Handler for .value.set command
pub fn set_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let name = cmd.get_string( "name" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: name" ) )?;
  let value_str = cmd.get_string( "value" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: value" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Parse value as string (simplest approach - all values stored as strings)
  let value = Value::String( value_str.to_string() );

  // Set the value in the archive's values collection
  archive.values_mut().insert( name, value );
  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Set value: {name} = {value_str}" ),
    _ => format!( "Set value:\n  Parameter: {name}\n  Value: {value_str}" ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .value.list command
pub fn list_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Access values if they exist
  let values = match &archive.values
  {
    Some( v ) => v,
    None =>
    {
      return Ok( OutputData
      {
        content : "No values set".to_string(),
        format : "text".to_string(),
        execution_time_ms : None,
      } );
    }
  };

  // Count values
  let value_count = values.len();

  let output = if value_count == 0
  {
    "No values set".to_string()
  }
  else
  {
    let mut lines = vec![ format!( "Values ({}):", value_count ) ];

    // Get serializable representation to iterate
    let serialized = values.to_serializable();
    for ( key, value ) in &serialized
    {
      if verbosity == 1
      {
        lines.push( format!( "  {key} = {value}" ) );
      }
      else if verbosity >= 2
      {
        lines.push( format!( "  {key}" ) );
        lines.push( format!( "    Value: {value}" ) );
      }
    }

    lines.join( "\n" )
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .value.clear command
pub fn clear_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Count values before clearing
  let value_count = archive.values.as_ref().map_or( 0, genfile_core::Values::len );

  // Clear all values
  archive.clear_values();
  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Cleared {value_count} value(s)" ),
    _ => format!( "Cleared all values\n  Count: {value_count}" ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}
