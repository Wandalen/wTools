pub use werror::*;
use std::collections::HashMap;

// static EmptyString : Box< str > = "".to_string().into_boxed_str();

#[ derive( Debug, PartialEq ) ]
pub struct Instruction // < S >
// where
//   S : AsRef< str >,
{
  pub err : Option< Error >,
  pub command_name : Box< str >,
  pub subject : Box< str >,
  pub properties_map : HashMap< Box< str >, Box< str > >,
}

impl // < S >
Instruction // < S >
// where
//   S : AsRef< str >,
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
  fn about_command_format( &self ) -> &'static str
  {
r#"Command should start from a dot `.`.
Command can have a subject and properties.
Property is pair delimited by colon `:`.
For example: `.command1 subject key1:val key2:val2`."#
  }
  fn instruction_split_is_command< Src : AsRef< str > >( &self, src : Src ) -> bool
  {
    src.as_ref().starts_with( "." )
  }
}

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

pub fn instruction_parse_from_splits< Params, I >( params : &Params, mut splits : I ) -> Instruction
where
  // < I as Iterator >::Item: std::fmt::Debug,
  < I as Iterator >::Item : std::fmt::Display,
  < I as Iterator >::Item : AsRef< str >,
  Params : InstructionParseParamsAdapter,
  I : core::iter::Iterator,
{
  let mut result = Instruction::new();

  // splits.for_each( | arg | println!( "{}", arg ) );
  // splits.for_each( | arg | println!( "{}", arg ) );

  let command_name = splits.next();

  if command_name.is_none()
  {
    result.err = Some( err!( "Lack of arguments" ) );
    return result;
  }

  let command_name = command_name.unwrap();

  // println!( "command_name : {}", command_name );

  if !params.instruction_split_is_command( &command_name )
  {
    result.err = Some( err!( "{}\nDoes not start as command\n{}", command_name, params.about_command_format() ) );
    return result;
  }

  result.command_name = command_name.as_ref().to_string().into_boxed_str();

  result
}

// var command = commandIdentitySet.command = Object.create( null );
// command.subjectHint = 'A name of identity.';
// command.hint = 'Modify an existed identity.';
// command.longHint = 'Much longer description.';
// command.properties =
// {
//   'login' : 'An identity login ( user name ) that is used for all identity scripts if no specifique login defined.',
//   'email' : 'An email that is used for all identity scripts if no specifique email defined.',
// };
