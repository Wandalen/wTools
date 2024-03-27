pub( crate ) mod private
{
  use crate::*;
  use std::collections::HashMap;

  /// Represents a grammatically correct command with a phrase descriptor, a list of command subjects, and a set of command options.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ VerifiedCommand, Value };
  /// # use std::collections::HashMap;
  /// VerifiedCommand
  /// {
  ///   phrase : "command".to_string(),
  ///   args : vec![ Value::String( "subject_value".to_string() ), /* ... */ ],
  ///   properties : HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), Value::Number( 42.0 ) ),
  ///     /* ... */
  ///   ]),
  ///   internal_command : false,
  /// };
  /// ```
  ///
  /// In the above example, a `VerifiedCommand` instance is created with the name "command", a single subject "subject_value", and one property "prop_name" with a typed values.
  ///
  #[ derive( Debug ) ]
  pub struct VerifiedCommand< C = () >
  {
    /// Context for the command.
    ///
    /// This field holds an optional value of type `C` which represents the context for the command.
    /// The `Option` type is used to indicate that the value may or may not be present.
    pub context : Option< C >,
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Flag indicating whether a command is internal or not.
    pub internal_command : bool,
    /// Command subjects.
    pub args : Args,
    /// Command options.
    pub properties : Props,
  }

}

//

crate::mod_interface!
{
  exposed use VerifiedCommand;
}

// qqq : use orphan instead of exposed for ALL files in the folder, dont use prelude for structs