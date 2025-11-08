//!
//! The interpreter for the Unilang framework.
//!

/// Internal namespace.
mod private
{
  use crate::data::{ ErrorData, ErrorCode, OutputData };
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
  /// Reference to the registry for accessing command routines
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
  -> Self
  {
    Self { commands, registry }
  }

  ///
  /// Runs the commands and returns a list of outputs or an error.
  ///
  /// This method currently does not return errors directly from command execution,
  /// but it is designed to propagate `Error` from command routines in future implementations.
  ///
  /// ## Performance Monitoring
  ///
  /// Each command's execution time is automatically captured and included in the
  /// `OutputData` result via the `execution_time_ms` field. This allows for
  /// performance tracking and optimization of command routines.
  #[allow(clippy::missing_errors_doc)]
  pub fn run
  (
    &self,
    context : &mut ExecutionContext,
  )
  -> Result< Vec< OutputData >, Error >
  {
    let mut results = Vec::new();
    for command in self.commands
    {
      // For now, just print the command to simulate execution
      // println!( "Executing: {command:?}" );

      // EXPLICIT COMMAND NAMING (FR-REG-6): Use command names exactly as registered
      // Following the governing principle: minimum implicit magic!
      // Command names are now required to have dot prefixes and are used as-is
      let full_command_name = command.definition.full_name();

      eprintln!( "[PANIC TEST 2] About to execute command: {}", full_command_name );

      // Special handling for .help - generate dynamic help using HelpGenerator
      // instead of using the broken mandatory help routine with hardcoded text
      if full_command_name == ".help"
      {
        panic!( "[PANIC TEST] .help intercepted in interpreter!" );
      }

      let routine = self.registry.get_routine( &full_command_name ).ok_or_else( ||
      {
        Error::Execution( ErrorData::new(
          ErrorCode::InternalError,
          format!( "Internal Error: No executable routine found for command '{}'. This is a system error, please report it.", command.definition.name().as_str() ),
        ))
      })?;

      // Capture execution timing
      let start_time = std::time::Instant::now();
      let output_or_error = routine( command.clone(), context.clone() ); // Clone command and context for routine
      let execution_time_ms = start_time.elapsed().as_millis() as u64;

      match output_or_error
      {
        Ok( mut output ) =>
        {
          // Add execution timing to output
          output.execution_time_ms = Some( execution_time_ms );
          results.push( output );
        },
        Err( error_data ) => return Err( Error::Execution( error_data ) ), // Stop on first error
      }
    }
    Ok( results )
  }
}

}

mod_interface::mod_interface!
{
  exposed use private::ExecutionContext;
  exposed use private::Interpreter;
  
  prelude use private::ExecutionContext;
  prelude use private::Interpreter;
}
