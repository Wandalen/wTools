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
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str,
    cell_id : &str,
  ) -> Result< JsonValue >
  {
    let result = hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!{}", table_name, cell_id ).as_str() )
    .doit()
    .await?
    .1
    .values;

    match result
    {
      Some( values ) => Ok( values.get( 0 ).unwrap().get( 0 ).unwrap().clone() ),
      None => Ok( JsonValue::Null.clone() )
    }

  }
}

crate::mod_interface!
{
  own use action;
}