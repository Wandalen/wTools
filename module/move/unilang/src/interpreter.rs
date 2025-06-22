//!
//! The interpreter for the Unilang framework.
//!

use crate::semantic::VerifiedCommand;
use crate::data::OutputData;
use crate::error::Error;

///
/// The execution context for a command.
///
#[ derive( Debug, Default ) ]
pub struct ExecutionContext
{
  // Placeholder for future context data
}

///
/// The interpreter for Unilang commands.
///
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
  /// Runs the commands.
  ///
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