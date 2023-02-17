pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,

    Command, RawCommand,
  };

  use former::Former;
  use wtools::HashMap;

  /// Grammatically correct command
  #[ derive( Debug ) ]
  pub struct GrammarCommand
  {
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Command subjects.
    /// TODO: Types
    pub subjects : Vec< String >,
    /// Command options.
    /// TODO: Types
    pub properties : HashMap< String, String >,
  }

  /// Converts from RawCommand to ExecutableCommand
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct GrammarConverter
  {
    commands : HashMap< String, Vec< Command > >,
  }

  impl GrammarConverterFormer
  {
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.commands.unwrap_or_default();
      let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( || vec![] );

      command_variants.push( command );

      self.commands = Some( commands );
      self
    }
  }

  impl GrammarConverter
  {
    /// Converts raw program to executable
    pub fn to_program( &self, raw_program : Program< Namespace< RawCommand > > ) -> Program< Namespace< GrammarCommand > >
    {
      let namespaces = raw_program.namespaces.into_iter().map( | n | self.to_namespace( n ) ).collect();
      Program { namespaces }
    }

    /// Converts raw namespace to executable
    pub fn to_namespace( &self, raw_namespace : Namespace< RawCommand > ) -> Namespace< GrammarCommand >
    {
      let commands = raw_namespace.commands.into_iter().filter_map( | c | self.to_command( c ) ).collect();
      Namespace { commands }
    }

    /// Converts raw command to executable
    pub fn to_command( &self, raw_command : RawCommand ) -> Option< GrammarCommand >
    {
      self.commands
      .get( &raw_command.name )
      .and_then( | cmds |
        // find needed command
        cmds.iter().find( | cmd | cmd.subjects_hints.len() == raw_command.subjects.len() )
      )
      .map(
        | cmd |
        GrammarCommand
        {
          phrase : cmd.phrase.to_owned(),
          subjects : raw_command.subjects,
          properties : raw_command.properties.into_iter().filter( |( key, _ )| cmd.properties_hints.contains_key( key ) ).collect(),  
        }
      )
    }
  }
}

//

crate::mod_interface!
{
  prelude use GrammarConverter;
  prelude use GrammarCommand;
}
