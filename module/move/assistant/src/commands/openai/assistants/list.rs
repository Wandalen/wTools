mod private
{

  use crate::*;
  use client::Client;

  /// List OpenAI assistants command.
  pub async fn list
  ( 
    client : &Client,
    show_records_as_tables : bool,
  )
  {
    let result = actions::assistants::list( client, show_records_as_tables ).await;

    match result
    {
      Ok ( report ) => println!( "{}", report ),
      Err ( error ) => println!( "{}", error )
    }
  }

}

crate::mod_interface!
{
  orphan use list;
}