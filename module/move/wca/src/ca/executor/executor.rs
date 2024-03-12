pub( crate ) mod private
{
  use crate::*;

  use ca::executor::runtime::_exec_command;
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
    /// Setup runtimes for each namespace into program and run it with specified execution type
    pub fn program( &self, dictionary : &Dictionary, program : Program< VerifiedCommand > ) -> Result< () >
    {
      let context = self.context.clone();
      let runtime = Runtime
      {
        context,
        pos : 0,
        namespace : program.commands,
      };

      Self::sequential_execution_loop( dictionary, runtime )?;

      Ok( () )
    }

    /// Executes a command
    ///
    /// Call command callback with context if it is necessary.
    pub fn command( &self, dictionary : &Dictionary, command : VerifiedCommand ) -> Result< () >
    {
      let routine = dictionary.command( &command.phrase ).unwrap().routine.clone();
      _exec_command( command, routine, self.context.clone() )
    }

    // qqq : for Bohdan : probably redundant
    // aaa : removed `parallel_execution_loop`

    fn sequential_execution_loop( dictionary : &Dictionary, mut runtime : Runtime ) -> Result< () >
    {
      while !runtime.is_finished()
      {
        let state = runtime.context.get_or_default::< RuntimeState >();
        state.pos = runtime.pos + 1;
        runtime.r#do( &dictionary )?;
        runtime.pos = runtime.context.get_ref::< RuntimeState >().unwrap().pos;
      }

      Ok( () )
    }
  }
}

//

crate::mod_interface!
{
  prelude use Executor;
}
