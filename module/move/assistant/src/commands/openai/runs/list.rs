mod private
{

  use crate::*;
  use client::Client;

  /// List runs in the thread in OpenAI API.
  pub async fn list
  ( 
    client : &Client, 
    thread_id : String,
    show_records_as_tables : bool,
  )
  {
    let result = actions::runs::list( client, thread_id, show_records_as_tables ).await;

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