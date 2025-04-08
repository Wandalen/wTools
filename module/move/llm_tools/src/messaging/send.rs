//! Messaging.

mod private
{
  use crate::*;
  use messaging::{ Message, Conversation };

  /// A trait defining asynchronous single-message retrieval from the LLM.
  pub trait MessageSend
  {

    /// Sends the current conversation state to the LLM and returns a single generated message.
    ///
    /// # Returns
    ///
    /// The newly generated message (with optional generation object inside), or an error if something went wrong.
    // async fn send( &self ) -> Result< Message, messaging::Error >;
    fn send( &self ) -> impl std::future::Future< Output = Result< Message, messaging::Error > > + Send;

  }

  impl MessageSend for Conversation
  {
    async fn send( &self ) -> Result< Message, messaging::Error >
    {
      // Example: returning a new message with a generation object
      Ok( Message
      {
        role : "assistant".to_string(),
        id : None,
        content : "Mocked LLM Response".to_string(),
      })
    }
  }

}

crate::mod_interface!
{
  orphan use private::
  {
    MessageSend,
  };
}
