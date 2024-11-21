//!
//! Action for command "cell get"
//!
//! It returns a selected cell
//!

mod private
{
  use crate::*;
  use actions::gspread::Result;
  use client::SheetsType;
  use crate::ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str,
    cell_id : &str,
  ) -> Result< Vec< Vec< JsonValue > > >
  {
    let result = hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!{}", table_name, cell_id ).as_str() )
    .doit()
    .await?;

    Ok( result.1.values.unwrap() )
  }
}

crate::mod_interface!
{
  own use action;
}