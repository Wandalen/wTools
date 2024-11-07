//!
//! Client of API.
//!

/// Internal namespace.
mod private
{

  pub use openai_api_rs::v1::
  {
    api::OpenAIClient as Client,
    // api::Client,
    assistant::AssistantObject,
  };

  use std::
  {
    env,
    error::Error,
  };

  use former::Former;

  use crate::*;
  use secret::Secret;

  /// Creates a new OpenAI API client using the secrets.
  pub fn client(secrets: &Secret) -> Result< Client, Box< dyn Error > >
  {
    Ok( Client::new( secrets.OPENAI_API_KEY.clone() ) )
  }

}

crate::mod_interface!
{
  exposed use
  {
    Client,
    AssistantObject,
    client
  };
}
