pub( crate ) mod private
{
  use crate::
  {
    Parser,
    RawCommand as Command,
    parser::parser::any_word,
  };
  use wtools::{ Result, err };
  use nom::
  {
    branch::alt,
    bytes::complete::{ tag, take_while, escaped },
    character::complete::{ alpha1, multispace0, multispace1, none_of, one_of },
    combinator::{ map, map_opt, recognize },
    multi::many0,
    sequence::{ tuple, pair, delimited },
    IResult,
  };

  /// Can parse Commands
  pub trait CommandParser
  {
    /// Parses first command from string
    ///
    /// Command name must starts with letter or underscore
    fn command( &self, input : &str ) -> Result< Command >;
  }

  type CommandParserFunction< 'a > = Box< dyn Fn( &str ) -> IResult< &str, Command > + 'a >;

  /// Can be used as function to parse a Command
  pub trait CommandParserFn : GetCommandPrefix + CommandNameParserFn + CommandSubjectParserFn + CommandPropertyParserFn
  {
    /// Returns function that can parse a Command
    fn command_fn( &self ) -> CommandParserFunction
    {
      let command_prefix = self.get_command_prefix();
      Box::new( move | input : &str |
      map
      (
        tuple
        ((
          multispace0,
          tag( &*format!( "{command_prefix}" ) ),
          Self::command_name_fn(),
          many0( tuple(( multispace1, self.command_subject_fn() )) ),
          //? why multispace0
          many0( tuple(( multispace0, self.command_property_fn() )) )
        )),
        |( _, _, name, subjects, props )|
        {
          Command
          {
            name : name.to_string(),
            subjects : subjects.into_iter().filter_map( |( _, subject )| if subject.is_empty() { None } else { Some( subject ) } ).collect(),
            properties : props.into_iter().map( |( _, prop )| prop ).collect()
          }
        }
      )( input ) )
    }
  }

  pub trait GetCommandPrefix
  {
    fn get_command_prefix( &self ) -> char;
  }

  impl GetCommandPrefix for Parser
  {
    fn get_command_prefix( &self ) -> char { self.command_prefix }
  }

  type CommandNameParserFunction = Box< dyn Fn( &str ) -> IResult< &str, &str > >;

  /// Can be used as function to parse a Command name
  pub trait CommandNameParserFn
  {
    /// Returns function that can parse a Command name
    fn command_name_fn() -> CommandNameParserFunction;
  }

  type CommandSubjectParserFunction< 'a > = Box< dyn Fn( &str ) -> IResult< &str, String > + 'a >;

  /// Can be used as function to parse a Command subject
  pub trait CommandSubjectParserFn
  {
    /// Returns function that can parse a Command subject
    fn command_subject_fn( &self ) -> CommandSubjectParserFunction;
  }

  type CommandPropertyParserFunction< 'a > = Box< dyn Fn( &str ) -> IResult< &str, ( String, String ) > + 'a >;

  /// Can be used as function to parse a Command property
  pub trait CommandPropertyParserFn
  {
    /// Returns function that can parse a Command property
    fn command_property_fn( &self ) -> CommandPropertyParserFunction;
  }

  impl CommandNameParserFn for Parser
  {
    fn command_name_fn() -> CommandNameParserFunction
    {
      Box::new
      (
        move | input : &str |
        recognize( pair
        (
          alt(( alpha1, tag( "_" ) )),
          any_word
        ))( input )
      )
    }
  }

  impl CommandSubjectParserFn for Parser
  {
    fn command_subject_fn( &self ) -> CommandSubjectParserFunction
    {
      // ? looks not good
      // reason - all words can be `subject`
      let prop_delimeter = self.prop_delimeter;
      let namespace_delimeter = self.namespace_delimeter.clone();

      Box::new
      (
        move | input : &str |
        alt
        ((
          // quoted subject
          map( delimited( tag( "\"" ), escaped( none_of( "\\\"" ), '\\', one_of( "\\\"" ) ), tag( "\"" ) ), | s : &str | s.replace( "\\\"", "\"" ).replace( "\\\\", "\\" ) ),
          map( delimited( tag( "'" ), escaped( none_of( "\\'" ), '\\', one_of( "\\'" ) ), tag( "'" ) ), | s : &str | s.replace( "\\\'", "'" ).replace( "\\\\", "\\" ) ),
          // single word subject
          map_opt
          (
            any_word,
            | word |
            {
              // if next word - is a command => it isn't subject
              // if you want pass command as subject - take it in quotes
              let not_a_command = self.command_fn()( word ).is_err();
              if not_a_command && !word.contains( prop_delimeter ) && !word.contains( &*namespace_delimeter )
              {
                Some( word.to_owned() )
              }
              else
              {
                None
              }
            }
          )
        ))( input )
      )
    }
  }

  impl Parser
  {
    fn propery_name( &self ) -> impl Fn( &str ) -> IResult< &str, &str >
    {
      let property_delimeter = self.prop_delimeter;
      move | input : &str |
      recognize( pair
      (
        alt(( alpha1, tag( "_" ) )),
        take_while( | c : char | !c.is_whitespace() && c != property_delimeter )
      ))( input )
    }
  }

  impl CommandPropertyParserFn for Parser
  {
    fn command_property_fn( &self ) -> CommandPropertyParserFunction
    {
      let property_delimeter = self.prop_delimeter;
      Box::new
      (
        move | input : &str |
        map
        (
          tuple
          ((
            self.propery_name(),
            tag( &*format!( "{property_delimeter}" ) ),
            alt
            ((
              // quoted value
              map( delimited( tag( "\"" ), escaped( none_of( "\\\"" ), '\\', one_of( "\\\"" ) ), tag( "\"" ) ), | s : &str | s.replace( "\\\"", "\"" ).replace( "\\\\", "\\" ) ),
              map( delimited( tag( "'" ), escaped( none_of( "\\'" ), '\\', one_of( "\\'" ) ), tag( "'" ) ), | s : &str | s.replace( "\\\'", "'" ).replace( "\\\\", "\\" ) ),
              // single word
              map( any_word, | s : &str | s.to_owned() ),
            ))
          )),
          |( name, _, value ) : ( &str, _, _ ) | ( name.to_owned(), value )
        )( input )
      )
    }
  }

  impl CommandParserFn for Parser {}

  impl CommandParser for Parser
  {
    fn command( &self, input : &str ) -> Result< Command >
    {
      self.command_fn()( input )
      .map( |( _, command )| command )
      .map_err( | _ | err!( "Fail to parse `Command`" ) )
    }
  }
}

//

crate::mod_interface!
{
  prelude use CommandParser;
  protected use CommandParserFn;
}
