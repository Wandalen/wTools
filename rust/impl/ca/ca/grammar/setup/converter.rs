pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,
    Command, RawCommand, ExecutableCommand,
  };

  /// Converts from RawCommand to ExecutableCommand
  #[ derive( Debug ) ]
  pub struct Converter
  {
    commands : Vec< Command >,
  }

  impl From< Vec< Command > > for Converter
  {
    fn from( commands : Vec< Command > ) -> Self
    {
      Self { commands }
    }
  }

  impl Converter
  {
    /// Converts raw program to executable
    pub fn to_program( &self, raw_program : Program< Namespace< RawCommand > > ) -> Program< Namespace< ExecutableCommand > >
    {
      let namespaces = raw_program.namespaces.into_iter().map( | n | self.to_namespace( n ) ).collect();
      Program { namespaces }
    }

    /// Converts raw namespace to executable
    pub fn to_namespace( &self, raw_namespace : Namespace< RawCommand > ) -> Namespace< ExecutableCommand >
    {
      let commands = raw_namespace.commands.into_iter().filter_map( | c | self.to_command( c ) ).collect();
      Namespace { commands }
    }

    /// Converts raw command to executable
    pub fn to_command( &self, raw_command : RawCommand ) -> Option< ExecutableCommand >
    {
      self.commands
      .iter()
      .find( | &cmd | cmd.phrase == raw_command.name )
      .map
      (
        | cmd |
        ExecutableCommand
        {
          subjects : raw_command.subjects.into_iter().take( cmd.subjects_hint.len() ).collect(),
          properties : raw_command.properties.into_iter().filter( |( key, _ )| cmd.properties_hints.contains_key( key ) ).collect(),  
          routine : cmd.routine.clone(),
        }
      )
    }
  }
}

//

crate::mod_interface!
{
  prelude use Converter;
}