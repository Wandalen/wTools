mod private
{
  use crate::*;

  use help::{ HelpGeneratorOptions, LevelOfDetail, generate_help_content };
  use grammar::{ Dictionary, Command, command::ValueDescription };
  use executor::{ Args, Props };
  use std::collections::HashMap;
  use indexmap::IndexMap;
  use verifier::VerifiedCommand;
  use parser::{ Program, ParsedCommand };

  #[ allow( missing_docs ) ]
  #[ derive( Debug, error::typed::Error ) ]
  pub enum VerificationError
  {
    #[ error
    (
      "Command not found. {} {}",
      if let Some( phrase ) = name_suggestion { format!( "Maybe you mean `.{phrase}`?" ) } else { "Please use `.` command to see the list of available commands.".into() },
      if let Some( info ) = command_info { format!( "Command info: `{info}`" ) } else { "".into() }
    )]
    CommandNotFound { name_suggestion: Option< String >, command_info: Option< String > },
    #[ error( "Fail in command `.{command_name}` while processing subjects. {error}" ) ]
    Subject { command_name: String, error: SubjectError },
    #[ error( "Fail in command `.{command_name}` while processing properties. {error}" ) ]
    Property { command_name: String, error: PropertyError },
  }

  #[ allow( missing_docs ) ]
  #[ derive( Debug, error::typed::Error ) ]
  pub enum SubjectError
  {
    #[ error( "Missing not optional subject" ) ]
    MissingNotOptional,
    #[ error( "Can not identify a subject: `{value}`" ) ]
    CanNotIdentify { value: String },
  }

  #[ allow( missing_docs ) ]
  #[ derive( Debug, error::typed::Error ) ]
  pub enum PropertyError
  {
    #[ error( "Expected: {description:?}. Found: {input}" ) ]
    Cast { description: ValueDescription, input: String },
  }

  // xxx

  /// Converts a `ParsedCommand` to a `VerifiedCommand` by performing validation and type casting on values.
  ///
  /// ```
  /// # use wca::{ Type, verifier::Verifier, grammar::{ Dictionary, Command }, parser::ParsedCommand };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > >
  /// # {
  /// # let verifier = Verifier;
  /// let dictionary = Dictionary::former()
  /// .command( Command::former().phrase( "command" ).form() )
  /// .form();
  ///
  /// let raw_command = ParsedCommand
  /// {
  ///   name: "command".to_string(),
  ///   subjects: vec![],
  ///   properties: HashMap::new(),
  /// };
  ///
  /// let grammar_command = verifier.to_command( &dictionary, raw_command )?;
  /// # Ok( () )
  /// # }
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct Verifier;

  impl Verifier
  {
    /// Converts raw program to grammatically correct
    ///
    /// Converts all namespaces into it with `to_namespace` method.
    pub fn to_program
    (
      &self,
      dictionary : &Dictionary,
      raw_program : Program< ParsedCommand >
    )
    -> Result< Program< VerifiedCommand >, VerificationError >
    // aaa : use typed error
    // aaa : done
    {
      let commands: Result< Vec< VerifiedCommand >, VerificationError > = raw_program.commands
      .into_iter()
      .map( | n | self.to_command( dictionary, n ) )
      .collect();
      let commands = commands?;

      Ok( Program { commands } )
    }

    #[ cfg( feature = "on_unknown_suggest" ) ]
    fn suggest_command< 'a >( dictionary : &'a Dictionary, user_input: &str ) -> Option< &'a str >
    {
      use textdistance::{ Algorithm, JaroWinkler };
      let jaro = JaroWinkler::default();
      let sim = dictionary
      .commands
      .iter()
      .map( |( name, c )| ( jaro.for_str( name.as_str(), user_input ).nsim(), c ) )
      .max_by( |( s1, _ ), ( s2, _ )| s1.total_cmp( s2 ) );
      if let Some(( sim, variant )) = sim
      {
        if sim > 0.0
        {
          let phrase = &variant.phrase;
          return Some( phrase );
        }
      }

      None
    }

    fn get_count_from_properties
    (
      properties : &IndexMap< String, ValueDescription >,
      properties_aliases : &HashMap< String, String >,
      raw_properties : &HashMap< String, String >
    ) -> usize
    {
      raw_properties.iter()
        .filter( |( k, _ )| !( properties.contains_key( *k ) || properties_aliases.get( *k ).map_or( false, | key | properties.contains_key( key ) ) ) )
        .count()
    }

    fn is_valid_command_variant( subjects_count : usize, raw_count : usize, possible_count : usize ) -> bool
    {
      raw_count + possible_count <= subjects_count
    }

    fn check_command< 'a >( variant : &'a Command, raw_command : &ParsedCommand ) -> Option< &'a Command >
    {
      let Command { subjects, properties, properties_aliases, .. } = variant;
      let raw_subjects_count = raw_command.subjects.len();
      let expected_subjects_count = subjects.len();
      if raw_subjects_count > expected_subjects_count { return None; }

      let possible_subjects_count = Self::get_count_from_properties( properties, properties_aliases, &raw_command.properties );
      if Self::is_valid_command_variant( expected_subjects_count, raw_subjects_count, possible_subjects_count ) { Some( variant ) } else { None }
    }

    // aaa : use typed error
    // aaa : done.
    fn extract_subjects( command : &Command, raw_command : &ParsedCommand, used_properties : &[ &String ] )
    ->
    Result< Vec< Value >, SubjectError >
    {
      let mut subjects = vec![];

      let all_subjects: Vec< _ > = raw_command
      .subjects.clone().into_iter()
      .chain
      (
        raw_command.properties.iter()
        .filter( |( key, _ )| !used_properties.contains( key ) )
        .map( |( key, value )| format!( "{key}:{value}" ) )
      )
      .collect();
      let mut rc_subjects_iter = all_subjects.iter();
      let mut current = rc_subjects_iter.next();

      for ValueDescription { kind, optional, .. } in &command.subjects
      {
        let value = match current.and_then( | v | kind.try_cast( v.clone() ).ok() )
        {
          Some( v ) => v,
          None if *optional => continue,
          _ => return Err( SubjectError::MissingNotOptional ),
        };
        subjects.push( value );
        current = rc_subjects_iter.next();
      }
      if let Some( value ) = current { return Err( SubjectError::CanNotIdentify { value: value.clone() } ) }

      Ok( subjects )
    }

    // aaa : use typed error
    // aaa : done.
    fn extract_properties( command: &Command, raw_command : HashMap< String, String > )
    ->
    Result< HashMap< String, Value >, PropertyError >
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
        value_description.kind.try_cast( value.clone() ).map( | v | ( key.clone(), v ) ).map_err( | _ | PropertyError::Cast { description: value_description.clone(), input: format!( "{key}: {value}" ) } )
      )
      .collect()
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
      .collect()
    }

    /// Converts raw command to grammatically correct
    ///
    /// Make sure that this command is described in the grammar and matches it(command itself and all it options too).
    // aaa : use typed error
    // aaa : done.
    pub fn to_command( &self, dictionary : &Dictionary, raw_command : ParsedCommand )
    ->
    Result< VerifiedCommand, VerificationError >
    {
      if raw_command.name.ends_with( '.' ) | raw_command.name.ends_with( ".?" )
      {
        return Ok( VerifiedCommand
        {
          phrase : raw_command.name,
          internal_command : true,
          args : Args( vec![] ),
          props : Props( HashMap::new() ),
        });
      }
      let command = dictionary.command( &raw_command.name )
      .ok_or_else::< VerificationError, _ >
      (
        ||
        {
          #[ cfg( feature = "on_unknown_suggest" ) ]
          if let Some( phrase ) = Self::suggest_command( dictionary, &raw_command.name )
          {
            return VerificationError::CommandNotFound { name_suggestion: Some( phrase ), command_info: None };
          }
          VerificationError::CommandNotFound { name_suggestion: None, command_info: None }
        }
      )?;

      let Some( cmd ) = Self::check_command( command, &raw_command ) else
      {
        return Err( VerificationError::CommandNotFound
        {
          name_suggestion: Some( command.phrase.clone() ),
          command_info: Some( generate_help_content( dictionary, HelpGeneratorOptions::former().for_commands([ dictionary.command( &raw_command.name ).unwrap() ]).command_prefix( "." ).subject_detailing( LevelOfDetail::Detailed ).form() ).strip_suffix( "  " ).unwrap().into() ),
        } );
      };

      let properties = Self::extract_properties( cmd, raw_command.properties.clone() ).map_err( | e | VerificationError::Property { command_name: cmd.phrase.clone(), error: e } )?;
      let used_properties_with_their_aliases = Self::group_properties_and_their_aliases( &cmd.properties_aliases, properties.keys() );
      let subjects = Self::extract_subjects( cmd, &raw_command, &used_properties_with_their_aliases ).map_err( | e | VerificationError::Subject { command_name: cmd.phrase.clone(), error: e } )?;

      Ok( VerifiedCommand
      {
        phrase : cmd.phrase.to_owned(),
        internal_command : false,
        args : Args( subjects ),
        props : Props( properties ),
      })
    }
  }
}

//

crate::mod_interface!
{
  orphan use Verifier;
  orphan use VerificationError;

  // own use LevelOfDetail;
  // own use generate_help_content;

}
