pub( crate ) mod private
{
  use crate::Routine;

  use wtools::HashMap;

  /// Commands that be executed
  #[ derive( Debug, Clone ) ]
  pub struct ExecutableCommand
  {
    /// subjects values
    pub subjects : Vec< String >,
    /// properties value
    pub properties : HashMap< String, String >,
    /// function that will be called
    pub routine : Routine,
  }
}

//

crate::mod_interface!
{
  prelude use ExecutableCommand;
}
