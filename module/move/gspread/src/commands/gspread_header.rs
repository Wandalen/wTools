//!
//! Command "header"
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
  use util::display_table::display_header;

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

  pub async fn command
  (
    hub : &SheetsType,
    args : CommonArgs,
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

        let result = actions::gspread_get_header::action
          (
            hub,
            spreadsheet_id,
            tab.as_str()
          ).await;

        match result
        {
          Ok( header ) =>
            {
              let header_wrapped = header
              .into_iter()
              .map( | row | RowWrapper{ max_len: row.len(), row } )
              .collect();

              println!( "Header: \n {}", Report{ rows: header_wrapped } );
            }
          Err( error ) => println!( "Error: {}", error ),
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

