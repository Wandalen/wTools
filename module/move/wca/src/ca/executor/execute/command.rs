pub( crate ) mod private
{
  use crate::*;
  use std::collections::HashMap;

  /// Represents a command that can be executed, with a list of command subjects and a set of command options, and a callback function that defines the command logic.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ ExecutableCommand, Routine, Value };
  /// # use std::collections::HashMap;
  /// ExecutableCommand
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
  pub struct ExecutableCommand
  {
    /// subjects values
    pub subjects : Vec< Value >,
    /// properties value
    pub properties : HashMap< String, Value >,
    /// function that will be called
    pub routine : Routine,
  }
}

//

crate::mod_interface!
{
  exposed use ExecutableCommand;
}
