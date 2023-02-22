pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,

    Command, RawCommand,

    TryCast,
    Value,
    ca::grammar::settings::ValueDescription,
  };

  use former::Former;
  use wtools::{ HashMap, Result, err };

  /// Grammatically correct command
  #[ derive( Debug ) ]
  pub struct GrammarCommand
  {
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Command subjects.
    pub subjects : Vec< Value >,
    /// Command options.
    pub properties : HashMap< String, Value >,
  }

  /// Converts from RawCommand to ExecutableCommand
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct GrammarConverter
  {
    #[ setter( false ) ]
    pub( crate ) commands : HashMap< String, Vec< Command > >,
  }

  impl GrammarConverterFormer
  {
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.commands.unwrap_or_default();

      let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( command );

      self.commands = Some( commands );
      self
    }

    pub fn commands< V >( mut self, commands : V ) -> Self
    where
      V : Into< Vec< Command > >
    {
      let mut self_commands = self.commands.unwrap_or_default();

      for command in commands.into()
      {
        let command_variants = self_commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
        command_variants.push( command );
      }

      self.commands = Some( self_commands );
      self
    }
  }

  impl GrammarConverter
  {
    /// Converts raw program to executable
    pub fn to_program( &self, raw_program : Program< Namespace< RawCommand > > ) -> Result< Program< Namespace< GrammarCommand > > >
    {
      let namespaces = raw_program.namespaces
      .into_iter()
      .map( | n | self.to_namespace( n ) )
      .collect::< Result< Vec< Namespace< GrammarCommand > > > >()?;

      Ok( Program { namespaces } )
    }

    /// Converts raw namespace to executable
    pub fn to_namespace( &self, raw_namespace : Namespace< RawCommand > ) -> Result< Namespace< GrammarCommand > >
    {
      let commands = raw_namespace.commands
      .into_iter()
      .map( | c | self.to_command( c ) )
      .collect::< Result< Vec< GrammarCommand > > >()?;

      Ok( Namespace { commands } )
    }

    /// Converts raw command to executable
    pub fn to_command( &self, raw_command : RawCommand ) -> Result< GrammarCommand >
    {
      self.commands
      .get( &raw_command.name )
      .and_then
      (
        | cmds |
        // find needed command
        // if it will be needed:
        // find command where number raw_command.subjects more or equal a command.subjects
        // and add trailing subjects as "Trailing" somehow
        cmds.iter().find( | cmd | cmd.subjects.len() == raw_command.subjects.len() )
      )
      .ok_or_else( || err!( "Command not found" ) )
      .and_then
      (
        | cmd |
        {
          let subjects = raw_command.subjects
          .into_iter()
          .zip( cmd.subjects.iter() )
          // an error can be extended with the value's hint
          .map( |( x, ValueDescription { kind, .. } )| kind.try_cast( x ) )
          .collect::< Result< Vec< _ > > >()?;

          let properties = raw_command.properties
          .into_iter()
          .map
          (
            |( key, value )|
            // find a key
            if cmd.properties.contains_key( &key ) { Ok( key ) }
            else { cmd.properties_aliases.get( &key ).cloned().ok_or_else( || err!( "`{}` not found", key ) ) }
            // give a description
            .map( | key | ( key.clone(), cmd.properties.get( &key ).unwrap(), value ) )
          )
          .collect::< Result< Vec< _ > > >()?
          .into_iter()
          // an error can be extended with the value's hint
          .map
          (
            |( key, value_description, value )|
            value_description.kind.try_cast( value ).map( | v | ( key.clone(), v ) )
          )
          .collect::< Result< HashMap< _, _ > > >()?;

          Ok( GrammarCommand
          {
            phrase : cmd.phrase.to_owned(),
            subjects,
            properties,
          })
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
