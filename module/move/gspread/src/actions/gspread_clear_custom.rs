//!
//! Action for clear custom command.
//! 

mod private
{
  use crate::*;
  use gcore::Secret;
  use gcore::{ client::Client, error::Result };
  use actions::gspread::clear_by_custom_row_key; 
  use actions::utils::{ parse_key_by, parse_on_find };

  pub async fn action<S: Secret>
  (
    client : &Client<'_, S>,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : &str,
    on_find : &str
  ) -> Result< Vec< String > >
  {
    let key_by = parse_key_by( key_by )?;
    let on_find = parse_on_find( on_find )?;

    match clear_by_custom_row_key
    (
      client, 
      spreadsheet_id, 
      sheet_name, 
      key_by, 
      on_find
    )
    .await
    {
      Ok( response ) => Ok( response.cleared_ranges.unwrap_or_default() ),
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