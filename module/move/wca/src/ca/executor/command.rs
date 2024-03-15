pub( crate ) mod private
{
  use crate::*;
  use std::collections::HashMap;

  /// Represents a command that can be executed, with a list of command subjects and a set of command options, and a callback function that defines the command logic.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ ExecutableCommand_, Routine, Value };
  /// # use std::collections::HashMap;
  /// ExecutableCommand_
  /// {
  ///   subjects : vec![ Value::String( "subject_value".to_string() ), /* ... */ ],
  ///   properties : HashMap::from_iter
  ///   ([
  ///     ( "prop_name".to_string(), Value::Number( 42.0 ) ),
  ///     /* ... */
  ///   ]),
  ///   routine : Routine::new( |( args, props )| Ok( () ) )
  /// };
  /// ```
  ///
  #[ derive( Debug, Clone ) ]
  pub struct ExecutableCommand_
  {
    /// subjects values
    pub subjects : Vec< Value >,
    /// properties value
    pub properties : HashMap< String, Value >,
    /// function that will be called
    pub routine : Routine,
  }
  // qqq : for Bohdan : rid off the structure. VerifiedCommand should be used and passed to userland.

}

//

crate::mod_interface!
{
  exposed use ExecutableCommand_;
}

// qqq : use orphan instead of exposed for ALL files in the folder, dont use prelude for structs