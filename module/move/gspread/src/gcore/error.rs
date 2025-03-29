//!
//! Gspread errors.
//! 


mod private
{
  use derive_tools::AsRefStr;
  use error_tools::typed::Error;
  use crate::*;
  use ser;

  /// # Error
  ///
  /// Represents errors that can occur while interacting with the Google Sheets API 
  /// or during related operations in the application.
  ///
  /// ## Variants:
  ///
  /// ### `ApiError`
  ///
  /// Represents an error returned by the Google Sheets API.
  ///
  /// **Details:**  
  /// This error occurs when the API returns a specific error message.  
  /// The error message from the Google Sheets API is stored and displayed.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   The raw error returned by the API.
  ///
  /// ### `InvalidUrl`
  ///
  /// Represents an error caused by an invalid URL format.
  ///
  /// **Details:**  
  /// This error occurs when the provided URL does not match the expected format.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   The invalid URL or a message describing the issue.
  ///
  /// ### `CellError`
  ///
  /// Represents an error related to a cell in the spreadsheet.
  ///
  /// **Details:**  
  /// This error indicates that a cell was not retrieved or updated successfully.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A message describing the issue with the cell.
  ///
  /// ### `InvalidJSON`
  ///
  /// Represents an error caused by invalid JSON input or parsing issues.
  ///
  /// **Details:**  
  /// This error occurs when the provided JSON data does not conform to the expected structure or format.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A detailed error message describing the JSON issue.
  ///
  /// ### `ParseError`
  ///
  /// Represents a generic parsing error.
  ///
  /// **Details:**  
  /// This error is raised when a string or other input cannot be parsed into the expected format or structure.
  ///
  /// **Fields:**  
  /// - `String`:  
  ///   A message describing the parse error.
  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    /// Represents an error returned by the Google Sheets API.
    /// 
    /// # Details
    /// This error occurs when the API returns a specific error message.
    /// The error message from the Google Sheets API is stored and displayed.
    /// 
    /// # Fields
    /// - `String`: The raw error returned by the API.
    #[ error( "Google Sheets returned error:\n{0}" ) ]
    ApiError( String ),

    /// Represents an error returned by yup_oauth2.
    /// 
    /// # Details
    /// This error can error while token initialization.
    /// 
    /// # Fields
    /// - `String`: The raw error returned by token().
    #[ error( "Authentication error:\n{0}" ) ]
    AuthError( String ),

    /// Represents an error caused by an invalid URL format.
    /// 
    /// # Details
    /// This error occurs when the provided URL does not match the expected format
    /// 
    /// # Fields
    /// - `String`: The invalid URL or a message describing the issue.
    #[ error( "Invalid URL format:\n{0}" ) ]
    InvalidUrl( String ),

    /// Represents an error related to a cell in the spreadsheet.
    /// 
    /// # Details
    /// This error indicates that a cell was not got or updated
    /// 
    /// # Fields
    /// - `String`: A message describing the issue with the cell.
    #[ error( "Cell error:\n{0}" ) ]
    CellError( String ),

    /// Represents an error caused by invalid JSON input or parsing issues.
    /// 
    /// # Details
    /// This error occurs when the provided JSON data does not conform to the expected
    /// structure or format.
    /// 
    /// # Fields
    /// - `String`: A detailed error message describing the JSON issue.
    #[ error( "Invalid JSON format:\n{0}" ) ]
    InvalidJSON( String ),

    /// Represents a generic parsing error.
    /// 
    /// # Details
    /// This error is raised when a string or other input cannot be parsed
    /// into the expected format or structure.
    /// 
    /// # Fields
    /// - `String`: A message describing the parse error.
    #[ error( "Parse error:\n{0}" ) ]
    ParseError( String )
  }

  /// Type alias for `std::result::Result< T, Error >`.
  pub type Result< T > = std::result::Result< T, Error >;
}

crate::mod_interface!
{
  own use
  {
    Error,
    Result
  };
}