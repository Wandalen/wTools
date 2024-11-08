//!
//! List assistants in OpenAI API (command part).
//!

mod private
{

  use crate::*;
  use client::Client;
  use actions;

  /// List OpenAI assistants command.
  pub async fn command
  ( 
    client : &Client,
    show_records_as_tables : bool,
  )
  {
    let result = actions::openai_assistants_list::action( client, show_records_as_tables ).await;

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