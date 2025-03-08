//!
//! Action which calls `get_row` function.
//! 

mod private
{
  use crate::*;
  use actions::gspread::get_row;
  use gcore::Secret;
  use gcore::error::Result;
  use gcore::client::Client;

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : serde_json::Value
  ) -> Result< Vec< serde_json::Value > >
  {
    match get_row
    (
      client, 
      spreadsheet_id, 
      sheet_name, 
      row_key
    )
    .await
    {
      Ok( row ) => Ok( row ),
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