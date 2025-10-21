//! Content command handlers
//!
//! Implementation of content management operations (internalize, externalize, list)

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use genfile_core::DefaultContentResolver;
/// Use shared state for current archive
use super::shared_state::{ get_current_archive, set_current_archive };

/// Handler for .content.internalize command
pub fn internalize_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let dry = cmd.get_boolean( "dry" ).unwrap_or( false );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Count files with external references before internalization
  let external_count = archive.files.iter()
    .filter( | f | f.content_source.is_some() )
    .count();

  if dry
  {
    let output = match verbosity
    {
      0 => String::new(),
      1 => format!( "Would internalize {external_count} external reference(s)" ),
      _ => format!( "Dry run: Would internalize {external_count} external reference(s)\nNo changes made" ),
    };
    return Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } );
  }

  // Perform internalization
  let resolver = DefaultContentResolver::new();
  archive.internalize( &resolver )
    .map_err( | e | crate::error::format_error( e, "CONTENT" ) )?;

  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Internalized {external_count} external reference(s)" ),
    _ => format!( "Internalized {external_count} external reference(s)\nAll content is now inline" ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .content.externalize command
pub fn externalize_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let base_path = cmd.get_path( "base_path" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: base_path" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let dry = cmd.get_boolean( "dry" ).unwrap_or( false );

  let mut archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Count files with inline content before externalization
  let inline_count = archive.files.iter()
    .filter( | f | f.content_source.is_none() )
    .count();

  if dry
  {
    let output = match verbosity
    {
      0 => String::new(),
      1 => format!( "Would externalize {} file(s) to {}", inline_count, base_path.display() ),
      _ => format!( "Dry run: Would externalize {} file(s)\nContent would be written to: {}\nNo changes made", inline_count, base_path.display() ),
    };
    return Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } );
  }

  // Perform externalization
  archive.externalize( base_path )
    .map_err( | e | crate::error::format_error( e, "CONTENT" ) )?;

  set_current_archive( archive );

  let output = match verbosity
  {
    0 => String::new(),
    1 => format!( "Externalized {} file(s) to {}", inline_count, base_path.display() ),
    _ => format!( "Externalized {} file(s)\nContent written to: {}", inline_count, base_path.display() ),
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}

/// Handler for .content.list command
pub fn list_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let filter = cmd.get_string( "filter" );
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  let archive = get_current_archive()
    .ok_or_else( || crate::error::usage_error( "No archive loaded" ) )?;

  // Categorize files by content source type
  let mut inline_files = Vec::new();
  let mut file_refs = Vec::new();
  let mut url_refs = Vec::new();

  for file in &archive.files
  {
    match &file.content_source
    {
      None => inline_files.push( &file.path ),
      Some( source ) => match source
      {
        genfile_core::ContentSource::Inline { .. } => inline_files.push( &file.path ),
        genfile_core::ContentSource::File { path } =>
        {
          file_refs.push( ( &file.path, path ) );
        }
        genfile_core::ContentSource::Url { url } =>
        {
          url_refs.push( ( &file.path, url ) );
        }
      },
    }
  }

  // Apply filter
  let filter_str = filter.unwrap_or( "all" );
  let mut output_lines = Vec::new();

  match filter_str
  {
    "inline" | "all" =>
    {
      if filter_str == "all" || !inline_files.is_empty()
      {
        output_lines.push( format!( "Inline content ({}):", inline_files.len() ) );
        for path in &inline_files
        {
          output_lines.push( format!( "  {}", path.display() ) );
        }
      }
    }
    _ => {}
  }

  match filter_str
  {
    "file" | "all" =>
    {
      if filter_str == "all" || !file_refs.is_empty()
      {
        if !output_lines.is_empty()
        {
          output_lines.push( String::new() );
        }
        output_lines.push( format!( "File references ({}):", file_refs.len() ) );
        for ( path, ref_path ) in &file_refs
        {
          if verbosity >= 2
          {
            output_lines.push( format!( "  {} → {}", path.display(), ref_path.display() ) );
          }
          else
          {
            output_lines.push( format!( "  {}", path.display() ) );
          }
        }
      }
    }
    _ => {}
  }

  match filter_str
  {
    "url" | "all" =>
    {
      if filter_str == "all" || !url_refs.is_empty()
      {
        if !output_lines.is_empty()
        {
          output_lines.push( String::new() );
        }
        output_lines.push( format!( "URL references ({}):", url_refs.len() ) );
        for ( path, url ) in &url_refs
        {
          if verbosity >= 2
          {
            output_lines.push( format!( "  {} → {}", path.display(), url ) );
          }
          else
          {
            output_lines.push( format!( "  {}", path.display() ) );
          }
        }
      }
    }
    _ => {}
  }

  let output = if output_lines.is_empty()
  {
    "No files in archive".to_string()
  }
  else
  {
    output_lines.join( "\n" )
  };

  Ok( OutputData { content : output, format : "text".to_string(), execution_time_ms : None } )
}
