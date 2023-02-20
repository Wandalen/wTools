pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace, ExecutableCommand,

    Context,
    Runtime, Routine,
    ca::executor::runtime::_exec_command, 
  };

  use wtools::{ Result, HashMap };

  #[ derive( Debug ) ]
  /// TODO: THINK
  pub enum ExecutorType
  {
    /// Separate context for each namespaces
    ResetsContext,
    /// single context for all namespaces
    Simple,
  }

  #[ derive( Debug, former::Former ) ]
  /// TODO: THINK
  pub struct Executor
  {
    /// represent how executor will work
    #[ default( ExecutorType::Simple ) ]
    pub kind : ExecutorType,
    /// default context
    #[ default( Context::default() ) ]
    pub context : Context,
    /// commands routins
    pub commands : HashMap< String, Routine >
  }

  impl ExecutorFormer
  {
    pub fn command< S : Into< String > >( mut self, phrase : S, routine : Routine ) -> Self
    {
      let mut commands = self.commands.unwrap_or_default();
      commands.insert( phrase.into(), routine );

      self.commands = Some( commands );
      self
    }
  }

  impl Executor
  {
    /// executes a program
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
          let context = match self.kind {
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

      Self::sequential_execution_loop( runtimes )?;

      Ok( () )
    }

    /// executes a namespace
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
        runtime.r#do()?
      }

      Ok( () )
    }

    /// executes a command
    pub fn command( &self, command : ExecutableCommand ) -> Result< () >
    {
      _exec_command( command, self.context.clone() )
    }

    fn sequential_execution_loop( mut runtimes : Vec< Runtime > ) -> Result< () >
    {
      while
      {
        // iteration
        for runtime in runtimes.iter_mut()
        {
          // Thoughts on Improvements
          // * unlock after get commands
          // 1) let commands = { global_ctx.lock().unwrap().get_commands() };
          // 2) run_commands( commands )
          // * somehow lock only current namespace
          // 2 - into command) global_ctx.lock().unwrap().change_pos( 8 );
          runtime.r#do()?;
        }
        !runtimes.is_empty()
      }
      {
        // remove finished
        runtimes = runtimes.into_iter().filter( | r | !r.is_finished() ).collect::< Vec< _ > >();
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
