pub( crate ) mod private
{
  use std::borrow::Cow;
  use former::Former;
  use nom::
  {
    bytes::complete::take_while,
    IResult,
  };

  /// `Parser` provides parsing command strings into `RawCommand` objects.
  /// It allows you to specify the symbols that will be used to interpret the command string, such as the command delimiter, property delimiter, and namespace delimiter.
  ///
  /// ```
  /// use wca::{ Parser, CommandParser };
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// /// Configure the parser
  /// let parser = Parser::former()
  /// .command_prefix( '.' )
  /// .prop_delimeter( ':' )
  /// .form();
  ///
  /// /// Parse a command from a` string
  /// let raw_command = parser.command( ".command subject_value prop_name:prop_value" )?;
  /// # Ok( () ) }
  /// ```
  ///
  /// In the above example, a `Parser` object is created and configured to accept commands with a `.` prefix and `:` delimiters for properties.
  ///
  /// Note that `Parser` uses `CommandParser` trait to parse commands from user input( You can also use `NamespaceParser` to parse namespaces, or `ProgramParser` to parse programs from string ).
  ///
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct Parser
  {
    /// Symbol that will be interpreted as the beginning of a command
    ///
    /// command_delimiter = `.`
    ///
    /// ".command" -> Command( "command" )
    #[ default( '.' ) ]
    pub command_prefix : char,
    /// Symbol that will be interpreted as a separator for the name and value of the property
    ///
    /// prop_delimiter = `:`
    ///
    /// "prop:value" -> ( "prop", "value" )
    #[ default( ':' ) ]
    pub prop_delimeter : char,
    /// String that will be interpreted as a separator for namespaces
    ///
    /// namespace_delimiter = ".also"
    ///
    /// "<commands1> .also <commands2>" -> Namespace( < commands1 > ), Namespace( < commands2 > )
    #[ default( ".also" ) ]
    pub namespace_delimeter : Cow< 'static, str >,
  }

  /// Parses first word from string. All characters before first space
  pub fn any_word( input : &str ) -> IResult< &str, &str >
  {
    take_while( | c : char | !c.is_whitespace() )( input )
  }
}

//

crate::mod_interface!
{
  exposed use Parser;
  protected use any_word;
}
