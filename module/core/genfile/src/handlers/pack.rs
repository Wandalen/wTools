//! Pack command handler
//!
//! Creates portable archives with all content internalized (inline mode).
//! Implements FR7: Archive Serialization.
//!
//! ## Design Decisions
//!
//! **Directory-Only Input:**
//! Originally considered supporting both directory and archive file inputs.
//! Simplified to directory-only because:
//! - Matches spec.md quick start example (lib.rs:13)
//! - `genfile_core::TemplateArchive::pack_from_dir()` automatically creates inline content
//! - Archive-to-archive internalization would require `ContentResolver` parameter complexity
//! - Users can achieve same result via: `.archive.load` → `.content.internalize` → `.archive.save`
//!
//! **API Choice:**
//! Uses `pack_from_dir()` instead of manual `new()` + `add_file()` loop because:
//! - Single operation = atomic, less error-prone
//! - Automatically handles recursive directory traversal
//! - Creates inline content by default (portability guarantee)
//! - Performance: Batched I/O operations
//!
//! **Portability Guarantee:**
//! Packed archives contain ALL file content inline (no external references).
//! This ensures archives can be:
//! - Transferred between systems
//! - Committed to version control
//! - Distributed without dependency on source directories

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use genfile_core::TemplateArchive;

/// Handler for .pack command
///
/// Creates a portable archive from a source directory or existing archive,
/// internalizing all file content for maximum portability.
///
/// # Parameters
/// - `input` - Source directory or archive path
/// - `output` - Output file path
/// - `verbosity` - Output verbosity (0-5, default: 1)
/// - `dry` - Dry run mode (default: 0)
pub fn pack_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  // Extract arguments
  let input = cmd.get_path( "input" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: input" ) )?;
  let output = cmd.get_path( "output" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: output" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let dry = cmd.get_boolean( "dry" ).unwrap_or( false );

  // Check if input exists and is a directory
  if !input.exists()
  {
    return Err( crate::error::file_error( format!( "Input path does not exist: {}", input.display() ) ) );
  }

  if !input.is_dir()
  {
    return Err( crate::error::file_error( format!( "Input must be a directory: {}", input.display() ) ) );
  }

  // Pack from directory - create archive with inline content
  let archive_name = input
    .file_name()
    .and_then( | n | n.to_str() )
    .unwrap_or( "archive" );

  let archive = TemplateArchive::pack_from_dir( archive_name, input )
    .map_err( | e | crate::error::format_error( e, "PACK" ) )?;

  let file_count = archive.file_count();
  let archive_name = archive.name.clone();

  // Dry run preview
  if dry
  {
    let output_content = match verbosity
    {
      0 => String::new(),
      1 => format!( "Dry run: Would pack to {}", output.display() ),
      _ =>
      {
        format!(
          "Dry run: Would pack archive\n\
          Input: {}\n\
          Output: {}\n\
          Archive: {}\n\
          Files: {}\n\
          Mode: inline (portable)",
          input.display(),
          output.display(),
          archive_name,
          file_count
        )
      }
    };

    return Ok( OutputData
    {
      content : output_content,
      format : "text".to_string(),
      execution_time_ms : None,
    } );
  }

  // Save archive to output file
  archive.save_to_file( output )
    .map_err( | e | crate::error::format_error( e, "PACK" ) )?;

  // Format output based on verbosity
  let output_content = match verbosity
  {
    0 => String::new(),
    1 => format!( "Packed archive to: {}", output.display() ),
    _ =>
    {
      format!(
        "Packed archive: {}\n\
        Input: {}\n\
        Output: {}\n\
        Files: {}\n\
        Mode: inline (portable)",
        archive_name,
        input.display(),
        output.display(),
        file_count
      )
    }
  };

  Ok( OutputData
  {
    content : output_content,
    format : "text".to_string(),
    execution_time_ms : None,
  } )
}
