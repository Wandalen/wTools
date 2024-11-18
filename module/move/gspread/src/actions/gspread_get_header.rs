//!
//! Action for command "header"
//!
//! It returns header (first row)
//!


mod private
{
  use std::fmt;
  use crate::*;
  use client::SheetsType;
  use actions::gspread::Result;
  use format_tools::AsTable;
  use util::display_table::display_header;

  pub struct Report
  {
    pub rows: Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    fn fmt
    (
      &self,
      f: &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_header( &AsTable::new( &self.rows ), f )
    }
  }

  pub async fn action
  (
    hub: &SheetsType,
    spreadsheet_id: &str,
    table_name: &str) -> Result< Report >
  {
    let result = hub
      .spreadsheets()
      .values_get( spreadsheet_id, format!( "{}!A1:Z1", table_name ).as_str() )
      .doit()
      .await?
      .1
      .values
      .unwrap();


    let rows = result.into_iter().map( RowWrapper ).collect();

    Ok( Report { rows } )
  }
}

crate::mod_interface!
{
  own use action;
}