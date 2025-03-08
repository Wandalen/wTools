//!
//! Action for clear command.
//! 

mod private
{
  use crate::*;
  use gcore::Secret;
  use gcore::error::Result;
  use gcore::client::Client;
  use actions::gspread::clear;

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< String >
  {
    match clear( client, spreadsheet_id, sheet_name ).await
    {
      Ok( response ) => Ok( response.cleared_range.unwrap_or_default() ),
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