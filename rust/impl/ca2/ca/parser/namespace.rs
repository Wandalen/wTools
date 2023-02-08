pub( crate ) mod private
{
  use crate::
  {
    Parser,
    Namespace,
    parser::parser::any_word,
  };
  use wtools::{ Result, err };
  use nom::
  {
    branch::alt,
    character::complete::{ anychar, multispace0 },
    combinator::{ map, verify, not },
    multi::many_till,
    sequence::tuple,
    IResult,
  };

  impl Parser
  {
    pub( crate ) fn namespace_fn( &self ) -> impl Fn( &str ) -> IResult< &str, Namespace > + '_
    {
      let delimeter = self.namespace_delimeter.clone();
      move | input : &str |
      map( many_till
      (
        self.command_fn(),
        alt((
          map
          (
            tuple(( multispace0, verify( any_word, | word : &str | word == delimeter ) )),
            | _ | ()
          ),
          not( anychar )
        ))
      ), | x | Namespace { commands : x.0 }
      )( input )
    }

    /// Parses namespace from string
    pub fn namespace< 'a >( &'a self, input : &'a str ) -> Result< Namespace >
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

}
