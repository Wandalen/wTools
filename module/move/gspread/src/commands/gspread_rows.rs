//!
//! Command "rows"
//!

mod private
{
  use std::fmt;
  use crate::*;
  use actions;
  use gcore::client::Client;
  use format_tools::AsTable;
  use commands::gspread::CommonArgs;
  use utils::display_table::display_rows;
  use actions::gspread::get_spreadsheet_id_from_url;

  /// # Report
  ///
  /// A structure to display retrieved rows in the console using `format_tools`.
  ///
  /// ## Fields:
  /// - `rows`:  
  ///   A `Vec<RowWrapper>` containing the rows to be displayed.
  ///
  /// ## Usage:
  /// This structure is used in conjunction with the `fmt::Display` trait to render rows in a formatted table view.
  pub struct Report
  {
    pub rows : Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    /// Formats the rows for display by calling the `display_rows` function,
    /// which uses appropriate functions from `format_tools`.
    ///
    /// ## Parameters:
    /// - `f`:  
    ///   A mutable reference to the `fmt::Formatter` used to write the formatted output.
    ///
    /// ## Returns:
    /// - `fmt::Result`:  
    fn fmt
    (
      &self,
      f : &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_rows( &AsTable::new( &self.rows ), f )
    }
  }

  /// # `command`
  ///
  /// Processes the `rows` command by retrieving rows from a specified Google Sheet
  /// and displaying them in a table format in the console.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A `GspreadClient` enum.
  ///   - `Variants`: 
  ///     `SheetsType` variant is used for interacting with the Google Sheets API. 
  ///     `MockClient` variant is used for mock testing.
  /// - `args`:  
  ///   A `CommonArgs` instance containing the sheet's URL and tab name.
  ///
  /// ## Errors:
  /// - Prints an error message if the spreadsheet ID extraction or row retrieval fails.
  pub async fn command
  (
    client : &Client<'_>,
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

        match actions::gspread_rows_get::action
        (
          client,
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
