//! Archive command handlers
//!
//! Implementation of archive lifecycle operations.

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use genfile_core::TemplateArchive;


/// Use shared state for current archive
use super::shared_state::{ get_current_archive, set_current_archive };

/// Handler for .archive.new command
///
/// Creates a new empty template archive with the given name and description.
pub fn new_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  // Extract arguments
  let name = cmd.get_string( "name" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: name" ) )?;
  let description = cmd.get_string( "description" ).unwrap_or( "" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Create new archive
  let mut archive = TemplateArchive::new( name );
  if !description.is_empty()
  {
    archive.set_description( description );
  }

  // Store in thread-local state
  set_current_archive( archive );

  // Format output based on verbosity
  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Created archive: {name}" ),
    _ => format!( "Created archive: {name}\nDescription: {description}" ),
  };

  Ok( OutputData
  {
    content : output,
    format : "text".to_string(),
    execution_time_ms : None,
  } )
}

/// Handler for .archive.load command
///
/// Loads an archive from a JSON or YAML file.
pub fn load_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  // Extract arguments
  let path = cmd.get_path( "path" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: path" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Load archive from file
  let path_buf = path;
  let archive = TemplateArchive::load_from_file( path_buf )
    .map_err( | e | crate::error::format_error( e, "ARCHIVE" ) )?;

  let archive_name = archive.name.clone();
  let file_count = archive.file_count();

  // Store in thread-local state
  set_current_archive( archive );

  // Format output based on verbosity
  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Loaded archive: {archive_name}" ),
    _ =>
    {
      format!(
        "Loaded archive: {}\nPath: {}\nFiles: {}",
        archive_name, path_buf.display(), file_count
      )
    }
  };

  Ok( OutputData
  {
    content : output,
    format : "text".to_string(),
    execution_time_ms : None,
  } )
}

/// Handler for .archive.save command
///
/// Saves the current archive to a JSON or YAML file.
pub fn save_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  // Extract arguments
  let path = cmd.get_path( "path" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: path" ) )?;
  let format = cmd.get_string( "format" ).unwrap_or( "json" );
  let pretty = cmd.get_boolean( "pretty" ).unwrap_or( true );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let dry = cmd.get_boolean( "dry" ).unwrap_or( false );

  // Get current archive
  let archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded. Use .archive.new or .archive.load first" ) )?;

  let path_buf = path;

  // Use the format as-is (already have explicit or default "json")
  let final_format = format;

  if dry
  {
    let output = if verbosity == 0
    {
      String::new()
    }
    else
    {
      format!( "Dry run: Would save archive to {} (format: {})", path.display(), final_format )
    };

    return Ok( OutputData
    {
      content : output,
      format : "text".to_string(),
      execution_time_ms : None,
    } );
  }

  // Save archive to file
  // TODO: Implement format-specific saving once genfile_core supports it
  archive.save_to_file( path_buf )
    .map_err( | e | crate::error::format_error( e, "ARCHIVE" ) )?;

  // Format output based on verbosity
  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Saved archive to: {}", path.display() ),
    _ =>
    {
      format!(
        "Saved archive: {}\nPath: {}\nFormat: {}\nPretty: {}",
        archive.name, path.display(), final_format, pretty
      )
    }
  };

  Ok( OutputData
  {
    content : output,
    format : "text".to_string(),
    execution_time_ms : None,
  } )
}

/// Handler for .`archive.from_directory` command
///
/// Creates an archive from a filesystem directory.
pub fn from_directory_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  // Extract arguments
  let source = cmd.get_path( "source" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: source" ) )?;
  let mode = cmd.get_string( "mode" ).unwrap_or( "reference" );
  let recursive = cmd.get_boolean( "recursive" ).unwrap_or( true );
  let include_pattern = cmd.get_string( "include_pattern" );
  let exclude_pattern = cmd.get_string( "exclude_pattern" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let source_path = source;

  // Check if source directory exists
  if !source_path.exists()
  {
    return Err( crate::error::file_error( format!( "Source directory not found: {}", source_path.display() ) ) );
  }

  if !source_path.is_dir()
  {
    return Err( crate::error::file_error( format!( "Source path is not a directory: {}", source_path.display() ) ) );
  }

  // Create archive using pack_from_dir
  // Determine archive name from directory
  let archive_name = source_path
    .file_name()
    .and_then( | n | n.to_str() )
    .unwrap_or( "archive" );

  // TODO: Implement mode selection (inline vs reference) when genfile_core supports it
  let archive = TemplateArchive::pack_from_dir( archive_name, source_path )
    .map_err( | e | crate::error::format_error( e, "ARCHIVE" ) )?;

  let file_count = archive.file_count();

  // Store in thread-local state
  set_current_archive( archive );

  // Format output based on verbosity
  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Created archive from directory: {} ({} files)", source_path.display(), file_count ),
    _ =>
    {
      format!(
        "Created archive from directory\n\
        Source: {}\n\
        Mode: {}\n\
        Recursive: {}\n\
        Files: {}\n\
        Include pattern: {}\n\
        Exclude pattern: {}",
        source_path.display(),
        mode,
        recursive,
        file_count,
        include_pattern.unwrap_or( "none" ),
        exclude_pattern.unwrap_or( "none" )
      )
    }
  };

  Ok( OutputData
  {
    content : output,
    format : "text".to_string(),
    execution_time_ms : None,
  } )
}
