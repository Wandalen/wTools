pub( crate ) mod private
{
  use std::collections::HashMap;
  
  /// Represents a program that contains one or more namespaces, where each namespace contains a list of commands.
  ///
  /// A `Program` consists of one or more commannd
  ///
  /// The program can be executed by iterating over each commands and executing it
  // aaa : xxx : for Bohdan : Commands should be here instead of Namespace
  // aaa : remove concept Namespace
  // aaa : introduce concept Dictionary for grammar
  // aaa : done
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct Program< Command >
  {
    /// list of namespaces with commands
    pub commands : Vec< Command >,
  }
  
  /// Represents a parsed command that has been extracted from an input string by a `Parser`.
  ///
  /// The `ParsedCommand` struct is designed to be flexible and allow for a wide variety of commands to be parsed and represented. However, this flexibility also means that a `ParsedCommand` may contain invalid or unexpected data.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::ParsedCommand;
  /// # use std::collections::HashMap;
  /// ParsedCommand
  /// {
  ///   name : "command".to_string(),
  ///   subjects : vec![ "subject_value".to_string(), /* ... */ ],
  ///   properties : HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), "raw_prop_value".to_string() ),
  ///     /* ... */
  ///   ])
  /// };
  /// ```
  ///
  /// In the above example, a `ParsedCommand` instance is created with the name "command", a single subject "subject_value", and one property "prop_name" with a raw value of "raw_prop_value".
  ///
  #[ derive( Default, Debug, Clone, PartialEq, Eq ) ]
  pub struct ParsedCommand
  {
    /// name of command without delimiter
    pub name : String,
    /// list of all subjects for the command
    pub subjects : Vec< String >,
    /// dictionary of properties. Each property has a name and a raw value
    pub properties : HashMap< String, String >
  }
}

//

crate::mod_interface!
{
  exposed use Program;
  exposed use ParsedCommand;
}
