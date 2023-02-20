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
    commands : HashMap< String, Vec< Command > >,
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
      .and_then( | cmds |
        // find needed command
        cmds.iter().find( | cmd | cmd.subjects.len() == raw_command.subjects.len() )
      )
      .ok_or_else( || err!( "Command not found" ) )
      .and_then(
        | cmd |
        {
          let subjects = raw_command.subjects
          .into_iter()
          .zip( cmd.subjects.iter() )
          .map( |( x, ValueDescription { kind, .. } )| kind.try_cast( x ) )
          .collect::< Vec< _ > >();
          if let Some( Err( err ) ) = subjects.iter().find( | x | x.is_err() )
          {
            return Err( err!( "At command `{}` in subjects got error: `{}`", cmd.phrase, err.msg() ) );
          }

          let properties = raw_command.properties
          .into_iter()
          .filter( |( key, _ )| cmd.properties.contains_key( key ) )
          .map( |( key, value )| ( key.clone(), cmd.properties[ &key ].kind.try_cast( value ) ))
          .collect::< Vec< _ > >();

          if let Some( pos ) = properties.iter().map( |( _, prop )| prop ).position( | prop | prop.is_err() )
          {
            if let ( name, Err( error ) ) = properties[ pos ].to_owned()
            {
              return Err( err!( "At command `{}` in property `{name}` got error: `{}`", cmd.phrase, error.msg() ) );
            }
          }

          Ok( GrammarCommand
          {
            phrase : cmd.phrase.to_owned(),
            subjects : subjects.into_iter().map( | x | x.unwrap() ).collect(),
            properties  : properties.into_iter().map( |( key, value )| ( key, value.unwrap() ) ).collect()
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
