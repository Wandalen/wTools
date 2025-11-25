//! Analysis command handlers - FR8: Archive Analysis
//!
//! Provides inspection and analysis capabilities for template archives.
//!
//! ## Design Decisions
//!
//! **Idempotent Read-Only Operations:**
//! All analysis commands are read-only and idempotent - they never modify archive state.
//! This allows safe exploration without worrying about side effects.
//!
//! **`genfile_core` API Usage:**
//! - `archive.discover_parameters()` - Auto-detects {{}} placeholders via regex
//! - `archive.analyze_parameter_usage()` - Maps parameters to files using them
//! - `archive.file_count()`, `text_file_count()`, `binary_file_count()` - Statistics
//! - `archive.total_size()` - Size calculations
//!
//! **Verbosity Levels:**
//! - 0: Silent (for scripting)
//! - 1: Summary (default, user-friendly)
//! - 2+: Detailed (includes lists, breakdowns)

use unilang::semantic::VerifiedCommand;
use unilang::data::{ OutputData, ErrorData };
use unilang::interpreter::ExecutionContext;
use core::fmt::Write as _;

/// Handler for .info command
///
/// Displays archive metadata and statistics.
///
/// # Parameters
/// - `verbosity` - Output verbosity (0-5, default: 1)
pub fn info_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Get loaded archive from shared state
  let archive = crate::handlers::shared_state::get_current_archive()
    .ok_or_else( || crate::error::state_error( "No archive loaded. Use .archive.load first." ) )?;

  let file_count = archive.file_count();
  let text_count = archive.text_file_count();
  let binary_count = archive.binary_file_count();
  let total_size = archive.total_size();
  let param_count = archive.parameters.descriptors.len();

  // Format output based on verbosity
  let output_content = match verbosity
  {
    0 => String::new(),
    1 =>
    {
      format!(
        "Archive: {}\nFiles: {} ({} text, {} binary)\nSize: {} bytes\nParameters: {}",
        archive.name,
        file_count,
        text_count,
        binary_count,
        total_size,
        param_count
      )
    }
    _ =>
    {
      let mut details = format!(
        "Archive Information\n\
        ==================\n\
        Name: {}\n\
        Version: {}\n",
        archive.name,
        archive.version
      );

      if let Some( ref desc ) = archive.description
      {
        let _ = writeln!( &mut details, "Description: {desc}" );
      }

      let _ = write!(
        &mut details,
        "\nStatistics\n\
        ----------\n\
        Total Files: {file_count}\n\
        Text Files: {text_count}\n\
        Binary Files: {binary_count}\n\
        Total Size: {total_size} bytes\n\
        Parameters Defined: {param_count}\n"
      );

      if verbosity >= 3 && !archive.files.is_empty()
      {
        details.push_str( "\nFiles\n-----\n" );
        for file in &archive.files
        {
          let _ = writeln!( &mut details, "  {}", file.path.display() );
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

/// Handler for .discover.parameters command
///
/// Auto-detects template parameters in archive files.
/// Uses `genfile_core`'s regex-based {{variable}} detection.
///
/// # Parameters
/// - `verbosity` - Output verbosity (0-5, default: 1)
pub fn discover_parameters_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Get loaded archive from shared state
  let archive = crate::handlers::shared_state::get_current_archive()
    .ok_or_else( || crate::error::state_error( "No archive loaded. Use .archive.load first." ) )?;

  // Discover parameters using genfile_core
  let discovered = archive.discover_parameters();
  let param_count = discovered.len();

  // Convert HashSet to sorted Vec for consistent output
  let mut params : Vec< String > = discovered.into_iter().collect();
  params.sort();

  // Format output based on verbosity
  let output_content = match verbosity
  {
    0 => String::new(),
    1 =>
    {
      if param_count == 0
      {
        "No parameters discovered".to_string()
      }
      else
      {
        format!( "Discovered {} parameters: {}", param_count, params.join( ", " ) )
      }
    }
    _ =>
    {
      let mut details = format!(
        "Parameter Discovery\n\
        ===================\n\
        Found: {param_count} parameters\n"
      );

      if !params.is_empty()
      {
        details.push_str( "\nDiscovered Parameters\n---------------------\n" );
        for param in &params
        {
          let _ = writeln!( &mut details, "  - {param}" );
        }
      }

      if verbosity >= 3 && !params.is_empty()
      {
        // Show parameter usage across files
        let usage = archive.analyze_parameter_usage();
        details.push_str( "\nParameter Usage\n---------------\n" );
        for param in &params
        {
          if let Some( files ) = usage.get( param )
          {
            let _ = writeln!( &mut details, "  {} (used in {} files)", param, files.len() );
            if verbosity >= 4
            {
              for file in files
              {
                let _ = writeln!( &mut details, "    - {}", file.display() );
              }
            }
          }
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

/// Handler for .status command
///
/// Shows archive readiness and completeness status.
///
/// # Parameters
/// - `verbosity` - Output verbosity (0-5, default: 1)
pub fn status_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Get loaded archive from shared state
  let archive = crate::handlers::shared_state::get_current_archive()
    .ok_or_else( || crate::error::state_error( "No archive loaded. Use .archive.load first." ) )?;

  // Analyze readiness
  let mandatory_params = archive.parameters.list_mandatory();
  let defined_params = archive.parameters.descriptors.len();
  let set_values = archive.values.as_ref().map_or( 0, genfile_core::Values::len );

  // Check if ready to materialize
  let missing_mandatory : Vec< &str > = mandatory_params
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

  let ready = missing_mandatory.is_empty();

  // Format output based on verbosity
  let output_content = match verbosity
  {
    0 => String::new(),
    1 =>
    {
      if ready
      {
        format!( "Status: Ready to materialize ({set_values} parameters set)" )
      }
      else
      {
        format!(
          "Status: Not ready - {} mandatory parameters missing: {}",
          missing_mandatory.len(),
          missing_mandatory.join( ", " )
        )
      }
    }
    _ =>
    {
      let mut details = format!(
        "Archive Status\n\
        ==============\n\
        Archive: {}\n\
        Files: {}\n\
        Parameters Defined: {}\n\
        Values Set: {}\n\
        Mandatory Parameters: {}\n",
        archive.name,
        archive.file_count(),
        defined_params,
        set_values,
        mandatory_params.len()
      );

      if ready
      {
        details.push_str( "\nReadiness: ✓ Ready to materialize\n" );
      }
      else
      {
        details.push_str( "\nReadiness: ✗ Not ready\n" );
        details.push_str( "\nMissing Mandatory Values\n------------------------\n" );
        for param in &missing_mandatory
        {
          let _ = writeln!( &mut details, "  - {param}" );
        }
      }

      if verbosity >= 3 && defined_params > 0
      {
        details.push_str( "\nDefined Parameters\n------------------\n" );
        for param_desc in &archive.parameters.descriptors
        {
          let mandatory_marker = if param_desc.is_mandatory { " (mandatory)" } else { "" };
          let has_value = archive
            .values
            .as_ref()
            .is_some_and( | v | v.has_value( &param_desc.parameter ) );
          let value_marker = if has_value { " [set]" } else { "" };

          let _ = writeln!(
            &mut details,
            "  - {}{}{}",
            param_desc.parameter,
            mandatory_marker,
            value_marker
          );
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

/// Handler for .analyze command
///
/// Comprehensive archive analysis combining all insights.
///
/// # Parameters
/// - `verbosity` - Output verbosity (0-5, default: 1)
pub fn analyze_handler(
  cmd : VerifiedCommand,
  _ctx : ExecutionContext
) -> Result< OutputData, ErrorData >
{
  let verbosity = cmd.get_integer( "verbosity" ).unwrap_or( 1 );

  // Get loaded archive from shared state
  let archive = crate::handlers::shared_state::get_current_archive()
    .ok_or_else( || crate::error::state_error( "No archive loaded. Use .archive.load first." ) )?;

  // Gather all analysis data
  let file_count = archive.file_count();
  let text_count = archive.text_file_count();
  let binary_count = archive.binary_file_count();
  let total_size = archive.total_size();
  let discovered = archive.discover_parameters();
  let defined_params = archive.parameters.descriptors.len();
  let mandatory_params = archive.parameters.list_mandatory();
  let set_values = archive.values.as_ref().map_or( 0, genfile_core::Values::len );

  // Check readiness
  let missing_mandatory : Vec< &str > = mandatory_params
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

  let ready = missing_mandatory.is_empty();

  // Format output based on verbosity
  let output_content = match verbosity
  {
    0 => String::new(),
    1 =>
    {
      format!(
        "Archive Analysis Summary\n\
        Archive: {}\n\
        Files: {} ({} text, {} binary, {} bytes)\n\
        Parameters: {} defined, {} discovered\n\
        Status: {}",
        archive.name,
        file_count,
        text_count,
        binary_count,
        total_size,
        defined_params,
        discovered.len(),
        if ready { "Ready to materialize" } else { "Not ready (missing mandatory values)" }
      )
    }
    _ =>
    {
      let mut details = format!(
        "Comprehensive Archive Analysis\n\
        ==============================\n\
        \n\
        Archive Metadata\n\
        ----------------\n\
        Name: {}\n\
        Version: {}\n",
        archive.name,
        archive.version
      );

      if let Some( ref desc ) = archive.description
      {
        let _ = writeln!( &mut details, "Description: {desc}" );
      }

      let _ = write!(
        &mut details,
        "\nFile Statistics\n\
        ---------------\n\
        Total Files: {file_count}\n\
        Text Files: {text_count}\n\
        Binary Files: {binary_count}\n\
        Total Size: {total_size} bytes\n"
      );

      let _ = write!(
        &mut details,
        "\nParameter Analysis\n\
        ------------------\n\
        Discovered in Templates: {}\n\
        Defined: {}\n\
        Mandatory: {}\n\
        Values Set: {}\n",
        discovered.len(),
        defined_params,
        mandatory_params.len(),
        set_values
      );

      if ready
      {
        details.push_str( "\nReadiness Status\n----------------\n✓ Ready to materialize\n" );
      }
      else
      {
        let _ = write!(
          &mut details,
          "\nReadiness Status\n\
          ----------------\n\
          ✗ Not ready\n\
          Missing mandatory values: {}\n",
          missing_mandatory.join( ", " )
        );
      }

      if verbosity >= 3
      {
        let mut discovered_vec : Vec< String > = discovered.into_iter().collect();
        discovered_vec.sort();

        if !discovered_vec.is_empty()
        {
          details.push_str( "\nDiscovered Parameters\n---------------------\n" );
          for param in &discovered_vec
          {
            let _ = writeln!( &mut details, "  - {param}" );
          }
        }

        if !archive.files.is_empty()
        {
          details.push_str( "\nArchive Files\n-------------\n" );
          for file in &archive.files
          {
            let _ = writeln!( &mut details, "  {}", file.path.display() );
          }
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

