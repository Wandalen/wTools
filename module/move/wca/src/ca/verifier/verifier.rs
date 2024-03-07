pub( crate ) mod private
{
  use crate::*;
  // use super::super::*;

  use ca::grammar::command::ValueDescription;
  use former::Former;
  use std::collections::HashMap;
  use wtools::{ error, error::Result, err };

  // TODO: Remove Clone
  /// Converts a `ParsedCommand` to a `VerifiedCommand` by performing validation and type casting on values.
  ///
  /// ```
  /// # use wca::{ Command, Type, Verifier, ParsedCommand };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > >
  /// # {
  /// let grammar = Verifier::former()
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
  /// let raw_command = ParsedCommand
  /// {
  ///   name: "command".to_string(),
  ///   subjects: vec![],
  ///   properties: HashMap::new(),
  /// };
  ///
  /// let grammar_command = grammar.to_command( raw_command )?;
  /// # Ok( () )
  /// # }
  /// ```
  #[ derive( Debug, Clone ) ]
  #[ derive( Former ) ]
  pub struct Verifier
  {
    // TODO: Make getters
    /// all available commands
    #[ setter( false ) ]
    pub commands : HashMap< String, Vec< Command > >, // qqq : for Bohdan : <- introduce Dictionary for HashMap< String, Vec< Command > >
  }

  impl VerifierFormer
  {
    /// Insert a command to the commands list
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.container.commands.unwrap_or_default();

      let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( command );

      self.container.commands = Some( commands );
      self
    }

    /// Expands the list of commands with received commands
    pub fn commands< V >( mut self, commands : V ) -> Self
    where
      V : Into< Vec< Command > >
    {
      let mut self_commands = self.container.commands.unwrap_or_default();

      for command in commands.into()
      {
        let command_variants = self_commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
        command_variants.push( command );
      }

      self.container.commands = Some( self_commands );
      self
    }
  }

  impl Verifier
  {
    /// Converts raw program to grammatically correct
    ///
    /// Converts all namespaces into it with `to_namespace` method.
    pub fn to_program( &self, raw_program : Program< Namespace< ParsedCommand > > )
    -> Result< Program< Namespace< VerifiedCommand > > >
    {
      let namespaces = raw_program.namespaces
      .into_iter()
      .map( | n | self.to_namespace( n ) )
      .collect::< Result< Vec< Namespace< VerifiedCommand > > > >()?;

      Ok( Program { namespaces } )
    }

    // qqq : for Bohdan : probably rdundant
    /// Converts raw namespace to grammatically correct
    ///
    /// Converts all commands into it with `to_command` method.
    pub fn to_namespace( &self, raw_namespace : Namespace< ParsedCommand > ) -> Result< Namespace< VerifiedCommand > >
    {
      let commands = raw_namespace.commands
      .into_iter()
      .map( | c | self.to_command( c ) )
      .collect::< Result< Vec< VerifiedCommand > > >()?;

      Ok( Namespace { commands } )
    }

    #[ cfg( feature = "on_unknown_suggest" ) ]
    fn suggest_command( &self, user_input: &str ) -> Option< &str >
    {
      let jaro = eddie::JaroWinkler::new();
      let sim = self
      .commands
      .iter()
      .map( |( name, c )| ( jaro.similarity( name, user_input ), c ) )
      .max_by( |( s1, _ ), ( s2, _ )| s1.total_cmp( s2 ) );
      if let Some(( sim, variant )) = sim
      {
        if sim > 0.0
        {
          let phrase = &variant[ 0 ].phrase;
          return Some( phrase );
        }
      }

      None
    }

    fn find_variant< 'a >
    (
      variants: &'a [ Command ],
      raw_command : &ParsedCommand,
    ) -> Option< &'a Command >
    {
      let mut maybe_valid_variants = vec![];

      for variant @ Command
      {
        subjects,
        properties,
        properties_aliases,
        ..
      }
      in variants
      {
        let raw_subjects_count = raw_command.subjects.len();
        let expected_subjects_count = subjects.len();
        if raw_subjects_count > expected_subjects_count { continue; }

        let mut maybe_subjects_count = 0_usize;
        for ( k, _v ) in &raw_command.properties
        {
          if properties.contains_key( k ) { continue; }
          if let Some( key ) = properties_aliases.get( k )
          {
            if properties.contains_key( key ) { continue; }
          }
          maybe_subjects_count += 1;
        }

        if raw_subjects_count + maybe_subjects_count > expected_subjects_count { continue; }

        maybe_valid_variants.push( variant );
      }

      // if maybe_valid_variants.len() == 1 { return Some( maybe_valid_variants[ 0 ] ) }
      // qqq: provide better variant selection( E.g. based on types )
      if !maybe_valid_variants.is_empty() { return Some( maybe_valid_variants[ 0 ] ) }
      else { None }
    }

    fn extract_subjects( command : &Command, raw_command : &ParsedCommand, used_properties : &[ &String ] ) -> Result< Vec< Value > >
    {
      let mut subjects = vec![];

      let all_subjects = raw_command
      .subjects.clone().into_iter()
      .chain
      (
        raw_command.properties.iter()
        .filter( |( key, _ )| !used_properties.contains( key ) )
        .map( |( key, value )| format!( "{key}:{value}" ) )
      )
      .collect::< Vec< _ > >();
      let mut rc_subjects_iter = all_subjects.iter();
      let mut current = rc_subjects_iter.next();

      for ValueDescription { kind, optional, .. } in &command.subjects
      {
        let value = match current.and_then( | v | kind.try_cast( v.clone() ).ok() )
        {
          Some( v ) => v,
          None if *optional => continue,
          _ => return Err( err!( "Missing not optional subject" ) ),
        };
        subjects.push( value );
        current = rc_subjects_iter.next();
      }
      if let Some( value ) = current { return Err( err!( "Can not identify a subject: `{}`", value ) ) }

      Ok( subjects )
    }

    fn extract_properties( command: &Command, raw_command : HashMap< String, String > ) -> Result< HashMap< String, Value > >
    {
      raw_command.into_iter()
      .filter_map
      (
        |( key, value )|
        // try to find a key
        if command.properties.contains_key( &key ) { Some( key ) }
        else if let Some( original_key ) = command.properties_aliases.get( &key ) { Some( original_key.clone() ) }
        else { None }
        // give a description. unwrap is safe because previous checks
        .map( | key | ( command.properties.get( &key ).unwrap(), key, value ) )
      )
      .map
      (
        |( value_description, key, value )|
        value_description.kind.try_cast( value ).map( | v | ( key.clone(), v ) )
      )
      .collect::< Result< HashMap< _, _ > > >()
    }

    fn group_properties_and_their_aliases< 'a, Ks >( aliases : &'a HashMap< String, String >, used_keys :  Ks ) -> Vec< &String >
    where
      Ks : Iterator< Item = &'a String >
    {
      let reverse_aliases =
      {
        let mut map = HashMap::< &String, Vec< &String > >::new();
        for ( property, alias ) in aliases
        {
          map.entry( alias ).or_default().push( property );
        }
        map
      };

      used_keys.flat_map( | key |
      {
        reverse_aliases.get( key ).into_iter().flatten().map( | k | *k ).chain( Some( key ) )
      })
      .collect::< Vec< _ > >()
    }

    /// Converts raw command to grammatically correct
    ///
    /// Make sure that this command is described in the grammar and matches it(command itself and all it options too).
    pub fn to_command( &self, raw_command : ParsedCommand ) -> Result< VerifiedCommand >
    {
      let variants = self.commands.get( &raw_command.name )
      .ok_or_else::< error::for_app::Error, _ >
      (
        ||
        {
          #[ cfg( feature = "on_unknown_suggest" ) ]
          if let Some( phrase ) = self.suggest_command( &raw_command.name )
          { return err!( "Command not found. Maybe you mean `.{}`?", phrase ) }
          err!( "Command not found. Please use `.` command to see the list of available commands." )
        }
      )?;

      let Some( cmd ) = Self::find_variant( variants, &raw_command ) else
      {
        error::for_app::bail!
        (
          "`{}` command with specified subjects not found. Available variants `{:#?}`",
          &raw_command.name,
          variants.iter()
          .map
          (
            | x |
            format!
            (
              ".{}{}",
              &raw_command.name,
              {
                let variants = x.subjects.iter().filter( | x | !x.optional ).map( | x | format!( "{:?}", x.kind ) ).collect::< Vec< _ > >();
                if variants.is_empty() { String::new() } else { variants.join( "" ) }
              }
            )
          )
          .collect::< Vec< _ > >()
        );
      };

      let properties = Self::extract_properties( cmd, raw_command.properties.clone() )?;
      let used_properties_with_their_aliases = Self::group_properties_and_their_aliases( &cmd.properties_aliases, properties.keys() );
      let subjects = Self::extract_subjects( cmd, &raw_command, &used_properties_with_their_aliases )?;

      Ok( VerifiedCommand
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
  exposed use Verifier;
}
