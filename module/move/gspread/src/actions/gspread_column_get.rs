//!
//! Action for column get command.
//! 

mod private
{
  use crate::*;
  use gcore::Secret;
  use gcore::error::Result;
  use gcore::client::Client;
  use actions::gspread::get_column; 

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    column_id : &str
  ) -> Result< Vec< serde_json::Value > >
  {
    match get_column
    (
      client, 
      spreadsheet_id, 
      sheet_name, 
      column_id
    )
    .await
    {
      Ok( column ) => Ok( column ),
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    action
  };
}