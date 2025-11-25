//! File command handlers
//!
//! Implementation of file operations (add, remove, list, show)

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use genfile_core::{ FileContent, WriteMode };
/// Use shared state for current archive
use super::shared_state::{ get_current_archive, set_current_archive };

/// Handler for .file.add command
pub fn add_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let path = cmd.get_path( "path" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: path" ) )?;
  let content = cmd.get_string( "content" );
  let from_file = cmd.get_path( "from_file" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  let file_content = if let Some( content_str ) = content
  {
    FileContent::Text( content_str.to_string() )
  }
  else if let Some( source_path ) = from_file
  {
    if !source_path.exists()
    {
      return Err( crate::error::file_error( format!( "Source file not found: {}", source_path.display() ) ) );
    }
    let data = std::fs::read( source_path )
      .map_err( | e | crate::error::file_error( format!( "Failed to read: {e}" ) ) )?;
    match String::from_utf8( data.clone() )
    {
      Ok( text ) => FileContent::Text( text ),
      Err( _ ) => FileContent::Binary( data ),
    }
  }
  else
  {
    return Err( crate::error::usage_error( "Either 'content' or 'from_file' required" ) );
  };

  let size = match &file_content
  {
    FileContent::Text( s ) => s.len(),
    FileContent::Binary( b ) => b.len(),
  };

  let path_buf = path.to_path_buf();
  archive.add_file( path_buf, file_content, WriteMode::Rewrite );
  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Added file: {}", path.display() ),
    _ => format!( "Added file: {}\nSize: {} bytes", path.display(), size ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .file.remove command
pub fn remove_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let path = cmd.get_path( "path" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: path" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  if !archive.has_file( path )
  {
    return Err( crate::error::file_error( format!( "File not found: {}", path.display() ) ) );
  }

  archive.remove_file( path );
  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    _ => format!( "Removed file: {}", path.display() ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .file.list command
pub fn list_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let _verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  let files = archive.list_files();
  let output = if files.is_empty()
  {
    "No files in archive".to_string()
  }
  else
  {
    let mut lines = vec![ format!( "Files ({}):", files.len() ) ];
    for file_path in files
    {
      lines.push( format!( "  {}", file_path.display() ) );
    }
    lines.join( "\n" )
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .file.show command
pub fn show_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let path = cmd.get_path( "path" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: path" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  let file = archive.get_file( path )
    .ok_or_else( || crate::error::file_error( format!( "File not found: {}", path.display() ) ) )?;

  let output = match verbosity
  {
    0 => match &file.content
    {
      FileContent::Text( s ) => s.clone(),
      FileContent::Binary( _ ) => "[Binary]".to_string(),
    },
    _ => match &file.content
    {
      FileContent::Text( s ) => format!( "=== {} ===\n{}", path.display(), s ),
      FileContent::Binary( b ) => format!( "[Binary: {} bytes]", b.len() ),
    },
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}
