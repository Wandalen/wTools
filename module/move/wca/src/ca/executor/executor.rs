pub( crate ) mod private
{
  use crate::*;

  use wtools::error::Result;

  // aaa : for Bohdan : how is it useful? where is it used?
  // aaa : `ExecutorType` has been removed


  /// Executor that is responsible for executing the program's commands.
  /// It uses the given `Context` to store and retrieve values during runtime.
  #[ derive( Debug, former::Former ) ]
  pub struct Executor
  {
    /// The default context for the executor
    #[ default( Context::default() ) ]
    pub context : Context,
  }

  impl Executor
  {
    /// Executes a program
    ///
    /// Iterates over the commands in the program and executes each command using the provided dictionary.
    /// This method returns a `Result` indicating whether the execution was successful or not.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - A reference to the dictionary used to look up the command routine.
    /// * `program` - The program to be executed, which is a `Program` object consisting of a list of commands.
    ///
    /// # Returns
    ///
    /// A `Result` with `Ok(())` if the execution was successful, or an `Err` containing an error message if an error occurred.
    ///
    pub fn program( &self, dictionary : &Dictionary, program : Program< VerifiedCommand > ) -> Result< () >
    {
      for command in program.commands
      {
        self.command( dictionary, command )?;
      }

      Ok( () )
    }

    /// Executes a given command using a provided dictionary and command.
    ///
    /// Calls the command callback with the given context if it is necessary.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - A reference to the dictionary used to look up the command routine.
    /// * `command` - The verified command that needs to be executed.
    ///
    /// # Returns
    ///
    /// Returns a Result indicating success or failure. If successful, returns `Ok(())`, otherwise returns an error.
    pub fn command( &self, dictionary : &Dictionary, command : VerifiedCommand ) -> Result< () >
    {
      let routine = dictionary.command( &command.phrase ).unwrap().routine.clone();
      _exec_command( command, routine, self.context.clone() )
    }
    
    // aaa : for Bohdan : probably redundant
    // aaa : removed `parallel_execution_loop`
  }
  
  fn _exec_command( command : VerifiedCommand, routine : Routine, ctx : Context ) -> Result< () >
  {
    match routine
    {
      Routine::WithoutContext( routine ) => routine(( Args( command.subjects ), Props( command.properties ) )),
      Routine::WithContext( routine ) => routine( ( Args( command.subjects ), Props( command.properties ) ), ctx ),
    }
  }
}

//

crate::mod_interface!
{
  prelude use Executor;
}
