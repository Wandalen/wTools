pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,

    GrammarCommand, ExecutableCommand,

    Routine, 
  };

  use former::Former;
  use wtools::HashMap;

  /// Converts from RawCommand to ExecutableCommand
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct ExecutorConverter
  {
    routines : HashMap< String, Routine >,
  }

  impl ExecutorConverterFormer
  {
    pub fn routine< S >( mut self, phrase : S, routine : Routine ) -> Self
    where
      S : Into< String >,
      Routine : Into< Routine >
    {
      let mut routines = self.routines.unwrap_or_default();

      routines.insert( phrase.into(), routine );

      self.routines = Some( routines );
      self
    }
  }

  impl ExecutorConverter
  {
    /// Converts raw program to executable
    pub fn to_program( &self, raw_program : Program< Namespace< GrammarCommand > > ) -> Program< Namespace< ExecutableCommand > >
    {
      let namespaces = raw_program.namespaces.into_iter().map( | n | self.to_namespace( n ) ).collect();
      Program { namespaces }
    }

    /// Converts raw namespace to executable
    pub fn to_namespace( &self, raw_namespace : Namespace< GrammarCommand > ) -> Namespace< ExecutableCommand >
    {
      let commands = raw_namespace.commands.into_iter().filter_map( | c | self.to_command( c ) ).collect();
      Namespace { commands }
    }

    /// Converts raw command to executable
    pub fn to_command( &self, command : GrammarCommand ) -> Option< ExecutableCommand >
    {
      self.routines
      .get( &command.phrase )
      .map(
        | routine |
        ExecutableCommand
        {
          subjects : command.subjects,
          properties : command.properties,
          routine : routine.clone(),
        }
      )
    }
  }
}

//

crate::mod_interface!
{
  prelude use ExecutorConverter;
}
