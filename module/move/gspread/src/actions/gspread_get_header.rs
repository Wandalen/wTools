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
  use ser::JsonValue;

  #[ derive( Debug ) ]
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
      display_header( &AsTable::new( &self.rows ), f )
    }
  }

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name: &str) -> Result< Vec< Vec< JsonValue > > >
  {
    let result = hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!A1:Z1", table_name ).as_str() )
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