mod private
{
  use crate::*;

  /// Represents a grammatically correct command with a phrase descriptor, a list of command subjects, and a set of command options.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ VerifiedCommand, Value, Args, Props };
  /// # use std::collections::HashMap;
  /// VerifiedCommand
  /// {
  ///   phrase : "command".to_string(),
  ///   internal_command : false,
  ///   args : Args( vec![ Value::String( "subject_value".to_string() ), /* ... */ ] ),
  ///   props : Props( HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), Value::Number( 42.0 ) ),
  ///     /* ... */
  ///   ]))
  /// };
  /// ```
  ///
  /// In the above example, a `VerifiedCommand` instance is created with the name "command", a single subject "subject_value", and one property "prop_name" with a typed values.
  ///
  #[ derive( Debug, Clone ) ]
  pub struct VerifiedCommand
  {
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Flag indicating whether a command is internal or not.
    pub internal_command : bool,
    /// Command subjects.
    pub args : Args,
    /// Command options.
    pub props : Props,
  }

}

//

crate::mod_interface!
{
  exposed use VerifiedCommand;
}

// qqq : use orphan instead of exposed for ALL files in the folder, dont use prelude for structs