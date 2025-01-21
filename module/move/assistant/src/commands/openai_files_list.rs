//!
//! List files in OpenAI API (command part).
//!

mod private
{

  use crate::*;
  use client::Client;
  use actions;
  use commands::TableConfig;

  /// List files in your OpenAI API.
  pub async fn command
  ( 
    client : &Client,
    table_config : TableConfig,
  )
  {
    let result = actions::openai_files_list::action( client, table_config ).await;

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