pub( crate ) mod private
{
  use crate::*;

  use { Command };
  use std::collections::HashMap;
  use former::Former;

  // qqq : `Former` does not handle this situation well

  // /// A collection of commands.
  // ///
  // /// This structure holds a hashmap of commands where each command is mapped to its name.
  // #[ derive( Debug, Former ) ]
  // pub struct Dictionary( HashMap< String, Command > );

  /// A collection of commands.
  ///
  /// This structure holds a hashmap of commands where each command is mapped to its name.
  #[ derive( Debug, Default, Former, Clone ) ]
  pub struct Dictionary
  {
    #[ setter( false ) ]
    pub( crate ) commands : HashMap< String, Command >,
  }

  // qqq : IDK how to integrate it into the `CommandsAggregatorFormer`
  //
  impl DictionaryFormer
  {
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.storage.commands.unwrap_or_default();
      commands.extend([( command.phrase.clone(), command )]);
      self.storage.commands = Some( commands );

      self
    }
  }

  impl Dictionary
  {
    /// Registers a command into the command list.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be registered.
    pub fn register( &mut self, command : Command ) -> Option< Command >
    {
      self.commands.insert( command.phrase.clone(), command )
    }

    /// Retrieves the command with the specified `name` from the `commands` hashmap.
    ///
    /// # Arguments
    ///
    /// * `name` - A reference to the name of the command to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the command with the specified `name`, if it exists.
    /// Returns `None` if no command with the specified `name` is found.
    pub fn command< Name >( &self, name : &Name ) -> Option< &Command >
    where
      String : std::borrow::Borrow< Name >,
      Name : std::hash::Hash + Eq,
    {
      self.commands.get( name )
    }
  }
}

//

crate::mod_interface!
{
  exposed use Dictionary;
}
