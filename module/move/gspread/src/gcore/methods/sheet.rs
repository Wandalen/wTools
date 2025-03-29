//!
//! spreadsheet methods.
//! 


mod private
{
  use crate::*;
  use ser;
  use gcore::
  {
    Client, 
    Response,
    Secret,
    Error, 
    Result
  };
  use gcore::types::
  {
    DeleteDimensionRequest, 
    SheetProperties
  };

  /// # SpreadSheetMethod
  ///
  /// A helper struct that provides methods for working with spreadsheet sheet in the
  /// Google Sheets API. This struct is associated with a given [`Client`] instance and
  /// offers specialized methods for working with sheets.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   - A reference to a [`Client`] object.  
  ///   - Used to perform authenticated HTTP requests against the Google Sheets API.
  ///
  /// ## Methods
  ///
  /// - **`copy_to`**:
  ///   Copy a source sheet to a destination spreadsheet.
  ///  
  /// ## Usage
  ///
  /// This struct is usually obtained by calling the `sheet()` method on a
  /// fully-initialized [`Client`] instance:
  pub struct SpreadSheetMethod< 'a, S : Secret >
  {
    pub client : &'a Client< 'a, S >,
  }

  impl< S : Secret > SpreadSheetMethod< '_, S >
  {
    /// Build SheetCopyMethod.
    pub fn copy_to< 'a >
    (
      &'a self,
      spreadsheet_id : &'a str,
      sheet_id : &'a str,
      dest : &'a str
    ) -> SheetCopyMethod< 'a, S >
    {
      SheetCopyMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id,
        _sheet_id : sheet_id,
        _dest : dest
      }
    }
  }

  /// # SheetCopyMethod
  ///
  /// Represents a specialized request builder for copying a sheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetMethod::copy_to`].
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_spreadsheet_id`  
  ///   The `String` ID of the spreadsheet from which values are fetched.
  /// - `_sheet_id`
  ///   The source sheet id.
  /// - `_dest`
  ///   The destination spreadsheet id.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to copy a source sheet to destinayion one.
  pub struct SheetCopyMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _spreadsheet_id : &'a str,
    _sheet_id : &'a str,
    _dest : &'a str
  }

  impl< S : Secret > SheetCopyMethod< '_, S >
  {
    /// Sends the POST request to
    /// https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/sheets/{sheetId}:copyTo
    /// 
    /// ## Returns:
    ///  - `Result< [SheetProperties] >`
    /// 
    /// ## Errors:
    ///  - `ApiError`
    ///  - `ParseError`
    pub async fn doit( &self ) -> Result< SheetProperties >
    {
      let endpoint = format!
      ( 
        "{}/{}/sheets/{}:copyTo",
        self.client.endpoint,
        self._spreadsheet_id,
        self._sheet_id
      );

      let request = SheetCopyRequest
      {
        dest : Some( self._dest.to_string() )
      };

      let token = match &self.client.auth
      {
        Some( auth_data ) => 
        {
          let mut token_ref = auth_data.token.borrow_mut();

          if let Some( token ) = &*token_ref 
          {
            token.clone()
          } 
          else 
          {
            let new_token = auth_data
            .secret
            .get_token()
            .await
            .map_err( | err | Error::ApiError( err.to_string() ) )?;

            *token_ref = Some( new_token.clone() );

            new_token
          }
        }
        None => "".to_string()
      };

      let response = reqwest::Client::new()
      .post( endpoint )
      .json( &request )
      .bearer_auth( token )
      .send()
      .await
      .map_err( | err | Error::ApiError( err.to_string() ) )?;

      if !response.status().is_success()
      {
        let response_text = response
        .text()
        .await
        .map_err( | err | Error::ParseError( err.to_string() ) )?;
        
        return Err( Error::ApiError( response_text ) );
      }

      let response_parsed = response.json::< SheetProperties >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( response_parsed )
    }
  }

  #[ derive( Debug, ser::Serialize, ser::Deserialize ) ]
  pub struct BatchUpdateResponse
  {
    #[ serde( rename = "spreadsheetId" ) ]
    spreadsheet_id : Option< String >,
  }

  #[ derive( Debug, ser::Serialize, ser::Deserialize ) ]
  pub struct SheetCopyRequest
  {
    #[ serde( rename = "destinationSpreadsheetId" ) ]
    pub dest : Option< String >
  }
}

crate::mod_interface!
{
  own use
  {
    SpreadSheetMethod
  };
}