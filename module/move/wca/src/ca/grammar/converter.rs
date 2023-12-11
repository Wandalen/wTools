pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,

    Command, RawCommand,

    TryCast,
    Value,
    ca::grammar::settings::ValueDescription, wtools,
  };

  use former::Former;
  use std::collections::HashMap;
  use wtools::{ error::Result, err };
  use crate::wtools::error;

  /// Represents a grammatically correct command with a phrase descriptor, a list of command subjects, and a set of command options.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ GrammarCommand, Value };
  /// # use std::collections::HashMap;
  /// GrammarCommand
  /// {
  ///   phrase : "command".to_string(),
  ///   subjects : vec![ Value::String( "subject_value".to_string() ), /* ... */ ],
  ///   properties : HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), Value::Number( 42.0 ) ),
  ///     /* ... */
  ///   ])
  /// };
  /// ```
  ///
  /// In the above example, a `GrammarCommand` instance is created with the name "command", a single subject "subject_value", and one property "prop_name" with a typed values.
  ///
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

  // TODO: Remove Clone
  /// Converts a `RawCommand` to a `GrammarCommand` by performing validation and type casting on values.
  ///
  /// ```
  /// # use wca::{ Command, Type, GrammarConverter, RawCommand };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let grammar = GrammarConverter::former()
  /// .command
  /// (
  ///   Command::former()
  ///   .hint( "hint" )
  ///   .long_hint( "long_hint" )
  ///   .phrase( "command" )
  ///   .form()
  /// )
  /// .form();
  ///
  /// let raw_command = RawCommand
  /// {
  ///   name: "command".to_string(),
  ///   subjects: vec![],
  ///   properties: HashMap::new(),
  /// };
  ///
  /// let grammar_command = grammar.to_command( raw_command )?;
  /// # Ok( () ) }
  /// ```
  #[ derive( Debug, Clone ) ]
  #[ derive( Former ) ]
  pub struct GrammarConverter
  {
    // TODO: Make getters
    /// all available commands
    #[ setter( false ) ]
    pub commands : HashMap< String, Vec< Command > >,
  }

  impl GrammarConverterFormer
  {
    /// Insert a command to the commands list
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.commands.unwrap_or_default();

      let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( command );

      self.commands = Some( commands );
      self
    }

    /// Expands the list of commands with received commands
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
    /// Converts raw program to grammatically correct
    ///
    /// Converts all namespaces into it with `to_namespace` method.
    pub fn to_program( &self, raw_program : Program< Namespace< RawCommand > > ) -> Result< Program< Namespace< GrammarCommand > > >
    {
      let namespaces = raw_program.namespaces
      .into_iter()
      .map( | n | self.to_namespace( n ) )
      .collect::< Result< Vec< Namespace< GrammarCommand > > > >()?;

      Ok( Program { namespaces } )
    }

    /// Converts raw namespace to grammatically correct
    ///
    /// Converts all commands into it with `to_command` method.
    pub fn to_namespace( &self, raw_namespace : Namespace< RawCommand > ) -> Result< Namespace< GrammarCommand > >
    {
      let commands = raw_namespace.commands
      .into_iter()
      .map( | c | self.to_command( c ) )
      .collect::< Result< Vec< GrammarCommand > > >()?;

      Ok( Namespace { commands } )
    }

    /// Converts raw command to grammatically correct
    ///
    /// Make sure that this command is described in the grammar and matches it(command itself and all it options too).
    pub fn to_command( &self, raw_command : RawCommand ) -> Result< GrammarCommand >
    {
      let variants = self
      .commands
      .get( &raw_command.name )
      .ok_or_else::< error::for_app::Error, _ >( || err!( "Command not found. Got `{:?}`", raw_command ) )?;

      let mut cmd = None;
      let mut subjects = vec![];

      // find a variant that meets requirements
      'variants_loop: for variant in variants
      {
        subjects.clear();

        if raw_command.subjects.len() > variant.subjects.len() { continue; }
        let mut rc_subjects_iter = raw_command.subjects.iter();

        let mut current = rc_subjects_iter.next();
        // try to match subjects
        'internal: for ValueDescription { kind, optional, .. } in &variant.subjects
        {
          let value = match current.and_then( | v | kind.try_cast( v.clone() ).ok() )
          {
            Some( v ) => v,
            None if *optional => continue 'internal,
            _ => continue 'variants_loop,
          };

          subjects.push( value );
          current = rc_subjects_iter.next();
        }
        // if something exists after all expected subjects - this isn't correct variant
        if current.is_some() { continue 'variants_loop; }

        cmd = Some( variant );
      }
      let Some( cmd ) = cmd else
      {
        error::for_app::bail!( "Command with specified subjects not found. Got `{:?}`", raw_command );
      };

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
  }
}

//

crate::mod_interface!
{
  prelude use GrammarConverter;
  prelude use GrammarCommand;
}
