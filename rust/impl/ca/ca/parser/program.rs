pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace, RawCommand,
    Parser,
    ca::parser::namespace::private::NamespaceParserFn,
  };
  use error_tools::{ Result, err };
  use nom::
  {
    character::complete::anychar,
    combinator::{ map, not },
    multi::many_till,
    IResult,
  };

  /// Can parser Programs
  pub trait ProgramParser
  {
    /// Parses program from string
    fn program( &self, input : &str ) -> Result< Program< Namespace< RawCommand > > >;
  }

  type ProgramParserFunction< 'a > = Box< dyn Fn( &str ) -> IResult< &str, Program< Namespace< RawCommand > > > + 'a >;

  /// Can be used as function to parse a Namespace
  pub( crate ) trait ProgramParserFn : NamespaceParserFn
  {
    /// Returns function that can parse a Namespace
    fn program_fn( &self ) -> ProgramParserFunction
    {
      Box::new
      (
        move | input : &str |
        map( many_till
        (
          self.namespace_fn(),
          not( anychar )
        ), |( namespaces, _ )| Program { namespaces }
        )( input )
      )
    }
  }

  impl ProgramParserFn for Parser {}

  impl ProgramParser for Parser
  {
    fn program< 'a >( &'a self, input : &'a str ) -> Result< Program< Namespace< RawCommand > > >
    {
      self.program_fn()( input.trim() )
      .map( |( _, program )| program )
      .map_err( | e | { dbg!( e ); err!( "Fail to parse `Program`" ) } )
    }
  }
}

//

crate::mod_interface!
{
  prelude use ProgramParser;
}
