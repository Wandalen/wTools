/// Private namespace of the module.
mod private
{
  use std::collections::HashMap;

  // use wtools::error::{ BasicError, err };
  use error_tools::error::{ BasicError, err };
  // use error_tools::BasicError;
  // use error_tools::err;

  ///
  /// Instruction.
  ///

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct Instruction
  {
    /// Error of parsing an instruction.
    pub err : Option< BasicError >,
    /// Command name.
    pub command_name : Box< str >,
    /// Subject of command.
    pub subject : Vec< Box< str > >,
    /// Properties of command.
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

  ///
  /// Adapter for instruction.
  ///

  pub trait InstructionParseParamsAdapter
  {

    /// Print info about command format.
    fn about_command_format( &self ) -> &'static str
    {
  r#"Command should start from a dot `.`.
  Command can have a subject and properties.
  Property is pair delimited by colon `:`.
  For example: `.struct1 subject key1:val key2:val2`."#
    }

    /// Check that command begins with dot.
    fn instruction_split_is_command< Src : AsRef< str > >( &self, src : Src ) -> bool
    {
      src.as_ref().starts_with( '.' )
    }

    /// Normalize command name.
    fn command_name_normalize< Src : AsRef< str > >( &self, src : Src ) -> Box< str >
    {
      let splits : Vec< &str > = src.as_ref()
      .split_whitespace()
      .flat_map( | e | e.split( '.' ) )
      .filter( | e | e != &"" )
      .collect();
      ( ".".to_string() + &splits.join( "." ) ).into_boxed_str()
    }

    /// Make properties map.
    fn split_belong_to_properties< Src : AsRef< str > >( &self, src : Src ) -> i32
    {
      let src = src.as_ref();
      if !src.contains( ':' )
      {
        return 0;
      }
      let splits : Vec< &str > = src
      .split_ascii_whitespace()
      .flat_map( | e | e.split( ':' ) )
      .filter( | e | e != &"" )
      .collect();
      let index = splits.iter().position( | e | *e == ":" ).unwrap();
      if index == 0
      {
        return 2;
      }
      1
    }

    /// Parse instruction from splits.
    /* zzz : make it accept also vector */
    fn parse_from_splits< I >( &self, mut splits : I ) -> Instruction
    where
      < I as Iterator >::Item : core::fmt::Display,
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
            result.properties_map = crate::props::parse_from_splits( props_splits );
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

  ///
  /// Parameters of instruction.
  ///

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct InstructionParseParams
  {
  }

  impl InstructionParseParams
  {
    /// Create new instruction parameters.
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

  ///
  /// Parse input as instruction from splits.
  ///

  pub fn parse_from_splits< I >( splits : I ) -> Instruction
  where
    < I as Iterator >::Item : core::fmt::Display,
    < I as Iterator >::Item : AsRef< str >,
    I : core::iter::Iterator,
  {
    let params = InstructionParseParams::new();
    params.parse_from_splits( splits )
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
}

//

::meta_tools::mod_interface!
{
  // qqq : for Dima : bad : list all elements, don't use * for private /* aaa : Dmytro : expanded */
  prelude use Instruction;
  prelude use InstructionParseParamsAdapter;
  prelude use InstructionParseParams;
  prelude use parse_from_splits;
}
