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
  ///   subjects : vec![ Value::String( "subject_value".to_string() ), /* ... */ ],
  ///   properties : HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), Value::Number( 42.0 ) ),
  ///     /* ... */
  ///   ])
  /// };
  /// ```
  ///
  /// In the above example, a `VerifiedCommand` instance is created with the name "command", a single subject "subject_value", and one property "prop_name" with a typed values.
  ///
  #[ derive( Debug ) ]
  pub struct VerifiedCommand
  {
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Command subjects.
    pub subjects : Vec< Value >,
    /// Command options.
    pub properties : HashMap< String, Value >,
  }

}

//

crate::mod_interface!
{
  exposed use VerifiedCommand;
}

// qqq : use orphan instead of exposed for ALL files in the folder, dont use prelude for structs