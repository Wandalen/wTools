pub( crate ) mod private
{
  use crate::
  {
    Parser,
    Namespace, Program,
  };
  use wtools::{ Result, err };
  use nom::
  {
    character::complete::anychar,
    combinator::{ map, not },
    multi::many_till,
    IResult,
  };

  impl Parser
  {
    fn namespaces_fn( &self ) -> impl Fn( &str ) -> IResult< &str, Vec< Namespace > > + '_
    {
      move | input : &str |
      map( many_till
      (
        self.namespace_fn(),
        not( anychar )
      ), | x | x.0
      )( input )
    }

    pub( crate ) fn program_fn( &self ) -> impl Fn( &str ) -> IResult< &str, Program > + '_
    {
      move | input : &str |
      map
      (
        self.namespaces_fn(),
        | namespaces | Program { namespaces }
      )( dbg!( input ) )
    }

    /// Parses program from string
    pub fn program< 'a >( &'a self, input : &'a str ) -> Result< Program >
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

}
