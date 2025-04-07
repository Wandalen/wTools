//! Messaging.

mod private
{
  use crate::*;
  use messaging::{ Conversation };

  use std::
  {
    pin::Pin,
    task::{ Context, Poll },
  };

  use futures_core::
  {
    // future::BoxFuture,
    stream::Stream,
  };

  /// A default-like implementation of the generation object.
  ///
  /// This example doesn't truly stream anything, but it showcases how you can
  /// store metadata and optionally produce partial content.
  #[derive(Debug, Clone)]
  pub struct MessageGeneration
  {
    /// The total number of tokens used for generation, if known.
    pub tokens_used_val : Option< u64 >,
    /// The model used to generate this message, if known.
    pub model_name : Option< String >,
    /// Whether the generation is considered final/complete.
    pub complete : bool,
    /// A queue of partial tokens or chunks for streaming (mocked).
    pub partial_content : Vec< String >,
  }

  /// A trait defining asynchronous stream-based retrieval from the LLM.
  ///
  /// The associated type is a Stream of concrete items (MessageGeneration).
  pub trait MessageStream
  {
    /// The type of the asynchronous stream that yields generated results one by one.
    type MessagesStream : Stream< Item = MessageGeneration > + Send + 'static;

    /// Sends the current conversation state to the LLM and returns a stream of generated message chunks.
    ///
    /// # Returns
    ///
    /// An asynchronous stream of MessageGeneration objects, or an error if something went wrong.
    // async fn stream( &self ) -> Result< Self::MessagesStream, messaging::Error >;
    fn stream( &self ) -> impl std::future::Future< Output = Result< Self::MessagesStream, messaging::Error > > + Send;
  }

  /// The concrete stream type that yields MessageGeneration objects.
  #[ derive( Debug ) ]
  pub struct ConversationGenerationStream
  {
    gens : std::vec::IntoIter< MessageGeneration >,
  }

  impl Stream for ConversationGenerationStream
  {
    type Item = MessageGeneration;

    fn poll_next
    (
      self : Pin<&mut Self>,
      _cx : &mut Context<'_>
    ) -> Poll< Option< Self::Item > >
    {
      let me = self.get_mut();
      Poll::Ready( me.gens.next() )
    }
  }

  impl MessageStream for Conversation
  {
    type MessagesStream = ConversationGenerationStream;

    async fn stream( &self ) -> Result< Self::MessagesStream, messaging::Error >
    {
      let all = Vec::new();
      Ok( ConversationGenerationStream
      {
        gens : all.into_iter(),
      })
    }
  }

}

crate::mod_interface!
{
  orphan use private::
  {
    MessageGeneration,
    MessageStream,
    ConversationGenerationStream,
  };
}
