//!
//! List runs in OpenAI API (command part).
//!

mod private
{

  use crate::*;
  use client::Client;
  use actions;
  use commands::TableConfig;

  /// List runs in the thread in OpenAI API.
  pub async fn command
  ( 
    client : &Client, 
    thread_id : String,
    table_config : TableConfig,
  )
  {
    let result = actions::openai_runs_list::action( client, thread_id, table_config ).await;

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