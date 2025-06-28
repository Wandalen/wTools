//!
//! The help generation components for the Unilang framework.
//!

use crate::data::CommandDefinition;
use core::fmt::Write; // Changed from std::fmt::Write

///
/// Generates help information for commands.
///
/// This struct provides methods to create formatted help messages from
/// `CommandDefinition` instances, which can be displayed to the user.
#[ derive( Debug, Default ) ]
pub struct HelpGenerator;

impl HelpGenerator
{
  ///
  /// Creates a new `HelpGenerator`.
  ///
  #[must_use]
  pub fn new() -> Self
  {
    Self {}
  }

  ///
  /// Generates a help string for a single command.
  ///
  /// The output is a formatted string containing the command's usage,
  /// description, and a list of its arguments.
  #[must_use]
  pub fn command( &self, command : &CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "Usage: {}", command.name ).unwrap(); // Changed to writeln!
    writeln!( &mut help, "\n  {}\n", command.description ).unwrap(); // Changed to writeln!

    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in &command.arguments
      {
        writeln!( &mut help, "  {:<15} {}", arg.name, arg.description ).unwrap(); // Changed to writeln!
      }
    }

    help
  }
}