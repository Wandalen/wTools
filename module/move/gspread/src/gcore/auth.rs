//!
//! Authentication and client core functionality for Google Sheets API.
//!

mod private
{
  use std::cell::RefCell;
  use former::Former;
  use crate::*;
  use gcore::Secret;
  use crate::utils::constants::GOOGLE_API_URL;

  /// # Auth
  /// 
  /// Structure to keep oauth2 token.
  /// 
  /// ## Fields:
  /// - `secret`:
  ///   A structure which implemets [`Secret`] trait.
  /// - `token`:
  ///   Oauth2 token in string representation.
  pub struct Auth< 'a, S : Secret + 'a >
  {
    pub secret : &'a S,
    token : RefCell< Option< String > >
  }

  impl< 'a, S : Secret > Auth< 'a, S >
  {
    /// Just constructor.
    pub fn new( secret : &'a S ) -> Self
    {
      Self
      {
        secret : secret,
        token : RefCell::new( None )
      }
    }
  }
  
  /// # Gspread Client
  ///
  /// A struct that represents a client for interacting with Google Spreadsheets.
  ///
  /// This structure encapsulates the essential information and methods needed to
  /// authenticate and send requests to the Google Sheets API. It uses the [`Former`]
  /// procedural macro to provide builder-like functionality, allowing you to
  /// configure fields (like `token` and `endpoint`) before finalizing an instance.
  ///
  /// ## Fields
  ///
  /// - `token`  
  ///   - A `String` representing the OAuth2 access token needed to perform requests
  ///     against the Google Sheets API.  
  ///   - Typically set using the `token(&Secret)` method (see below).
  ///
  /// - `endpoint`  
  ///   - A `String` specifying the base API endpoint for Google Sheets.  
  ///   - Defaults to `"https://sheets.googleapis.com/v4/spreadsheets"` if no value
  ///     is provided.
  /// 
  /// ## Methods
  /// 
  /// - **`spreadsheet` → [`SpreadSheetValuesMethod`]**
  ///   Returns  [`SpreadSheetValuesMethod`].
  ///
  /// ## Usage
  ///
  /// An instance of `Client` can be created via its `Former` implementation. You have to
  /// set the `token` dynamically by providing a [`Secret`] to the `token( &Secret )`
  /// method, which handles OAuth2 authentication under the hood.
  /// You can use this client also for mock testing. In such case you need to provide `endpoint`
  /// using `endpoint( url )` and there is no need to set `token`.
  /// 
  /// Once the `Client` is fully constructed, you can use the `spreadsheet()` method
  /// to access various Google Sheets API operations, such as reading or updating
  /// spreadsheet cells.
  #[ derive( Former ) ]
  pub struct Client< 'a, S : Secret + 'a >
  {
    auth : Option< Auth< 'a, S > >,
    #[ former( default = GOOGLE_API_URL ) ]
    endpoint : &'a str,
  }

  // Implementation methods moved to methods.rs to avoid circular imports
}

crate::mod_interface!
{
  own use
  {
    Auth,
    Client,
  };
}