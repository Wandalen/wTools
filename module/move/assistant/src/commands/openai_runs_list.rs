//!
//! List runs in OpenAI API (command part).
//!

mod private
{

  use crate::*;
  use client::Client;
  use actions;

  /// List runs in the thread in OpenAI API.
  pub async fn command
  ( 
    client : &Client, 
    thread_id : String,
    show_records_as_tables : bool,
  )
  {
    let result = actions::openai_runs_list::action( client, thread_id, show_records_as_tables ).await;

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