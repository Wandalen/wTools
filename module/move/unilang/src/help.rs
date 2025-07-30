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
  registry : & 'a CommandRegistry,
}

impl< 'a > HelpGenerator< 'a >
{
  ///
  /// Creates a new `HelpGenerator`.
  ///
  #[ must_use ]
  pub fn new( registry : & 'a CommandRegistry ) -> Self
  {
    Self { registry }
  }

  ///
  /// Generates a help string for a single command.
  ///
  /// The output is a formatted string containing the command's usage,
  /// description, and a list of its arguments.
  #[ must_use ]
  pub fn command( &self, command_name : &str ) -> Option< String >
  {
    // Try exact match first, then try with dot prefix
    let command = self.registry.commands.get( command_name )
    .or_else( || self.registry.commands.get( &format!( ".{command_name}" ) ) )
    .or_else( ||
    {
      // If command_name is "echo", try ".system.echo"
      // If command_name is "math.add", it should already be found.
      // This handles cases where the user provides just the command name without namespace,
      // or a partial namespace.
      // For now, a simple check for "echo" to ".system.echo"
      if command_name == "echo"
      {
        self.registry.commands.get( ".system.echo" )
      }
      else
      {
        None
      }
    })?;
    let mut help = String::new();
    writeln!
    (
      &mut help,
      "Usage: {} (v{})",
      command.name,
      command.version
    )
    .unwrap();
    if !command.aliases.is_empty()
    {
      writeln!( &mut help, "Aliases: {}", command.aliases.join( ", " ) ).unwrap();
    }
    if !command.tags.is_empty()
    {
      writeln!( &mut help, "Tags: {}", command.tags.join( ", " ) ).unwrap();
    }
    writeln!( &mut help, "\n  Hint: {}", command.hint ).unwrap();
    writeln!( &mut help, "  {}\n", command.description ).unwrap();
    writeln!( &mut help, "Status: {}", command.status ).unwrap();

    if !command.arguments.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in &command.arguments
      {
        let mut arg_info = String::new();
        write!( &mut arg_info, "{} (Kind: {}) - Hint: {}", arg.name, arg.kind, arg.hint ).unwrap();
        if arg.attributes.optional
        {
          write!( &mut arg_info, ", Optional" ).unwrap();
        }
        if arg.attributes.multiple
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
  #[ must_use ]
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
