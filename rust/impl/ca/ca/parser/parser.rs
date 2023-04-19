pub( crate ) mod private
{
  use std::borrow::Cow;
  use former::Former;
  use nom::
  {
    bytes::complete::take_while,
    IResult,
  };

  /// Parser configuration
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct Parser
  {
    /// Symbol that will be interpreted as the beginning of a command
    /// 
    /// command_delimiter = '.'
    /// ".command" -> Command( "command" )
    #[ default( '.' ) ]
    pub command_prefix : char,
    /// Symbol that will be interpreted as a separator for the name and value of the property
    /// 
    /// prop_delimiter = ':'
    /// "prop:value" -> ( "prop", "value" )
    #[ default( ':' ) ]
    pub prop_delimeter : char,
    /// String that will be interpreted as a separator for namespaces
    /// 
    /// namespace_delimiter = ".also"
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
  prelude use Parser;
  protected use any_word;
}
