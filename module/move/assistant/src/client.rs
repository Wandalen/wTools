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
  use cli::*;

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

  /// Execute `OpenAiCommand`.
  pub async fn execute_command(
    client : &Client,
    command : OpenAiCommand,
  ) -> Result < (), Box< dyn Error > >
  {
    match command
    {
      OpenAiCommand::Assistants( assistants_command ) =>
      {
        execute_assistants_command(client, assistants_command ).await?;
      }

      OpenAiCommand::Threads( threads_command ) =>
      {
        execute_threads_command(client, threads_command ).await?;
      }

      OpenAiCommand::Runs( runs_command ) =>
      {
        execute_runs_command(client, runs_command ).await?;
      }
    }

    Ok( () )
  }

  /// Execute `OpenAiAssistantsCommand`.
  async fn execute_assistants_command(
    client : &Client,
    command : OpenAiAssistantsCommand,
  ) -> Result < (), Box< dyn Error > >
  {
    match command
    {
      OpenAiAssistantsCommand::List =>
      {
        let assistants = client.list_assistant( None, None, None, None ).await?.data;

        println!("Assistants: ");

        if assistants.len() == 0
        {
          println!("No assistants.")
        }
        else
        {
          assistants
          .into_iter()
          .map(AssistantObjectWrap)
          .for_each(|a| 
          {
            println!("{:?}", a);
          });
        }
      }
    }

    Ok( () )
  }

  /// Execute `OpenAiThreadsCommand`.
  async fn execute_threads_command(
    client : &Client,
    command : OpenAiThreadsCommand,
  ) -> Result < (), Box< dyn Error > >
  {
    match command
    {
      OpenAiThreadsCommand::List =>
      {
        todo!()
      }
    }

    Ok( () )
  }

  /// Execute `OpenAiRunsCommand`.
  async fn execute_runs_command(
    client : &Client,
    command : OpenAiRunsCommand,
  ) -> Result < (), Box< dyn Error > >
  {
    match command
    {
      OpenAiRunsCommand::List =>
      {
        todo!()
      }
    }

    Ok( () )
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
    execute_command
  };
}
