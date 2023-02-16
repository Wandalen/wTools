pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,
    Command, RawCommand, ExecutableCommand, Routine,
    Args, Props, Context
  };

  use former::Former;
  use wtools::{ HashMap, Result };

  /// Converts from RawCommand to ExecutableCommand
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct Converter
  {
    commands : HashMap< String, Vec<( Command, Routine )> >,
  }

  impl ConverterFormer
  {
    pub fn command< F >( mut self, command : Command, callback: F ) -> Self
    where
      F : Fn(( Args, Props )) -> Result< () > + 'static,
    {
      let mut commands = self.commands.unwrap_or_default();
      let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( || vec![] );

      command_variants.push(( command, Routine::new( callback ) ));

      self.commands = Some( commands );
      self
    }

    pub fn command_with_ctx< F >( mut self, command : Command, callback: F ) -> Self
    where
      F : Fn( ( Args, Props ), Context ) -> Result< () > + 'static,
    {
      let mut commands = self.commands.unwrap_or_default();
      let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( || vec![] );

      command_variants.push(( command, Routine::new_with_ctx( callback ) ));

      self.commands = Some( commands );
      self
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
      .get( &raw_command.name )
      .and_then( | cmds |
        // find needed command
        cmds.iter().find( |( cmd, _ )| cmd.subjects_hints.len() == raw_command.subjects.len() )
      )
      .map(
        |( cmd, routine )|
        ExecutableCommand
        {
          subjects : raw_command.subjects,
          properties : raw_command.properties.into_iter().filter( |( key, _ )| cmd.properties_hints.contains_key( key ) ).collect(),  
          routine : routine.clone(),
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
