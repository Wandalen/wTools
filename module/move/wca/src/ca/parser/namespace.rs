pub( crate ) mod private
{
  use crate::*;
  use ca::
  {
    Namespace, ParsedCommand,
    Parser,
    parser::
    {
      parser::any_word,
      command::CommandParserFn,
    }
  };
  use wtools::{ error::Result, err };
  use nom::
  {
    branch::alt,
    character::complete::{ anychar, multispace0 },
    combinator::{ map, verify, not },
    multi::many_till,
    sequence::tuple,
    IResult,
  };

  // qqq : for Bohdan : bad documentation. what is it for? example of input and output?
  /// Can parse Namespaces
  pub trait NamespaceParser
  {
    /// Parses first namespace from string
    fn namespace( &self, input : &str ) -> Result< Namespace< ParsedCommand > >;
  }

  pub( crate ) trait GetNamespaceDelimeter
  {
    fn get_namespace_delimeter( &self ) -> &str;
  }

  impl GetNamespaceDelimeter for Parser
  {
    fn get_namespace_delimeter( &self ) -> &str { &self.namespace_delimeter }
  }

  type NamespaceParserFunction< 'a > = Box< dyn Fn( &str ) -> IResult< &str, Namespace< ParsedCommand > > + 'a >;

  /// Can be used as function to parse a Namespace
  pub( crate ) trait NamespaceParserFn : CommandParserFn + GetNamespaceDelimeter
  {
    /// Returns function that can parse a Namespace
    fn namespace_fn( &self ) -> NamespaceParserFunction< '_ >
    {
      let delimeter = self.get_namespace_delimeter();
      Box::new
      (
        move | input : &str |
        map( many_till
        (
          self.command_fn(),
          alt
          ((
            map( tuple(( multispace0, verify( any_word, | word : &str | word == delimeter ) )), | _ | () ),
            not( anychar )
          ))
        ), | x | Namespace { commands : x.0 }
        )( input )
      )
    }
  }

  impl NamespaceParserFn for Parser {}

  impl NamespaceParser for Parser
  {
    fn namespace< 'a >( &'a self, input : &'a str ) -> Result< Namespace< ParsedCommand > >
    {
      self.namespace_fn()( input.trim() )
      .map( |( _, namespace )| namespace )
      .map_err( | _ | err!( "Fail to parse `Namespace`" ) )
    }
  }
}

//

crate::mod_interface!
{
  exposed use NamespaceParser;
}
