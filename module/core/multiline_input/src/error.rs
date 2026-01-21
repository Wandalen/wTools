//! Error types for multiline input collection

use error_tools::dependency::thiserror;

/// Errors that can occur during multiline input collection
#[ derive( thiserror::Error, Debug ) ]
pub enum Error
{
  /// Terminal I/O error
  #[ error( "Terminal I/O error: {0}" ) ]
  Io( #[ from ] std::io::Error ),

  /// Not running in a terminal (no TTY)
  #[ error( "Not running in a terminal (no TTY)" ) ]
  NoTty,

  /// Validation failed with custom message
  #[ error( "Validation failed: {0}" ) ]
  ValidationFailed( String ),

  /// Terminal size too small for rendering
  #[ error( "Terminal too small ({width}x{height}, need at least {min_width}x{min_height})" ) ]
  TerminalTooSmall
  {
    /// Current terminal width
    width: u16,
    /// Current terminal height
    height: u16,
    /// Minimum required width
    min_width: u16,
    /// Minimum required height
    min_height: u16,
  },
}
