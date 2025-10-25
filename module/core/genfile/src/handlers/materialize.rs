//! Materialize command handler
//!
//! Renders template archives to destination directories with parameter substitution.
//! Implements FR6: Template Materialization.
//!
//! ## Design Decisions
//!
//! **Mandatory Parameter Validation:**
//! Validates all mandatory parameters have values before materialization to prevent
//! partial/broken output. This ensures output quality and clear error messages.
//!
//! **`MaterializationReport` Usage:**
//! Uses `genfile_core`'s `MaterializationReport` to provide detailed feedback about
//! files created, updated, or skipped during materialization.
//!
//! **Dry Run Safety:**
//! Dry run mode previews what would be done without creating files, preventing
//! accidental overwrites. Critical for user confidence.

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use core::fmt::Write as _;

/// Handler for .materialize command
///
/// Renders template archive to destination directory with parameter substitution.
///
/// # Parameters
/// - `destination` - Output directory path
/// - `verbosity` - Output verbosity (0-5, default: 1)
/// - `dry` - Dry run mode (default: 0)
pub fn materialize_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  // Extract arguments
  let destination = cmd.get_path( "destination" )
    .ok_or_else( || crate::error::usage_error( "Missing required parameter: destination" ) )?;
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );
  let dry = cmd.get_boolean( "dry" ).unwrap_or( false );

  // Get loaded archive from shared state
  let archive = crate::handlers::shared_state::get_current_archive()
    .ok_or_else( || crate::error::state_error( "No archive loaded. Use .archive.load first." ) )?;

  // Validate mandatory parameters have values
  let mandatory_params = archive.parameters.list_mandatory();
  if !mandatory_params.is_empty()
  {
    let missing : Vec< &str > = mandatory_params
      .iter()
      .filter( | p |
      {
        archive
          .values
          .as_ref()
          .is_none_or( | v | !v.has_value( p ) )
      })
      .copied()
      .collect();

    if !missing.is_empty()
    {
      return Err( crate::error::validation_error( format!(
        "Missing mandatory parameter values: {}. Use .value.set to provide values.",
        missing.join( ", " )
      ) ) );
    }
  }

  // Dry run preview
  if dry
  {
    let file_count = archive.files.len();
    let param_count = archive.values.as_ref().map_or( 0, genfile_core::Values::len );

    let output_content = match verbosity
    {
      0 => String::new(),
      1 => format!( "Dry run: Would materialize {} files to {}", file_count, destination.display() ),
      _ =>
      {
        let files_preview = archive
          .files
          .iter()
          .take( 5 )
          .map( | f | format!( "  - {}", f.path.display() ) )
          .collect::< Vec< _ > >()
          .join( "\n" );

        let more = if file_count > 5 { format!( "\n  ... and {} more files", file_count - 5 ) } else { String::new() };

        format!(
          "Dry run: Would materialize templates\\n\\\n          Destination: {}\\n\\\n          Archive: {}\\n\\\n          Files: {}\\n\\\n          Parameters: {}\\n\\\n          Files to create:\\n\\\n          {}{}",
          destination.display(),
          archive.name,
          file_count,
          param_count,
          files_preview,
          more
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

  // Materialize templates to destination
  let report = archive.materialize( destination )
    .map_err( | e | crate::error::format_error( e, "MATERIALIZE" ) )?;

  let total_files = report.files_created.len() + report.files_updated.len() + report.files_skipped.len();

  // Format output based on verbosity
  let output_content = match verbosity
  {
    0 => String::new(),
    1 => format!( "Materialized {} files to {}", total_files, destination.display() ),
    _ =>
    {
      let mut details = format!(
        "Materialized templates\\n\\\n        Archive: {}\\n\\\n        Destination: {}\\n\\\n        Created: {}\\n\\\n        Updated: {}\\n\\\n        Skipped: {}",
        archive.name,
        destination.display(),
        report.files_created.len(),
        report.files_updated.len(),
        report.files_skipped.len()
      );

      if verbosity >= 3 && !report.files_created.is_empty()
      {
        details.push_str( "\\n\\nCreated files:\\n" );
        for file in &report.files_created
        {
          let _ = write!( &mut details, "  - {}\\n", file.display() );
        }
      }

      if verbosity >= 3 && !report.files_updated.is_empty()
      {
        details.push_str( "\\nUpdated files:\\n" );
        for file in &report.files_updated
        {
          let _ = write!( &mut details, "  - {}\\n", file.display() );
        }
      }

      details
    }
  };

  Ok( OutputData
  {
    content : output_content,
    format : "text".to_string(),
    execution_time_ms : None,
  } )
}
