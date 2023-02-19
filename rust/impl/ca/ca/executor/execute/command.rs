pub( crate ) mod private
{
  use crate::{ Routine, Value };

  use wtools::HashMap;

  /// Commands that be executed
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
  prelude use ExecutableCommand;
}
