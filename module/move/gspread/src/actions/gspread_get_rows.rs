//!
//! Action for command "rows"
//!
//! It returns all rows but not header
//!


mod private
{
  use std::fmt;
  use crate::*;
  use debug::row_wrapper::RowWrapper;
  use client::SheetsType;
  use actions::gspread::Result;
  use format_tools::AsTable;
  use crate::util::display_table::display_rows;

  pub struct Report
  {
    pub rows : Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    fn fmt
    (
      &self,
      f : &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_rows( &AsTable::new( &self.rows ), f )
    }
  }

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< Report >
  {
    let result = hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!A2:Z", table_name).as_str() )
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
