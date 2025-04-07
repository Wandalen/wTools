//! Messaging.

mod private
{
  use crate::*;
  use messaging::{ Value, Message, Messages, Error };

  use std::
  {
    collections::HashMap,
  };

  /// Represents a conversation that holds a series of messages and relevant parameters.
  #[ derive( Debug ) ]
  pub struct Conversation
  {
    /// A collection of messages exchanged within this conversation.
    pub messages : Messages,
    /// A collection of key-value parameters to customize LLM behavior.
    pub parameters : HashMap< String, Value >,
  }

  impl Conversation
  {
    /// Creates a new empty conversation with no parameters.
    pub fn new() -> Self
    {
      Self
      {
        messages : Messages::new(),
        parameters : HashMap::new(),
      }
    }

    /// Adds or updates a specific parameter for the LLM.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the parameter (e.g., "temperature").
    /// * `val` - The JSON-based value to store for this parameter.
    ///
    /// # Errors
    ///
    /// Returns `Err( Error::Unknown( ...) )` for unexpected conditions.
    pub fn parameter_set
    (
      &mut self,
      key : &str,
      val : Value
    ) -> Result< (), Error >
    {
      self.parameters.insert( key.to_string(), val );
      Ok(())
    }

    /// Inserts a context (usually from the conversation's system or user) into the conversation log.
    ///
    /// # Arguments
    ///
    /// * `context` - A new message describing the context (e.g., system instructions).
    ///
    /// # Errors
    ///
    /// Returns `Err( Error::Unknown( ... ) )` for unexpected conditions.
    pub fn context
    (
      &mut self,
      context : Message
    ) -> Result< (), Error >
    {
      self.messages.push( context );
      Ok(())
    }

    /// Adds a user or assistant message to the conversation.
    ///
    /// # Arguments
    ///
    /// * `message` - The new message to append to the conversation log.
    ///
    /// # Errors
    ///
    /// Returns `Err( Error::Unknown( ... ) )` for unexpected conditions.
    pub fn message_add
    (
      &mut self,
      message : Message
    ) -> Result< (), Error >
    {
      self.messages.push( message );
      Ok(())
    }
  }

}

crate::mod_interface!
{
  orphan use private::
  {
    Conversation,
  };
}
