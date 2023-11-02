pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace, ExecutableCommand,

    Context,
    RuntimeState, Runtime,
    ca::executor::runtime::_exec_command,
  };

  use werror::Result;

  /// Represents the type of executor to use for running commands.
  #[ derive( Debug ) ]
  pub enum ExecutorType
  {
    /// The executor will create a new context for each namespace
    ResetsContext,
    /// The executor will use a single context for all namespaces
    Simple,
  }

  /// Executor that is responsible for executing the program's commands.
  /// It uses the given `Context` to store and retrieve values during runtime.
  ///
  /// It takes an `ExecutableCommand` which contains subjects and properties that will be passed to the callback function of the associated command's routine.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ Executor, ExecutableCommand, Routine, Value };
  /// # use std::collections::HashMap;
  /// let executor = Executor::former().form();
  ///
  /// let executable_command = ExecutableCommand
  /// {
  ///   subjects : vec![ Value::String( "subject_value".to_string() ), /* ... */ ],
  ///   properties : HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), Value::Number( 42.0 ) ),
  ///     /* ... */
  ///   ]),
  ///   routine : Routine::new( |( args, props )| Ok( () ) )
  /// };
  ///
  /// assert!( executor.command( executable_command ).is_ok() );
  /// ```
  ///
  #[ derive( Debug, former::Former ) ]
  pub struct Executor
  {
    /// Represents how the executor will work
    #[ default( ExecutorType::Simple ) ]
    pub kind : ExecutorType,
    /// The default context for the executor
    #[ default( Context::default() ) ]
    pub context : Context,
  }

  impl Executor
  {
    /// Executes a program
    ///
    /// Setup runtimes for each namespace into program and run it with specified execution type
    pub fn program( &self, program : Program< Namespace< ExecutableCommand > > ) -> Result< () >
    {
      let context = self.context.clone();
      let runtimes_number = program.namespaces.len();
      let runtimes = program.namespaces
      .into_iter()
      .fold
      (
        Vec::with_capacity( runtimes_number ),
        | mut acc, namespace |
        {
          // local context for each namespace
          let context = match self.kind
          {
            ExecutorType::ResetsContext => context.deep_clone(),
            ExecutorType::Simple => context.clone(),
          };
          let runtime = Runtime
          {
            context,
            pos : 0,
            namespace,
          };
          acc.push( runtime );
          acc
        }
      );

      match self.kind
      {
        ExecutorType::ResetsContext => Self::parallel_execution_loop( runtimes )?,
        ExecutorType::Simple => Self::sequential_execution_loop( runtimes )?,
      }

      Ok( () )
    }

    /// Executes a namespace
    ///
    /// Configure `Runtime` and run commands from namespace at runtime position while it isn't finished
    pub fn namespace( &self, namespace : Namespace< ExecutableCommand > ) -> Result< () >
    {
      let context = self.context.clone();
      let mut runtime = Runtime
      {
        context,
        pos : 0,
        namespace,
      };

      while !runtime.is_finished()
      {
        let state = runtime.context.get_or_default::< RuntimeState >();
        state.pos = runtime.pos + 1;
        runtime.r#do()?;
        runtime.pos = runtime.context.get_ref::< RuntimeState >().unwrap().pos;
      }

      Ok( () )
    }

    /// Executes a command
    ///
    /// Call command callback with context if it is necessary.
    pub fn command( &self, command : ExecutableCommand ) -> Result< () >
    {
      _exec_command( command, self.context.clone() )
    }

    fn parallel_execution_loop( mut runtimes : Vec< Runtime > ) -> Result< () >
    {
      while
      {
        // iteration
        for runtime in runtimes.iter_mut()
        {
          let state = runtime.context.get_or_default::< RuntimeState >();
          state.pos = runtime.pos + 1;
          runtime.r#do()?;
          runtime.pos = runtime.context.get_ref::< RuntimeState >().unwrap().pos;
        }
        !runtimes.is_empty()
      }
      {
        // remove finished
        runtimes = runtimes.into_iter().filter( | r | !r.is_finished() ).collect::< Vec< _ > >();
      }

      Ok( () )
    }

    fn sequential_execution_loop( runtimes : Vec< Runtime > ) -> Result< () >
    {
      for mut runtime in runtimes
      {
        while !runtime.is_finished()
        {
          let state = runtime.context.get_or_default::< RuntimeState >();
          state.pos = runtime.pos + 1;
          runtime.r#do()?;
          runtime.pos = runtime.context.get_ref::< RuntimeState >().unwrap().pos;
        }
      }

      Ok( () )
    }
  }
}

//

crate::mod_interface!
{
  prelude use Executor;
  prelude use ExecutorType;
}
