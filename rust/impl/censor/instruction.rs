// #![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

// pub use werror::*;
pub use wtools::error::*;
use std::collections::HashMap;

#[ derive( Debug, PartialEq ) ]
pub struct Instruction
{
  pub err : Option< Error >,
  pub command_name : Box< str >,
  pub subject : Vec< Box< str > >,
  pub properties_map : HashMap< Box< str >, Box< str > >,
}

impl Instruction
{
  fn new() -> Self
  {
    Self
    {
      err : None,
      command_name : Default::default(),
      subject : Default::default(),
      properties_map : Default::default(),

    }
  }
}

//

pub trait InstructionParseParamsAdapter
{

  //

  fn about_command_format( &self ) -> &'static str
  {
r#"Command should start from a dot `.`.
Command can have a subject and properties.
Property is pair delimited by colon `:`.
For example: `.command1 subject key1:val key2:val2`."#
  }

  //

  fn instruction_split_is_command< Src : AsRef< str > >( &self, src : Src ) -> bool
  {
    src.as_ref().starts_with( "." )
  }

  //

  fn command_name_normalize< Src : AsRef< str > >( &self, src : Src ) -> Box< str >
  {
    let splits : Vec< &str > = src.as_ref()
    .split_whitespace()
    .flat_map( | e | e.split( "." ) )
    .filter( | e | e != &"" )
    .collect();
    ( ".".to_string() + &splits.join( "." ) ).to_string().into_boxed_str()
  }

  //

  fn split_belong_to_properties< Src : AsRef< str > >( &self, src : Src ) -> i32
  {
    let src = src.as_ref();
    if !src.contains( ':' )
    {
      return 0;
    }
    let splits : Vec< &str > = src
    .split_ascii_whitespace()
    .flat_map( | e | e.split( ":" ) )
    .filter( | e | e != &"" )
    .collect();
    let index = splits.iter().position( | e | *e == ":" ).unwrap();
    if index == 0
    {
      return 2;
    }
    return 1;
  }

  //

  /* xxx : make it accept also vector */
  fn parse_from_splits< I >( &self, mut splits : I ) -> Instruction
  where
    < I as Iterator >::Item : std::fmt::Display,
    < I as Iterator >::Item : AsRef< str >,
    I : core::iter::Iterator,
  {
    let mut result = Instruction::new();

    // splits.for_each( | arg | println!( "{}", arg ) );

    let command_name = splits.next();

    if command_name.is_none()
    {
      result.err = Some( err!( "Lack of arguments" ) );
      return result;
    }

    let command_name = command_name.unwrap();

    if !self.instruction_split_is_command( &command_name )
    {
      result.err = Some( err!( "{}\nDoes not start as command\n{}", command_name, self.about_command_format() ) );
      return result;
    }

    result.command_name = self.command_name_normalize( command_name );

    // let params_splits;

    while let Some( split ) = splits.next()
    {
      let split_unwrap = split.as_ref();
      let belong = self.split_belong_to_properties( split_unwrap );
      if belong > 0
      {
        // if belong == 1
        {
          let props_splits = std::iter::once( split ).chain( splits );
          result.properties_map = super::props::parse_from_splits( props_splits );
        }
        break;
      }
      result.subject.push( split_unwrap.to_string().into_boxed_str() );
      // params_splits.chain();
    }

    // dbg!(  );

    // super::params::parse_from_splits(  );

    result
  }

//   //
//
//   fn str_structure_parse()
//   {
//
//   }

}

#[ derive( Debug, PartialEq ) ]
pub struct InstructionParseParams
{
}

impl InstructionParseParams
{
  pub fn new() -> Self
  {
    Self
    {
    }
  }
}

impl InstructionParseParamsAdapter for InstructionParseParams
{
}

//

pub fn parse_from_splits< I >( splits : I ) -> Instruction
where
  < I as Iterator >::Item : std::fmt::Display,
  < I as Iterator >::Item : AsRef< str >,
  I : core::iter::Iterator,
{
  let params = InstructionParseParams::new();
  let instruction = params.parse_from_splits( splits );
  instruction
}

//

// var command = commandIdentitySet.command = Object.create( null );
// command.subjectHint = 'A name of identity.';
// command.hint = 'Modify an existed identity.';
// command.longHint = 'Much longer description.';
// command.properties =
// {
//   'login' : 'An identity login ( user name ) that is used for all identity scripts if no specifique login defined.',
//   'email' : 'An email that is used for all identity scripts if no specifique email defined.',
// };
