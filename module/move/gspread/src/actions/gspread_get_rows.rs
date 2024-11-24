//!
//! Action for command "rows"
//!
//! It returns all rows but not header
//!


mod private
{
  use crate::*;
  use client::SheetsType;
  use actions::gspread::Result;
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< Vec< Vec < JsonValue > > >
  {
    let result = hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!A2:Z", table_name ).as_str() )
    .doit()
    .await?
    .1
    .values
    .unwrap_or_else( | | Vec::new() );

    Ok( result )
  }
}

crate::mod_interface!
{
  own use action;
}
