//!
//! Command "rows"
//!

mod private
{
  use std::fmt;
  use crate::*;
  use commands::gspread::CommonArgs;
  use client::SheetsType;
  use actions;
  use actions::gspread::get_spreadsheet_id_from_url;
  use format_tools::AsTable;
  use util::display_table::display_rows;

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

  pub async fn command
  (
    hub : &SheetsType,
    args : CommonArgs
  )
  {
    match args
    {
      CommonArgs { url, tab } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( url.as_str() ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_get_rows::action
        (
          hub,
          spreadsheet_id,
          tab.as_str()
        )
        .await
        {
          Ok( rows ) =>
          {
            let max_len = rows.iter().map(|row| row.len()).max().unwrap_or(0);
            let rows_wrapped: Vec<RowWrapper> = rows
            .into_iter()
            .map(|row| RowWrapper { row, max_len })
            .collect();

            println!( "Rows:\n{}", Report{ rows: rows_wrapped } );
          }
          Err( error ) => eprintln!( "Error:\n{}", error ),
        }
      }
    }
  }
}

crate::mod_interface!
{
  own use
  {
    command
  };
}
