//!
//! The interpreter for the Unilang framework.
//!

use crate::semantic::VerifiedCommand;
use crate::data::OutputData;
use crate::error::Error;

///
/// The execution context for a command.
///
/// This struct holds all the necessary information for a command to be
/// executed, such as global arguments, configuration, and I/O streams.
#[ derive( Debug, Default ) ]
pub struct ExecutionContext
{
  // Placeholder for future context data
}

///
/// The interpreter for Unilang commands.
///
/// This struct takes a list of verified commands and executes them sequentially.
#[ derive( Debug ) ]
pub struct Interpreter< 'a >
{
  commands : &'a [ VerifiedCommand ],
}

impl< 'a > Interpreter< 'a >
{
  ///
  /// Creates a new `Interpreter`.
  ///
  pub fn new( commands : &'a [ VerifiedCommand ] ) -> Self
  {
    Self { commands }
  }

  ///
  /// Runs the commands and returns a list of outputs or an error.
  ///
  /// This method iterates through the verified commands and, for now,
  /// simulates their execution by printing them.
  pub fn run( &self, _context : &mut ExecutionContext ) -> Result< Vec< OutputData >, Error >
  {
    let mut results = Vec::new();
    for command in self.commands
    {
      // For now, just print the command to simulate execution
      println!( "Executing: {:?}", command );
      results.push( OutputData {
        content : format!( "Successfully executed command: {}", command.definition.name ),
        format : "text".to_string(),
      } );
    }
    Ok( results )
  }
}