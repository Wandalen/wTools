//! Messaging.

mod private
{

  /// Represents a single message in a conversation, either from a user or an assistant.
  ///
  /// It can optionally contain a dynamically-dispatched object that provides
  /// asynchronous access to the message generation process.
  #[ derive( Debug ) ]
  pub struct Message
  {
    /// The role of the message sender (e.g., "user", "assistant").
    pub role : String,
    /// A unique identifier for the message, if any.
    pub id : Option< String >,
    /// The main textual content of the message.
    pub content : String,
  }

  impl Message
  {
    /// Creates a new user-level or assistant-level message without generation data.
    pub fn new( role : &str, content : &str ) -> Self
    {
      Self
      {
        role : role.to_string(),
        id : None,
        content : content.to_string(),
      }
    }

  }

}

crate::mod_interface!
{
  orphan use private::
  {
    Message,
  };
}
