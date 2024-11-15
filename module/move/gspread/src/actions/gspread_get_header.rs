//!
//! Action for command "header"
//!
//! It returns header (first row)
//!


mod private
{

  use crate::*;
  use client::SheetsType;
  use actions::gspread::{ Value, Result };

  pub async fn action
  (
    hub: &SheetsType,
    spreadsheet_id: &str,
    table_name: &str) -> Result< Value >
  {
    let result = hub
      .spreadsheets()
      .values_get( spreadsheet_id, format!( "{}!A1:Z1", table_name ).as_str() )
      .doit()
      .await?;

    Ok( result.1.values.unwrap() )
  }
}

pub use private::
{
  action,
};
