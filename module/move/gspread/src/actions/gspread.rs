//!
//! Google Sheets API actions.
//!
//! This module also contains the definition of Google Sheets Error.
//!

mod private
{
  use regex::Regex;
  use error_tools::typed::Error;
  use derive_tools::AsRefStr;
  use crate::*;
  use ser::
  {
    DisplayFromStr, 
    JsonValue
  };
  use std::collections::HashMap;
  use google_sheets4::api::
  {
    BatchUpdateValuesResponse, 
    BatchUpdateValuesRequest, 
    ValueRange
  };

  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]

  /// Represents errors that can occur while interacting with the Google Sheets API 
  /// or during related operations in the application.
  pub enum Error
  {
    /// Represents an error returned by the Google Sheets API.
    /// 
    /// # Details
    /// This error occurs when the API returns a specific error message.
    /// The error message from the Google Sheets API is stored and displayed.
    /// 
    /// # Fields
    /// - `google_sheets4::Error`: The raw error returned by the API.
    #[ error( "Google Sheets returned error:\n{0}" ) ]
    ApiError
    (
      #[ from ]
      #[ serde_as( as = "DisplayFromStr" ) ]
      google_sheets4::Error
    ),

    /// Represents an error that occurs while initializing Google Sheets Hub.
    /// 
    /// # Details
    /// This error indicates that the application failed to properly configure with the Google Sheets Hub.
    /// 
    /// # Fields
    /// - `String`: A detailed error message describing the issue.
    #[ error( "Hub Error:\n{0}" ) ]
    HubError
    (
      String
    ),

    /// Represents an error caused by an invalid URL format.
    /// 
    /// # Details
    /// This error occurs when the provided URL does not match the expected format
    /// 
    /// # Fields
    /// - `String`: The invalid URL or a message describing the issue.
    #[ error( "Invalid URL format:\n{0}" ) ]
    InvalidUrl
    (
      String
    ),

    /// Represents an error related to a cell in the spreadsheet.
    /// 
    /// # Details
    /// This error indicates that a cell was not got or updated
    /// 
    /// # Fields
    /// - `String`: A message describing the issue with the cell.
    #[ error( "Cell error:\n{0}" ) ]
    CellError
    (
      String
    ),

    /// Represents an error caused by invalid JSON input or parsing issues.
    /// 
    /// # Details
    /// This error occurs when the provided JSON data does not conform to the expected
    /// structure or format.
    /// 
    /// # Fields
    /// - `String`: A detailed error message describing the JSON issue.
    #[ error( "Invalid JSON format:\n{0}" ) ]
    InvalidJSON
    (
      String
    ),

    /// Represents a generic parsing error.
    /// 
    /// # Details
    /// This error is raised when a string or other input cannot be parsed
    /// into the expected format or structure.
    /// 
    /// # Fields
    /// - `String`: A message describing the parse error.
    #[ error( "Parse error:\n{0}" ) ]
    ParseError
    (
      String
    )
  }

  /// Retrive spreadsheet id from url
  pub fn get_spreadsheet_id_from_url
  (
    url : &str
  ) -> Result< &str >
  {

    let re = Regex::new( r"d/([^/]+)/edit" ).unwrap();
    if let Some( captures ) = re.captures( url )
    {
      if let Some( id ) = captures.get( 1 )
      {
        return Ok( id.as_str() );
      }
    }

    Err
    ( 
      Error::InvalidUrl( "Wrong url format.\nFix: copy sheet's the whole url from your browser. Usage: --url '<your copied url>'".to_string() ) 
    )
  }

  /// Function to update a row on a Google Sheet.
  /// 
  /// It sends HTTP request to Google Sheets API and change row wich provided values. 
  /// 
  /// **Params**
  ///  - `spreadsheet_id` : Spreadsheet identifire.
  ///  - `sheet_name` : Sheet name.
  ///  - `row_key` : row's key.
  ///  - `row_key_val` : pairs of key value, where key is a column name and value is a new value.
  /// 
  /// **Returns**
  ///  - `Result`
  pub async fn update_row
  (
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : &str,
    row_key_val : HashMap< String, String >
  ) -> Result< BatchUpdateValuesResponse >
  {
    let secret = Secret::read();
    let hub = hub(&secret)
    .await
    .map_err( | _ | {
      Error::HubError( format!( "Failed to create a hub. Ensure that you have a .env file with Secrets" ) )
    })?;

    let mut value_ranges = Vec::with_capacity( row_key_val.len() );

    for ( col_name, value ) in row_key_val {
      value_ranges.push
      (
        ValueRange 
        { 
          major_dimension: Some( String::from( "ROWS" ) ),
          values: Some( vec![ vec![ JsonValue::String( value ) ] ] ),
          range: Some( format!( "{}!{}{}", sheet_name, col_name, row_key ) ), 
        }
      )
    }

    let req = BatchUpdateValuesRequest
    {
      value_input_option: Some( "USER_ENTERED".to_string() ),
      data: Some( value_ranges ),
      include_values_in_response: Some( true ),
      ..Default::default()
    };

    match hub
    .spreadsheets()
    .values_batch_update( req, spreadsheet_id )
    .doit()
    .await
    {
      Ok( ( _, response ) ) => Ok( response ),
      Err( error ) => Err( Error::ApiError( error ) ),
    }
  }

  pub type Result< T > = core::result::Result< T, Error >;
}

crate::mod_interface!
{
  own use
  {
    Error,
    Result,
    update_row,
    get_spreadsheet_id_from_url,
  };
}