//!
//! Command "header"
//!

mod private
{
  use std::fmt;
  use crate::*;
  use debug::RowWrapper;
  use gcore::Secret;
  use gcore::client::Client;
  use commands::gspread::CommonArgs;
  use actions;
  use actions::utils::get_spreadsheet_id_from_url;
  use format_tools::AsTable;
  use utils::display_table::display_header;

  /// # Report
  ///
  /// A structure to display the retrieved header in the console using `format_tools`.
  ///
  /// ## Fields:
  /// - `header`:  
  ///   A `Vec< RowWrapper >` representing the retrieved header rows.
  ///
  /// ## Usage:
  /// This structure is used in conjunction with the `fmt::Display` trait to render the header in a formatted table view.
  #[ derive( Debug ) ]
  pub struct Report
  {
    pub header : Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    /// Formats the header for display by calling the `display_header` function,
    /// which uses appropriate functions from `format_tools`.
    ///
    /// ## Parameters:
    /// - `f`:  
    ///   A mutable reference to the `fmt::Formatter` used to write the formatted output.
    ///
    /// ## Returns:
    /// - `fmt::Result` 
    fn fmt
    (
      &self,
      f : &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_header( &AsTable::new( &self.header ), f )
    }
  }

  /// # `command`
  ///
  /// Processes the `header` command by retrieving the header (first row) from a specified Google Sheet
  /// and displaying it in a table format in the console.
  /// 
  /// ## Errors:
  /// - Prints an error message if the spreadsheet ID extraction or header retrieval fails.
  pub async fn command< S : Secret >
  (
    client : &Client< '_, S >,
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

        match actions::gspread_header_get::action
        (
          client,
          spreadsheet_id,
          tab.as_str()
        )
        .await
        {
          Ok( header ) =>
            {
              let header_wrapped = RowWrapper
              { 
                max_len : header.len(),
                row : header
              };
              println!( "Header:\n{}", Report{ header : vec![ header_wrapped ] } );
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

