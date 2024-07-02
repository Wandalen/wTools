mod private
{
  use crate::*;

  use std::collections::HashMap;

  use error::{ return_err };

  /// `Parser` is a struct used for parsing data.
  #[ derive( Debug ) ]
  pub struct Parser;

  impl Parser
  {
    /// Parses a vector of command line arguments and returns a `Program` containing the parsed commands.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of strings representing the command line arguments.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a `Program` containing the parsed commands if successful, or an error if parsing fails.
    // qqq : use typed error
    pub fn parse< As, A >( &self, args : As ) -> error::untyped::Result< Program< ParsedCommand > >
    where
      As : IntoIterator< Item = A >,
      A : Into< String >,
    {
      let args = args.into_iter().map( Into::into ).collect::< Vec< _ > >();
      let mut commands = vec![];
      let mut i = 0;
      while i < args.len()
      {
        let ( command, relative_pos ) = Self::parse_command( &args[ i.. ] )?;
        i += relative_pos;
        commands.push( command );
      }

      Ok( Program { commands } )
    }

    // with dot at the beginning
    fn valid_command_name( input : &str ) -> bool
    {
      if let Some( name ) = input.strip_prefix( '.' )
      {
        name.is_empty() || name.starts_with( '?' ) || name.chars().next().is_some_and( | c | c.is_alphanumeric() )
      }
      else
      {
        false
      }
    }

    // returns ParsedCommand and relative position of the last parsed item
    // qqq : use typed error
    fn parse_command( args : &[ String ] ) -> error::untyped::Result< ( ParsedCommand, usize ) >
    {
      if args.is_empty() {
        return_err!( "Unexpected behaviour: Try to parse command without input" );
      }

      let mut i = 0;

      if !Self::valid_command_name( &args[ i ] )
      {
        return_err!( "Unexpected input: Expected a command, found: `{}`", args[ i ] );
      }
      let name = match args[ i ].strip_prefix( '.' ).unwrap()
      {
        "" => ".",
        "?" => ".?",
        other => other,
      };
      i += 1;
      let ( subjects, properties, relative_pos ) = Self::parse_command_args( &args[ i .. ] )?;

      i += relative_pos;

      return Ok(
      (
        ParsedCommand
        {
          name : name.to_string(),
          subjects,
          properties,
        },
        i,
      ))
    }

    // returns ( subjects, properties, relative_end_pos )
    // qqq : use typed error
    fn parse_command_args( args : &[ String ] ) -> error::untyped::Result< ( Vec< String >, HashMap< String, String >, usize ) >
    {
      let mut i = 0;

      let mut subjects = vec![];
      let mut properties = HashMap::new();

      let mut properties_turn = false;
      while i < args.len()
      {
        let item = &args[ i ];

        if Self::valid_command_name( item ) { break; }

        if item.contains( ':' )
        {
          properties_turn = true;
          let ( name, value ) = item.split_once( ':' ).unwrap();
          // prop:value
          if !value.is_empty()
          {
            properties.insert( name.to_string(), value.to_string() );
          }
          // prop: value
          else if args.len() > i + 1
          {
            properties.insert( name.to_string(), args[ i + 1 ].to_string() );
            i += 1;
          }
          // we can identify this as a subject, can't we?
          // prop:
          else
          {
            return_err!( "Unexpected input '{}': Detected a possible property key preceding the ':' character. However, no corresponding value was found.", item );
          }
        }
        // prop : value | prop :value
        else if args.len() > i + 1 && args[ i + 1 ].starts_with( ':' )
        {
          // :value
          if args[ i + 1 ].len() > 1
          {
            properties.insert( args[ i ].clone(), args[ i + 1 ].strip_prefix( ':' ).unwrap().to_string() );
            i += 1;
          }
          // : value
          else if args.len() > i + 2
          {
            properties.insert( args[ i ].clone(), args[ i + 2 ].clone() );
            i += 2;
          }
          // :
          else
          {
            return_err!( "Unexpected input '{} :': Detected a possible property key preceding the ':' character. However, no corresponding value was found.", item );
          }
        }

        else if !properties_turn { subjects.push( item.to_string() ); }

        else { return_err!( "Unexpected input: Expected `command` or `property`, found: `{}`", item ); }
        i += 1;
      }

      Ok(( subjects, properties, i ))
    }
  }
}

//

crate::mod_interface!
{
  exposed use Parser;
}
