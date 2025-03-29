//!
//! Action for the command "header".
//!
//! Retrieves the header (first row) from the specified Google Sheet.
//!


mod private
{
  use crate::*;
  use actions::gspread::get_header;
  use gcore::Secret;
  use gcore::client::Client;
  use gcore::error::Result;

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< Vec< serde_json::Value > >
  {
    match get_header( client, spreadsheet_id, sheet_name ).await
    {
      Ok( result ) => Ok( result ),
      Err( error ) => Err( error )
    }
  }

}

crate::mod_interface!
{
  own use action;
}