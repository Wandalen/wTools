//! Messaging.

mod private
{
  use std::
  {
    any::Any,
    collections::HashMap,
    ops::{ Deref, DerefMut },
    pin::Pin,
    task::{ Context, Poll },
  };

  use futures_core::
  {
    future::BoxFuture,
    stream::Stream,
  };

  /// Represents potential errors that may occur during LLM invocations.
  #[ derive( Debug ) ]
  pub enum LlmError
  {
    /// Catch-all variant for unknown or miscellaneous errors.
    Unknown( String ),
  }

  /// Alias for JSON-based values commonly used for LLM parameters.
  pub type Value = serde_json::Value;

  /// A trait defining the behaviors of a generated message from the LLM.
  ///
  /// Different LLM backends can implement this in various ways to handle
  /// partial streaming, token counting, or advanced metadata.
  pub trait MessageGenerated: std::fmt::Debug + Send + Sync
  {
    /// Exposes a method for downcasting implementations if needed.
    fn as_any( &self ) -> &dyn Any;

    /// Retrieves the number of tokens used so far, if known.
    fn tokens_used( &self ) -> Option<u64>;

    /// Retrieves the model identifier, if known.
    fn model( &self ) -> Option< &str >;

    /// Asynchronously waits for message generation to complete.
    fn wait_for_completion< 'a >( &'a mut self ) -> BoxFuture< 'a, Result< (), LlmError > >;

    /// Provides a stream of partial outputs (tokens, chunks, etc.), if available.
    fn partial_stream< 'a >( &'a mut self ) -> Pin< Box< dyn Stream< Item = String > + 'a + Send > >;
  }

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

  impl MessageGeneration
  {
    /// Creates a new generator with optional known tokens_used/model.
    pub fn new( tokens_used : Option<u64>, model : Option< String > ) -> Self
    {
      Self
      {
        tokens_used_val : tokens_used,
        model_name : model,
        complete : true,
        partial_content : Vec::new(),
      }
    }
  }

  impl MessageGenerated for MessageGeneration
  {
    fn as_any( &self ) -> &dyn Any
    {
      self
    }

    fn tokens_used( &self ) -> Option<u64>
    {
      self.tokens_used_val
    }

    fn model( &self ) -> Option< &str >
    {
      self.model_name.as_deref()
    }

    fn wait_for_completion< 'a >( &'a mut self ) -> BoxFuture< 'a, Result< (), LlmError > >
    {
      Box::pin( async move
      {
        // In a real implementation, you might poll an API or
        // wait on a background task. Here, we do nothing.
        self.complete = true;
        Ok(())
      })
    }

    fn partial_stream< 'a >( &'a mut self ) -> Pin< Box< dyn Stream< Item = String > + 'a + Send > >
    {
      // Transform the stored partial_content into a simple stream of strings.
      let iter = self.partial_content.drain(..);
      Box::pin( futures_util::stream::iter( iter ) )
    }
  }

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
    /// Optional handle to the generation process and metadata.
    pub response : Option<Box< dyn MessageGenerated > >,
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
        response : None,
      }
    }

    /// Creates a new message with the provided generation object.
    pub fn with_generation
    (
      role : &str,
      content : &str,
      gen : Box< dyn MessageGenerated >
    ) -> Self
    {
      Self
      {
        role : role.to_string(),
        id : None,
        content : content.to_string(),
        response : Some( gen ),
      }
    }
  }

  /// A thin wrapper around a vector of messages to store a conversation log.
  #[ repr( transparent ) ]
  #[ derive( Debug ) ]
  pub struct Messages( Vec<Message> );

  impl Messages
  {
    /// Creates a new, empty list of messages.
    pub fn new() -> Self
    {
      Self( Vec::new() )
    }
  }

  impl Deref for Messages
  {
    type Target = Vec<Message>;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl DerefMut for Messages
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

  /// Represents a conversation that holds a series of messages and relevant parameters.
  #[ derive( Debug ) ]
  pub struct Conversation
  {
    /// A collection of messages exchanged within this conversation.
    messages : Messages,
    /// A collection of key-value parameters to customize LLM behavior.
    parameters : HashMap< String, Value >,
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
    /// Returns `Err( LlmError::Unknown( ...) )` for unexpected conditions.
    pub fn parameter_set
    (
      &mut self,
      key : &str,
      val : Value
    ) -> Result< (), LlmError >
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
    /// Returns `Err( LlmError::Unknown( ... ) )` for unexpected conditions.
    pub fn context
    (
      &mut self,
      context : Message
    ) -> Result< (), LlmError >
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
    /// Returns `Err( LlmError::Unknown( ... ) )` for unexpected conditions.
    pub fn message_add
    (
      &mut self,
      message : Message
    ) -> Result< (), LlmError >
    {
      self.messages.push( message );
      Ok(())
    }
  }

  /// A trait defining asynchronous single-message retrieval from the LLM.
  pub trait MessageSend
  {
    /// Sends the current conversation state to the LLM and returns a single generated message.
    ///
    /// # Returns
    ///
    /// The newly generated message (with optional generation object inside), or an error if something went wrong.
    // async fn send( &self ) -> Result< Message, LlmError >;
    fn send( &self ) -> impl std::future::Future< Output = Result< Message, LlmError > > + Send;
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
    // async fn stream( &self ) -> Result< Self::MessagesStream, LlmError >;
    fn stream( &self ) -> impl std::future::Future< Output = Result< Self::MessagesStream, LlmError > > + Send;
  }

  impl MessageSend for Conversation
  {
    async fn send( &self ) -> Result< Message, LlmError >
    {
      // Example: returning a new message with a generation object
      Ok( Message
      {
        role : "assistant".to_string(),
        id : None,
        content : "Mocked LLM Response".to_string(),
        response : Some( Box::new( MessageGeneration
        {
          tokens_used_val : Some( 42 ),
          model_name : Some( "MockedModel".to_string() ),
          complete : true,
          partial_content : vec![],
        })),
      })
    }
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

    async fn stream( &self ) -> Result< Self::MessagesStream, LlmError >
    {
      // Gather all the message-generation objects from the conversation.
      let mut all = Vec::new();
      for msg in &self.messages.0
      {
        if let Some( ref gen ) = msg.response
        {
          // Attempt to downcast to our concrete MessageGeneration structure.
          if let Some( real_gen ) = gen.as_any().downcast_ref::< MessageGeneration >()
          {
            all.push( real_gen.clone() );
          }
        }
      }
      Ok( ConversationGenerationStream
      {
        gens : all.into_iter(),
      })
    }
  }
}

crate::mod_interface!
{
  own use
  {
    private::LlmError,
    private::Value,
    private::MessageGenerated,
    private::MessageGeneration,
    private::Message,
    private::Messages,
    private::Conversation,
    private::MessageSend,
    private::MessageStream,
    private::ConversationGenerationStream,
  };
}
