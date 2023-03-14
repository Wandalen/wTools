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
        // TODO: RuntimeState instead usize
        runtime.context.insert( runtime.pos + 1 );
        runtime.r#do()?;
        runtime.pos = *runtime.context.get_ref::< usize >().unwrap();
      }

      Ok( () )
    }

    /// executes a command
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
          // TODO: RuntimeState instead usize
          runtime.context.insert( runtime.pos + 1 );
          runtime.r#do()?;
          runtime.pos = *runtime.context.get_ref::< usize >().unwrap();
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
          // TODO: RuntimeState instead usize
          runtime.context.insert( runtime.pos + 1 );
          runtime.r#do()?;
          runtime.pos = *runtime.context.get_ref::< usize >().unwrap();
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
