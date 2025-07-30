//!
//! The interpreter for the Unilang framework.
//!

use crate::data::{ ErrorData, OutputData };
use crate::error::Error;
use crate::semantic::VerifiedCommand;

///
/// The execution context for a command.
///
/// This struct holds all the necessary information for a command to be
/// executed, such as global arguments, configuration, and I/O streams.
#[ derive( Debug, Default, Clone ) ]
pub struct ExecutionContext
{
  // Placeholder for future context data
}

///
/// The interpreter for Unilang commands.
///
/// This struct takes a list of verified commands and executes them sequentially.
#[ derive() ]
#[ allow( missing_debug_implementations ) ]
pub struct Interpreter< 'a >
{
  commands : & 'a [ VerifiedCommand ],
  // The interpreter needs access to the registry to get the routines
  // xxx: This should probably be a reference to the registry, not a direct copy of commands.
  // For now, we'll assume the VerifiedCommand contains enough info to find the routine.
  // Or, the commands should be paired with their routines.
  // This means the Interpreter needs a reference to the registry.
  registry : & 'a crate::registry::CommandRegistry,
}

impl< 'a > Interpreter< 'a >
{
  ///
  /// Creates a new `Interpreter`.
  ///
  #[ must_use ]
  pub fn new
  (
    commands : & 'a [ VerifiedCommand ],
    registry : & 'a crate::registry::CommandRegistry,
  )
  ->
  Self
  {
    Self { commands, registry }
  }

  ///
  /// Runs the commands and returns a list of outputs or an error.
  ///
  /// This method currently does not return errors directly from command execution,
  /// but it is designed to propagate `Error` from command routines in future implementations.
  pub fn run
  (
    &self,
    context : &mut ExecutionContext,
  )
  ->
  Result< Vec< OutputData >, Error >
  {
    let mut results = Vec::new();
    for command in self.commands
    {
      // For now, just print the command to simulate execution
      // println!( "Executing: {command:?}" );

      // Look up the routine from the registry
      let full_command_name = if command.definition.namespace.is_empty()
      {
        format!( ".{}", command.definition.name )
      }
      else
      {
        let ns = &command.definition.namespace;
        if ns.starts_with( '.' )
        {
          format!( "{}.{}", ns, command.definition.name )
        }
        else
        {
          format!( ".{}.{}", ns, command.definition.name )
        }
      };
      let routine = self.registry.get_routine( &full_command_name ).ok_or_else( ||
      {
        Error::Execution( ErrorData
        {
          code : "UNILANG_INTERNAL_ERROR".to_string(),
          message : format!( "Routine not found for command: {}", command.definition.name ),
        })
      })?;

      // Execute the routine
      let output_or_error = routine( command.clone(), context.clone() ); // Clone command and context for routine

      match output_or_error
      {
        Ok( output ) => results.push( output ),
        Err( error_data ) => return Err( Error::Execution( error_data ) ), // Stop on first error
      }
    }
    Ok( results )
  }
}
