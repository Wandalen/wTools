//!
//! The help generation components for the Unilang framework.
//!

use crate::data::CommandDefinition;

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
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Generates a help string for a single command.
  ///
  /// The output is a formatted string containing the command's usage,
  /// description, and a list of its arguments.
  pub fn command( &self, command : &CommandDefinition ) -> String
  {
    let mut help = String::new();
    help.push_str( &format!( "Usage: {}\n", command.name ) );
    help.push_str( &format!( "\n  {}\n", command.description ) );

    if !command.arguments.is_empty()
    {
      help.push_str( "\nArguments:\n" );
      for arg in &command.arguments
      {
        help.push_str( &format!( "  {:<15} {}\n", arg.name, arg.description ) );
      }
    }

    help
  }
}