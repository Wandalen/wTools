//!
//! Action for the command "rows".
//!
//! Retrieves all rows from the specified Google Sheet, excluding the header.
//!


mod private
{
  use crate::*;
  use actions::gspread::get_rows;
  use gcore::Secret;
  use gcore::error::Result;
  use gcore::client::Client;

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< Vec< Vec < serde_json::Value > > >
  {
    match get_rows( client, spreadsheet_id, sheet_name ).await
    {
      Ok( rows ) => Ok( rows ),
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use action;
}
