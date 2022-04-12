
pub( crate ) mod internal
{
  use woptions::*;
  use std::collections::HashMap;

  ///
  /// Parsed request data.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug, Default, PartialEq ) ]
  pub struct Request< 'a >
  {
    /// Original request string.
    pub original : &'a str,
    /// Delimeter for pairs `key:value`.
    pub key_val_delimeter : &'a str,
    /// Delimeter for commands.
    pub commands_delimeter : &'a str,
    /// Parsed subject of first command.
    pub subject : &'a str,
    /// All subjects of the commands in request.
    pub subjects : Vec<&'a str>,
    /// Options map of first command.
    pub map : HashMap<String, String>,
    /// All options maps of the commands in request.
    pub maps : Vec<HashMap<String, String>>,
  }

  ///
  /// Options for parser.
  ///

  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  #[ perform( fn parse( mut self ) -> Request< 'a > ) ]
  pub struct ParseOptions< 'a >
  {
    #[ default( "" ) ]
    src : &'a str,
    #[ default( ":" ) ]
    key_val_delimeter : &'a str,
    #[ default( ";" ) ]
    commands_delimeter : &'a str,
    #[ default( true ) ]
    quoting : bool,
    #[ default( true ) ]
    unquoting : bool,
    #[ default( true ) ]
    parsing_arrays : bool,
    #[ default( false ) ]
    several_values : bool,
    #[ default( false ) ]
    subject_win_paths_maybe : bool,
  }

  ///
  /// Adapter for ParseOptions.
  ///

  pub trait ParseOptionsAdapter< 'a >
  {
    /// A string to parse.
    fn src( &self ) -> &'a str;
    /// A delimeter for pairs `key:value`.
    fn key_val_delimeter( &self ) -> &'a str;
    /// Delimeter for commands.
    fn commands_delimeter( &self ) -> &'a str;
    /// Quoting of strings.
    fn quoting( &self ) -> bool;
    /// Unquoting of string.
    fn unquoting( &self ) -> bool;
    /// Parse arrays of values.
    fn parsing_arrays( &self ) -> bool;
    /// Append to a vector a values.
    fn several_values( &self ) -> bool;
    /// Parse subject on Windows taking into account colon in path.
    fn subject_win_paths_maybe( &self ) -> bool;

    /// Do parsing.
    fn parse( self ) -> Request< 'a >
    where
      Self : Sized,
    {
      Request::default()
    }
  }

  impl< 'a > ParseOptionsAdapter< 'a > for ParseOptions< 'a >
  {
    fn src( &self ) -> &'a str
    {
      self.src
    }
    fn key_val_delimeter( &self ) -> &'a str
    {
      self.key_val_delimeter
    }
    fn commands_delimeter( &self ) -> &'a str
    {
      self.commands_delimeter
    }
    fn quoting( &self ) -> bool
    {
      self.quoting
    }
    fn unquoting( &self ) -> bool
    {
      self.unquoting
    }
    fn parsing_arrays( &self ) -> bool
    {
      self.parsing_arrays
    }
    fn several_values( &self ) -> bool
    {
      self.several_values
    }
    fn subject_win_paths_maybe( &self ) -> bool
    {
      self.subject_win_paths_maybe
    }

    fn parse( mut self ) -> Request< 'a >
    where
      Self : Sized,
    {
      let mut result = Request::default();

      result.original = self.src();
      result.key_val_delimeter = self.key_val_delimeter();
      result.commands_delimeter = self.commands_delimeter();

      println!("'{}' '{}'", self.src, self.src.trim());
      self.src = self.src.trim();

      if self.src.is_empty()
      {
        return result;
      }

      result
    }
  }

  ///
  /// Function to parse a string with command request.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn request_parse<'a>() -> ParseOptionsFormer<'a>
  {
    ParseOptions::former()
  }
}

/// Owned namespace of the module.
pub mod own
{
  use super::internal as i;

  pub use i::Request;
  pub use i::ParseOptions;
  pub use i::ParseOptionsAdapter;
  pub use i::request_parse;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;

  pub use i::ParseOptionsAdapter;
  pub use i::request_parse;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::internal as i;

  pub use i::ParseOptionsAdapter;
}
