pub( crate ) mod private
{
  use crate::
  {
    Parser,
    Command,
    parser::parser::any_word,
  };
  use wtools::{ Result, err };
  use nom::
  {
    branch::alt,
    bytes::complete::{ tag, take_while },
    character::complete::{ alpha1, multispace0, multispace1 },
    combinator::{ map, recognize },
    multi::many0,
    sequence::{ tuple, pair },
    IResult,
  };

  impl Parser
  {
    fn command_name() -> impl Fn( &str ) -> IResult< &str, &str >
    {
      move | input : &str |
      recognize( pair
      (
        alt(( alpha1, tag( "_" ) )),
        any_word
      ))( input )
    }
  
    fn subject( &self ) -> impl Fn( &str ) -> IResult< &str, String > + '_
    {
      // ? looks not good
      // reason - all words can be `subject`
      let prop_delimeter = self.prop_delimeter;
      let namespace_delimeter = self.namespace_delimeter.clone();

      move | input : &str |
      any_word( input )
      .map( |( tail, word )|
      {
        // if next word - is a command => it isn't subject
        // if you want pass command as subject - take it in quotes
        let not_a_command = self.command_fn()( word ).is_err();
        if not_a_command && !word.contains( prop_delimeter ) && !word.contains( &*namespace_delimeter )
        {
          ( tail, word.to_owned() )
        }
        else
        {
          ( input, "".to_owned() )
        }
      })
    }

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

    fn property( &self ) -> impl Fn( &str ) -> IResult< &str, ( String, String ) > + '_
    {
      let property_delimeter = self.prop_delimeter;
      move | input : &str |
      map
      (
        tuple(( self.propery_name(), tag( &*format!( "{property_delimeter}" ) ), any_word )),
        |( name, _, value ) : ( &str, _, _ ) | ( name.to_owned(), value.to_owned() )
      )( input )
    }

    pub( crate ) fn command_fn( &self ) -> impl Fn( &str ) -> IResult< &str, Command > + '_
    {
      let command_delimeter = self.command_delimeter;
      move | input : &str |
      map
      (
        tuple
        ((
          multispace0,
          tag( &*format!( "{command_delimeter}" ) ),
          Self::command_name(),
          many0( tuple(( multispace1, self.subject() )) ),
          //? why multispace0
          many0( tuple(( multispace0, self.property() )) )
        )),
        |( _, _, name, subjects, props )|
        {
          Command
          {
            name : name.to_owned(),
            subjects : subjects.into_iter().filter_map( |( _, subject )| if subject.is_empty() { None } else { Some( subject ) } ).collect(),
            properties : props.into_iter().map( |( _, prop )| prop ).collect()
          }
        }
      )( input )
    }
  
    /// Parses first command from string
    /// 
    /// Command name must starts with letter or underscore
    pub fn command< 'a >( &'a self, input : &'a str ) -> Result< Command >
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

}
