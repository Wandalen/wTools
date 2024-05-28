pub( crate ) mod private
{
  use crate::*;
  use former::Former;
  use wtools::Itertools;
  use std::cmp::Ordering;
  use std::collections::BTreeMap;

  // qqq : `Former` does not handle this situation well

  // /// A collection of commands.
  // ///
  // /// This structure holds a hashmap of commands where each command is mapped to its name.
  // #[ derive( Debug, Former ) ]
  // pub struct Dictionary( HashMap< String, Command > );

  /// Command name with id.
  #[ derive( Debug, Default, Clone, Eq ) ]
  pub struct CommandName
  {
    /// id of command.
    pub( crate ) id : usize,
    /// Name of command.
    pub name : String,
  }

  impl std::borrow::Borrow< String > for CommandName
  {
    fn borrow( &self ) -> &String 
    {
      &self.name
    }
  }

  impl Ord for CommandName
  {
    fn cmp( &self, other : &Self ) -> Ordering
    {
      if self.name == other.name
      {
        Ordering::Equal
      }
      else 
      { 
        self.id.cmp( &other.id )
      }
    }
  }

  impl PartialEq< Self > for CommandName
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.name.eq( &other.name )
    }
  }

  impl PartialOrd for CommandName
  {
    fn partial_cmp( &self, other : &Self ) -> Option< Ordering >
    {
      self.id.partial_cmp( &other.id )
    }
  }

  /// A collection of commands.
  ///
  /// This structure holds a btreemap of commands where each command is mapped to its name.
  #[ derive( Debug, Default, Former, Clone ) ]
  pub struct Dictionary
  {
    #[ scalar( setter = false, hint = false ) ]
    pub( crate ) commands : BTreeMap< CommandName, Command >,
    #[ scalar( setter = false, hint = false ) ]
    dictionary_last_id : usize,
    pub( crate ) order : Order,
  }

  // qqq : IDK how to integrate it into the `CommandsAggregatorFormer`
  //
  impl DictionaryFormer
  {
    pub fn command( mut self, command : Command ) -> Self
    {
      let mut commands = self.storage.commands.unwrap_or_default();
      self.storage.dictionary_last_id = Some( self.storage.dictionary_last_id.unwrap_or_default() + 1 );
      let name = CommandName { id : self.storage.dictionary_last_id.unwrap(), name : command.phrase.clone() };
      commands.insert( name, command );
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
      self.dictionary_last_id += 1;
      let name = CommandName { id : self.dictionary_last_id, name : command.phrase.clone() };
      self.commands.insert( name, command )
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
      Name : std::hash::Hash + Eq + Ord + ToString,
    {
      self.commands.iter().find( | ( k, _ ) | k.name == name.to_string() ).map( | ( _,  v ) | v )
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
          self.commands.iter().map( | ( key, value ) | ( &key.name, value ) ).collect()
        }
        Order::Lexicography =>
        {
          self.commands.iter().map( | ( key, value ) | ( &key.name, value ) ).sorted_by_key( | ( key, _ ) | *key ).collect()
        }
      }
    }
  }
}

//

crate::mod_interface!
{
  exposed use Dictionary;
  exposed use CommandName;
}
