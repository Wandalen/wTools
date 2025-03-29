//!
//! List assistants in OpenAI API (command part).
//!

mod private
{

  use crate::*;
  use client::Client;
  use actions;
  use commands::TableConfig;

  /// List OpenAI assistants command.
  pub async fn command
  ( 
    client : &Client,
    table_config : TableConfig,
  )
  {
    let result = actions::openai_assistants_list::action( client, table_config ).await;

    match result
    {
      Ok ( report ) => println!( "{}", report ),
      Err ( error ) => println!( "{}", error )
    }
  }

}

crate::mod_interface!
{
  own use command;
}