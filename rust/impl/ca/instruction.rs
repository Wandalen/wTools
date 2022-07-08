
pub( crate ) mod private
{
  pub use wtools::error::BasicError;
  pub use wtools::string::parse_request;
  pub use wtools::string::parse_request::OpType;
  pub use wtools::former::Former;
  use std::collections::HashMap;

  ///
  /// Instruction handle.
  ///

  #[ derive( Debug, Default, PartialEq ) ]
  pub struct Instruction
  {
    /// Error of Instruction forming.
    pub err : Option< BasicError >,
    /// Name of command
    pub command_name : String,
    /// Subject.
    pub subject : String,
    /// Properties map.
    pub properties_map : HashMap< String, OpType< String > >,
  }

  ///
  /// Parameters for parsing instructions.
  ///

  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  #[ perform( fn parse( &self ) -> Instruction ) ]
  pub struct InstructionParseParams< 'a >
  {
    #[ default( "" ) ]
    instruction : &'a str,
    properties_map : Option< HashMap< String, OpType< String > > >,
    #[ default( true ) ]
    properties_map_parsing : bool,
    #[ default( true ) ]
    several_values : bool,
    #[ default( true ) ]
    quoting : bool,
    #[ default( false ) ]
    unquoting : bool,
    #[ default( false ) ]
    subject_win_paths_maybe : bool,
  }

  impl< 'a > InstructionParseParams< 'a >
  {
    /// Set not default builder field `properties_map`.
    pub fn properties_map( mut self, properties_map : HashMap< String, OpType< String > > ) -> InstructionParseParams< 'a >
    {
      self.properties_map = Some( properties_map );
      self
    }
  }

  ///
  /// Instruction behaviour.
  ///

  pub trait InstructionParseParamsAdapter
  {
    /// Print info about valid command format.
    fn about_command_format( &self ) -> &'static str;
    /// Check that command name is valid.
    fn command_name_is_valid( &self, command_name : &str ) -> bool;
    /// Parse instruction from string slice.
    fn parse_str( &self ) -> Instruction;
    /// Main parser.
    fn parse( &self ) -> Instruction;
  }

  //

  impl < 'a >InstructionParseParamsAdapter for InstructionParseParams< 'a >
  {
    fn about_command_format( &self ) -> &'static str
    {
  r#"Command should start from a dot `.`.
  Command can have a subject and properties.
  Property is pair delimited by colon `:`.
  For example: `.struct1 subject key1:val key2:val2`."#
    }
    fn command_name_is_valid( &self, command_name : &str ) -> bool
    {
      command_name.trim().starts_with( "." )
    }
    fn parse_str( &self ) -> Instruction
    {
      let mut result = Instruction::default();

      let ( command_name, request ) = match self.instruction.split_once( " " )
      {
        Some( entries ) => entries,
        None => ( self.instruction, "" ),
      };

      result.command_name = command_name.to_string();

      if !self.command_name_is_valid( &result.command_name[ .. ] )
      {
        self.about_command_format();
        result.err = Some( BasicError::new( "Invalid command" ) );
        return result;
      }

      let request = parse_request::request_parse()
      .src( request )
      .several_values( self.several_values )
      .quoting( self.quoting )
      .unquoting( self.unquoting )
      .subject_win_paths_maybe( self.subject_win_paths_maybe )
      .perform();

      if request.subjects.len() > 1
      {
        result.err = Some( BasicError::new( "Too many instructions" ) );
        return result;
      }

      if self.properties_map_parsing /* dead code to delete */
      {
      }

      result.subject = request.subject.clone();
      result.properties_map = request.map.clone();

      if self.properties_map.is_some()
      {
        for ( key, value ) in self.properties_map.as_ref().unwrap().iter()
        {
          result.properties_map.insert( key.clone(), value.clone() );
        }
      }

      result
    }
    fn parse( &self ) -> Instruction
    {
      self.parse_str()
    }
  }

  //

  ///
  /// Get instruction parser builder.
  ///

  pub fn instruction_parse< 'a >() -> InstructionParseParamsFormer< 'a >
  {
    InstructionParseParams::former()
  }
}

/// Protected namespace of the module.
pub mod protected
{
  // use super::private as i;

  pub use super::private::Instruction;
  pub use super::private::InstructionParseParams;
  pub use super::private::InstructionParseParamsAdapter;
  pub use super::private::instruction_parse;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::private as i;

  pub use super::private::InstructionParseParamsAdapter;
  pub use super::private::instruction_parse;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  // use super::private as i;

  pub use super::private::InstructionParseParamsAdapter;
}

