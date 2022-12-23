
pub( crate ) mod private
{
  pub use wtools::error::
  {
    Result,
    BasicError
  };
  pub use wtools::string::parse_request;
  pub use wtools::string::parse_request::OpType;
  pub use wtools::former::Former;
  use std::collections::HashMap;

  ///
  /// Instruction parser.
  ///

  pub trait InstructionParser
  {
    /// Return help about valid command format.
    fn about_command_format( &self ) -> &'static str;
    /// Check that command name is valid.
    fn command_name_is_valid( &self, command_name : &str ) -> bool;
    /// Parse instruction from string slice.
    fn parse( &self, input : impl AsRef< str > ) -> Result< Instruction >;
  }

  ///
  /// Instruction handle.
  ///

  #[ derive( Debug, Default, PartialEq, Eq ) ]
  pub struct Instruction
  {
    /// Name of command
    pub command_name : String,
    /// Subject.
    pub subject : String,
    /// Properties map.
    pub properties_map : HashMap< String, OpType< String > >,
  }

  ///
  /// Default parser used in CommandAggregator.
  ///

  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct DefaultInstructionParser
  {
    /// Allows to inject some properties in each parser call.
    properties: Vec< ( String, OpType< String > ) >,
    #[ default( true ) ]
    several_values : bool,
    #[ default( true ) ]
    quoting : bool,
    #[ default( false ) ]
    unquoting : bool,
    #[ default( false ) ]
    subject_win_paths_maybe : bool,
  }

  impl InstructionParser for DefaultInstructionParser
  {
    fn about_command_format( &self ) -> &'static str
    {
r#"A command should start from a dot `.`.
Commands can have a subject and properties.
Property is a pair that is delimited by a colon `:`.
For example: `.struct1 subject key1:val key2:val2`."#
    }

    fn command_name_is_valid( &self, command_name : &str ) -> bool
    {
      command_name.trim().starts_with( '.' )
    }

    fn parse( &self, input : impl AsRef< str > ) -> Result< Instruction >
    {
      let ( command_name, request ) = match input.as_ref().split_once( " " )
      {
        Some( entries ) => entries,
        None => ( input.as_ref(), "" ),
      };

      if !self.command_name_is_valid( command_name )
      {
        self.about_command_format();
        return Err( BasicError::new( "Invalid command" ) );
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
        return Err( BasicError::new( "Too many instructions" ) );
      }

      let subject = request.subject;
      let mut properties_map = request.map;
      let command_name = command_name.to_string();

      for ( key, value ) in &self.properties
      {
        properties_map.insert( key.clone(), value.clone() );
      }

      Ok( Instruction { command_name, subject, properties_map } )
    }
  }
}

//

crate::mod_interface!
{
  exposed use Instruction;
  exposed use InstructionParser;
  exposed use DefaultInstructionParser;
}
