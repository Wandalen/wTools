//!
//! Client to interact with Google Sheets API.
//! 

mod private
{
  use std::cell::RefCell;
  use former::Former;
 
  use crate::{gcore::methods::{sheet::SpreadSheetMethod, values::SpreadSheetValuesMethod}, *};
  use gcore::Secret;


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
    pub token : RefCell< Option< String > >
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
    pub auth : Option< Auth< 'a, S > >,
    #[ former( default = GOOGLE_API_URL ) ]
    pub endpoint : &'a str,
  }

  impl< S : Secret > Client< '_, S >
  {
    pub fn spreadsheet( &self ) -> SpreadSheetValuesMethod< S >
    {
      SpreadSheetValuesMethod
      {
        client : self
      }
    }

    pub fn sheet( &self ) -> SpreadSheetMethod< S >
    {
      SpreadSheetMethod
      {
        client : self
      }
    }
  }

}


crate::mod_interface!
{
  orphan use
  {
    Auth,
    Client,
  };
}