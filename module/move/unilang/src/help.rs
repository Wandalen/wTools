//!
//! The help generation components for the Unilang framework.
//!

use crate::registry::CommandRegistry;
use core::fmt::Write;

///
/// Generates help information for commands.
///
/// This struct provides methods to create formatted help messages from
/// `CommandDefinition` instances, which can be displayed to the user.
#[ allow( missing_debug_implementations ) ]
pub struct HelpGenerator< 'a >
{
  registry : &'a CommandRegistry,
}

impl< 'a > HelpGenerator< 'a >
{
  ///
  /// Creates a new `HelpGenerator`.
  ///
  #[must_use]
  pub fn new( registry : &'a CommandRegistry ) -> Self
  {
    Self { registry }
  }

  ///
  /// Generates a help string for a single command.
  ///
  /// The output is a formatted string containing the command's usage,
  /// description, and a list of its arguments.
  #[must_use]
  pub fn command( &self, command_name : &str ) -> Option< String >
  {
    let command = self.registry.commands.get( command_name )?;
    let mut help = String::new();
    writeln!( &mut help, "Usage: {}", command.name ).unwrap();
    writeln!( &mut help, "\n  {}\n", command.description ).unwrap();

    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in &command.arguments
      {
        let mut arg_info = String::new();
        write!( &mut arg_info, "  {:<15} {}", arg.name, arg.description ).unwrap();
        write!( &mut arg_info, " (Kind: {})", arg.kind ).unwrap();
        if arg.optional
        {
          write!( &mut arg_info, ", Optional" ).unwrap();
        }
        if arg.multiple
        {
          write!( &mut arg_info, ", Multiple" ).unwrap();
        }
        if !arg.validation_rules.is_empty()
        {
          write!( &mut arg_info, ", Rules: [{}]", arg.validation_rules.join( ", " ) ).unwrap();
        }
        writeln!( &mut help, "{arg_info}" ).unwrap();
      }
    }

    Some( help )
  }

  ///
  /// Generates a summary list of all available commands.
  ///
  #[must_use]
  pub fn list_commands( &self ) -> String
  {
    let mut summary = String::new();
    writeln!( &mut summary, "Available Commands:" ).unwrap();
    for ( name, command ) in &self.registry.commands
    {
      writeln!( &mut summary, "  {:<15} {}", name, command.description ).unwrap();
    }
    summary
  }
}