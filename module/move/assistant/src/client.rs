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

  /// Options for configuring the OpenAI API client.
  #[ derive( Former, Debug ) ]
  pub struct ClientOptions
  {
    /// The API key for authenticating with the OpenAI API.
    pub api_key : Option< String >,
  }

  /// Creates a new OpenAI API client using the API key from the environment variable `OPENAI_API_KEY`.
  pub fn client() -> Result< Client, Box< dyn Error > >
  {
    let api_key = env::var( "OPENAI_API_KEY" )?;
    println!( "api_key : {}", api_key );
    Ok( Client::new( api_key ) )
  }

}

crate::mod_interface!
{
  exposed use
  {
    Client,
    ClientOptions,
    AssistantObject,
    client,
  };
}
