pub( crate ) mod private
{
  use crate::*;
  use former::Former;
  use indexmap::IndexMap;
  use iter_tools::Itertools;

  // qqq : `Former` does not handle this situation well

  // /// A collection of commands.
  // ///
  // /// This structure holds a hashmap of commands where each command is mapped to its name.
  // #[ derive( Debug, Former ) ]
  // pub struct Dictionary( HashMap< String, Command > );

  /// A collection of commands.
  ///
  /// This structure holds a btreemap of commands where each command is mapped to its name.
  #[ derive( Debug, Default, Former, Clone ) ]
  pub struct Dictionary
  {
    #[ scalar( setter = false ) ]
    pub( crate ) commands : IndexMap< String, Command >,
    #[ scalar( setter = false ) ]
    pub( crate ) order : Order,
  }

  // qqq : IDK how to integrate it into the `CommandsAggregatorFormer`
  //
  impl DictionaryFormer
  {
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.storage.commands.unwrap_or_default();
      commands.insert( command.phrase.clone(), command );
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

    /// Find commands that match a given name part.
    ///
    /// This function accepts a `name_part` parameter which is of generic type `NamePart`.
    /// The `NamePart` type must implement the `AsRef<str>` trait.
    ///
    /// # Arguments
    ///
    /// * `name_part` - The name part to match against command phrases.
    ///
    /// # Returns
    ///
    /// A vector of references to `Command` that match the given `name_part`.
    pub fn search< NamePart >( &self, name_part : NamePart ) -> Vec< &Command >
    where
      NamePart : AsRef< str >,
    {
      self.commands.values().filter( | command | command.phrase.starts_with( name_part.as_ref() ) ).collect()
    }

    /// asd
    pub fn commands( &self ) -> Vec< ( &String, &Command ) >
    {
      match self.order
      {
        Order::Nature =>
        {
          self.commands.iter().map( | ( key, value ) | ( key, value ) ).collect()
        }
        Order::Lexicography =>
        {
          self.commands.iter().map( | ( key, value ) | ( key, value ) ).sorted_by_key( | ( key, _ ) | *key ).collect()
        }
      }
    }
  }
}

//

crate::mod_interface!
{
  exposed use Dictionary;
}
