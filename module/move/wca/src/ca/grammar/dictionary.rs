pub( crate ) mod private
{
  use crate::*;

  use { Command };
  use std::collections::HashMap;

  /// A collection of commands.
  ///
  /// This structure holds a hashmap of commands where each command is mapped to its name.
  pub struct Dictionary( HashMap< String, Command > );
}

//

crate::mod_interface!
{
  exposed use Dictionary;
}
