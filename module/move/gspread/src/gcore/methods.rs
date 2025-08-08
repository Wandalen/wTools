//!
//! Google Sheets API method implementations.
//! 
//! This module contains all the method structs that provide fluent interfaces
//! for interacting with the Google Sheets API. Each method struct encapsulates
//! the parameters and logic needed to make specific API requests.
//!

mod private
{
  use std::cell::RefCell;
  use former::Former;
  use serde_json::json;
  use reqwest::
  { 
    self, 
    Url 
  };
  
  use crate::gcore::auth::Client;
  use crate::gcore::error::
  { 
    Error, Result 
  };
  use crate::gcore::types::
  {
    SheetProperties,
    SheetCopyRequest,
    ValueRange,
    GetValuesRequest,
    Dimension,
    ValueRenderOption,
    DateTimeRenderOption,
    BatchGetValuesResponse,
    UpdateValuesRequest,
    UpdateValuesResponse,
    ValueInputOption,
    BatchUpdateValuesRequest,
    BatchUpdateValuesResponse,
    ValuesAppendRequest,
    ValuesAppendResponse,
    InsertDataOption,
    ValuesClearResponse,
    BatchClearValuesRequest,
    BatchClearValuesResponse,
  };
  use crate::gcore::Secret;

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
    client : &'a Client< 'a, S >,
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

  /// # SpreadSheetValuesMethod
  ///
  /// A helper struct that provides methods for working with spreadsheet values in the
  /// Google Sheets API. This struct is associated with a given [`Client`] instance and
  /// offers specialized methods for retrieving and updating data within a spreadsheet.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   - A reference to a [`Client`] object.  
  ///   - Used to perform authenticated HTTP requests against the Google Sheets API.
  ///
  /// ## Methods
  ///
  /// - **`values_get(
  /// spreadsheet_id, range
  /// )` → [`ValuesGetMethod`]**  
  ///   Creates a new request object that retrieves the values within the specified `range`
  ///   of the spreadsheet identified by `spreadsheet_id`. 
  ///
  /// - **`values_update( value_range, spreadsheet_id, range )` → [`ValuesUpdateMethod`]**  
  ///   Creates a new request object that updates the values within the specified `range`
  ///   of the spreadsheet identified by `spreadsheet_id`, using the provided `value_range`.
  ///
  /// - **`values_batch_update( spreadsheet_id, req )` → [`ValuesBatchUpdateMethod`]**  
  ///   Creates a new request object that performs multiple updates on the spreadsheet
  ///   identified by `spreadsheet_id`, based on the instructions defined in
  ///   `BatchUpdateValuesRequest`.
  /// 
  /// - **`append( spreadsheet_id, range, value_range )` → [`ValuesAppendMethod`]**
  ///   Appends a new row at the end of sheet.
  /// 
  /// - **`values_get_batch(spreadsheet_id)` -> [`ValuesBatchGetMethod`]**
  ///   Returns defined value ranges.
  /// 
  /// - **`clear(spreadsheet_id, range) -> `Result<[ValuesClearResponse]>``**
  ///   Returns metadata of a cleared range.
  /// 
  /// - **`clear_batch(spreadsheet_id, req) -> `Result<[BatchClearValuesResponse]>``**
  ///   Returns metadata of a cleared range.
  ///  
  /// ## Usage
  ///
  /// This struct is usually obtained by calling the `spreadsheet()` method on a
  /// fully-initialized [`Client`] instance:
  pub struct SpreadSheetValuesMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
  }

  impl< S : Secret > SpreadSheetValuesMethod< '_, S >
  {
    /// Creates a new request object that updates the values within the specified `range`
    /// of the spreadsheet identified by `spreadsheet_id`, using the provided `value_range`.
    pub fn values_get
    (
      &self,
      spreadsheet_id : &str,
      range : &str
    ) -> ValuesGetMethod< S >
    {
      ValuesGetMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id.to_string(),
        _range : range.to_string(),
        _major_dimension : Default::default(),
        _value_render_option : Default::default(),
        _date_time_render_option : Default::default()
      }
    }

    /// Returns defined value ranges.
    pub fn values_get_batch< 'a >
    (
      &'a self,
      spreadsheet_id : &'a str,
    ) -> ValuesBatchGetMethod< 'a, S >
    {
      ValuesBatchGetMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id,
        _ranges : Default::default(),
        _major_dimension : Default::default(),
        _value_render_option : Default::default(),
        _date_time_render_option : Default::default(),
      }
    }

    /// Creates a new request object that updates the values within the specified `range`
    /// of the spreadsheet identified by `spreadsheet_id`, using the provided `value_range`. 
    pub fn values_update< 'a >
    ( 
      &'a self,
      value_range : ValueRange,
      spreadsheet_id : &'a str,
      range : &'a str 
    ) -> ValuesUpdateMethod< 'a, S >
    {
      ValuesUpdateMethod
      {
        client : self.client,
        _value_range : value_range,
        _spreadsheet_id : spreadsheet_id,
        _range : range,
        _value_input_option : ValueInputOption::default(),
        _include_values_in_response : Default::default(),
        _response_value_render_option : Default::default(),
        _response_date_time_render_option : Default::default()
      }
    }

    /// Creates a new request object that performs multiple updates on the spreadsheet
    /// identified by `spreadsheet_id`, based on the instructions defined in
    /// `BatchUpdateValuesRequest`.
    pub fn values_batch_update
    ( 
      &self,
      spreadsheet_id : &str,
      req : BatchUpdateValuesRequest,
    ) -> ValuesBatchUpdateMethod< S >
    {
      ValuesBatchUpdateMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id.to_string(),
        _request : req,
      }
    }

    /// Appends a new row at the end of sheet.
    pub fn append< 'a >
    ( 
      &'a self,
      spreadsheet_id : &'a str,
      range : &'a str,
      value_range : ValueRange
    ) -> ValuesAppendMethod< 'a, S >
    {
      ValuesAppendMethod
      {
        client : self.client,
        _value_range : value_range,
        _spreadsheet_id : spreadsheet_id,
        _range : range,
        _value_input_option : ValueInputOption::default(),
        _include_values_in_response : Default::default(),
        _insert_data_option : Default::default(),
        _response_date_time_render_option : Default::default(),
        _response_value_render_option : Default::default()
      }
    }

    /// Clears a specified range.
    pub fn clear< 'a >
    (
      &'a self,
      spreadsheet_id : &'a str,
      range : &'a str
    ) -> ValuesClearMethod< 'a, S >
    {
      ValuesClearMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id,
        _range : range
      }
    }

    /// Clear a specified range.
    pub fn clear_batch< 'a >
    (
      &'a self,
      spreadsheet_id : &'a str,
      req : BatchClearValuesRequest
    ) -> ValuesBatchClearMethod< 'a, S >
    {
      ValuesBatchClearMethod
      {
        client : self.client,
        _spreadsheet_id : spreadsheet_id,
        _request : req
      }
    }
  }

  /// # ValuesGetMethod
  ///
  /// Represents a specialized request builder for retrieving values from a Google Spreadsheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetValuesMethod::values_get`]. It holds references and parameters
  /// required to execute a `GET` request against the Google Sheets API to fetch
  /// spreadsheet data.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_spreadsheet_id`  
  ///   The `String` ID of the spreadsheet from which values are fetched.
  /// - `_range`  
  ///   The `String` representing the cell range (e.g. `"A1:B10"`) to retrieve values for.
  /// - `_major_dimension`  
  ///   An optional [`Dimension`] that specifies whether the range is in rows or columns.
  /// - `_value_render_option`  
  ///   An optional [`ValueRenderOption`] that indicates how values should be
  ///   rendered in the response (e.g., formatted, unformatted or formula).
  /// - `_date_time_render_option`  
  ///   An optional [`DateTimeRenderOption`] specifying how date/time values are
  ///   rendered in the response.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to retrieve the
  ///   specified range of values. Returns a [`ValueRange`] on success, or an
  ///   [`Error`] if the API request fails.
  pub struct ValuesGetMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _spreadsheet_id : String,
    _range : String,
    _major_dimension : Option< Dimension >,
    _value_render_option : Option< ValueRenderOption >,
    _date_time_render_option : Option< DateTimeRenderOption >
  }

  impl< S : Secret > ValuesGetMethod< '_, S >
  {
    /// The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `ranges=["A1:B2"],majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `ranges=["A1:B2"],majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
    ///
    /// Sets the *major dimension* query property to the given value.
    pub fn major_dimension( mut self, new_val : Dimension ) -> Self
    {
      self._major_dimension = Some( new_val );
      self
    }

    /// How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE.
    ///
    /// Sets the *value render option* query property to the given value.
    pub fn value_render_option( mut self, new_val : ValueRenderOption ) -> Self
    {
      self._value_render_option = Some( new_val );
      self
    }

    /// Executes the request configured by `ValuesGetMethod`.
    ///
    /// Performs an HTTP `GET` to retrieve values for the configured spreadsheet range.
    /// On success, returns the [`ValueRange`] containing the fetched data.
    /// If the request fails or the response cannot be parsed, returns an [`Error`].
    pub async fn doit( &self ) -> Result< ValueRange >
    {
      let endpoint = format!
      ( 
        "{}/{}/values/{}", 
        self.client.endpoint, 
        self._spreadsheet_id, 
        self._range 
      );

      let query = GetValuesRequest
      {
        major_dimension : self._major_dimension,
        value_render_option : self._value_render_option,
        date_time_render_option : self._date_time_render_option
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
      .get( endpoint )
      .query( &query )
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

        return Err( Error::ApiError( response_text ) )
      }

      let value_range = response.json::< ValueRange >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( value_range )
    }
  }

  /// A builder for retrieving values from multiple ranges in a spreadsheet using the Google Sheets API.
  /// 
  /// This struct allows you to specify:
  /// 
  /// - **Spreadsheet ID** (the unique identifier of the spreadsheet),
  /// - **Ranges** in [A1 notation](https://developers.google.com/sheets/api/guides/concepts#a1_notation),
  /// 
  /// Then, by calling [`ValuesBatchGetMethod::doit`], you send the `GET` request to retrieve all those ranges in a single batch.  
  /// On success, it returns a [`BatchGetValuesResponse`] with the data. On error, it returns an [`Error`].
  pub struct ValuesBatchGetMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _spreadsheet_id : &'a str,
    _ranges : Vec< String >,
    _major_dimension : Option< Dimension >,
    _value_render_option : Option< ValueRenderOption >,
    _date_time_render_option : Option< DateTimeRenderOption >
  }

  impl< 'a, S : Secret > ValuesBatchGetMethod< 'a, S >
  {
    /// Executes the request configured by `ValuesBatchGetMethod`.
    ///
    /// Performs an HTTP `GET` to retrieve values for the configured spreadsheet range.
    /// On success, returns the [`BatchGetValuesResponse`] containing the fetched data.
    /// If the request fails or the response cannot be parsed, returns an [`Error`].
    pub async fn doit( &self ) -> Result< BatchGetValuesResponse >
    {
      let mut url = format!
      ( 
        "{}/{}/values:batchGet", 
        self.client.endpoint, 
        self._spreadsheet_id 
      );

      let mut parsed_url = Url::parse( &url )
      .map_err( | err | Error::ParseError( err.to_string() ) )?;
      
      {
        let mut pairs = parsed_url.query_pairs_mut();

        for r in &self._ranges
        {
          pairs.append_pair( "ranges", r );
        }
      }

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

      url = parsed_url.into();

      let response = reqwest::Client::new()
      .get( url )
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

        return Err( Error::ApiError( format!( "{}", response_text ) ) )
      }

      let parsed_response = response.json::< BatchGetValuesResponse >()
      .await
      .map_err( | err | Error::ApiError( err.to_string() ) )?;

      Ok( parsed_response )
    }

    /// Set ranges to retrive in A1 notation format.
    pub fn ranges( mut self, new_val : Vec< String >  ) -> ValuesBatchGetMethod< 'a, S >
    {
      self._ranges = new_val;
      self
    }
  }

  /// # ValuesUpdateMethod
  ///
  /// Represents a specialized request builder for updating values in a Google Spreadsheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetValuesMethod::values_update`]. It holds references and parameters
  /// required to execute a `PUT` request against the Google Sheets API to modify
  /// spreadsheet data.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_value_range`  
  ///   A [`ValueRange`] describing the new data to be written to the spreadsheet.
  /// - `_spreadsheet_id`  
  ///   A `&str` denoting the spreadsheet's identifier.
  /// - `_range`  
  ///   A `&str` specifying the cell range (e.g. `"A1:B10"`) where the values should be updated.
  /// - `_value_input_option`  
  ///   A [`ValueInputOption`] that indicates how the input data should be parsed
  ///   (e.g., as user-entered or raw data).
  /// - `_include_values_in_response`  
  ///   An optional `bool` indicating whether the updated values should be
  ///   returned in the response.
  /// - `_response_value_render_option`  
  ///   An optional [`ValueRenderOption`] that specifies how updated values should
  ///   be rendered in the response.
  /// - `_response_date_time_render_option`  
  ///   An optional [`DateTimeRenderOption`] that specifies how date/time values
  ///   should be rendered in the response if `_include_values_in_response` is `true`.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to update the specified
  ///   range with new data. Returns an [`UpdateValuesResponse`] on success, or an
  ///   [`Error`] if the API request fails.
  pub struct ValuesUpdateMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _value_range : ValueRange,
    _spreadsheet_id : &'a str,
    _range : &'a str,
    _value_input_option : ValueInputOption,
    _include_values_in_response : Option< bool >,
    _response_value_render_option : Option< ValueRenderOption >,
    _response_date_time_render_option : Option< DateTimeRenderOption >
  }

  impl< S : Secret > ValuesUpdateMethod< '_, S >
  {
    /// Executes the request configured by `ValuesUpdateMethod`.
    ///
    /// Performs an HTTP `PUT` to update spreadsheet values within the specified range.
    /// On success, returns an [`UpdateValuesResponse`] describing the result of the
    /// update operation. If the request fails or parsing the response is unsuccessful,
    /// an [`Error`] is returned.
    pub async fn doit( &self ) -> Result< UpdateValuesResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values/{}", 
        self.client.endpoint, 
        self._spreadsheet_id, 
        self._range
      );

      let query = UpdateValuesRequest
      {
        value_input_option : self._value_input_option,
        include_values_in_response : self._include_values_in_response,
        response_value_render_option : self._response_value_render_option,
        response_date_time_render_option : self._response_date_time_render_option
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
      .put( endpoint )
      .query( &query )
      .json( &self._value_range )
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

      let parsed_response = response.json::< UpdateValuesResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( parsed_response )
    }
  }

  /// # ValuesBatchUpdateMethod
  ///
  /// Represents a specialized request builder for performing batch updates
  /// of values in a Google Spreadsheet.
  ///
  /// This struct is constructed internally by the library when calling
  /// [`SpreadSheetValuesMethod::values_batch_update`]. It holds the information
  /// required to execute a `POST` request to apply multiple updates in a single
  /// call to the Google Sheets API.
  ///
  /// ## Fields
  ///
  /// - `client`  
  ///   A reference to the [`Client`] used for sending authenticated requests.
  /// - `_spreadsheet_id`  
  ///   The `String` ID of the spreadsheet to be updated.
  /// - `_request`  
  ///   A [`BatchUpdateValuesRequest`] containing multiple update instructions.
  ///
  /// ## Method
  ///
  /// - `doit()`  
  ///   Sends the configured request to the Google Sheets API to perform multiple
  ///   updates on the target spreadsheet. Returns a [`BatchUpdateValuesResponse`]
  ///   on success, or an [`Error`] if the API request fails.
  pub struct ValuesBatchUpdateMethod< 'a, S : Secret >
  {
    pub client : &'a Client< 'a, S >,
    pub _spreadsheet_id : String,
    pub _request : BatchUpdateValuesRequest
  }

  impl< S : Secret > ValuesBatchUpdateMethod< '_, S >
  {
    /// Executes the request configured by `ValuesBatchUpdateMethod`.
    ///
    /// Performs an HTTP `POST` to apply a batch of updates to the specified
    /// spreadsheet. On success, returns a [`BatchUpdateValuesResponse`] containing
    /// details about the applied updates. If the request fails or the response
    /// cannot be parsed, an [`Error`] is returned.
    pub async fn doit( &self ) -> Result< BatchUpdateValuesResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values:batchUpdate",
        self.client.endpoint,
        self._spreadsheet_id
      );

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
      .json( &self._request )
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

      let parsed_response = response.json::< BatchUpdateValuesResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( parsed_response )
    }
  }

  /// A builder for appending values to a sheet.
  ///
  /// This struct lets you configure:
  /// - The spreadsheet ID (`_spreadsheet_id`),
  /// - The input data (`_value_range`),
  ///
  /// By calling [`ValuesAppendMethod::doit`], you perform an HTTP `POST` request
  /// to `https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:append`.
  /// 
  /// On success, it returns a [`ValuesAppendResponse`] containing metadata about the append result.
  /// On error, returns an [`Error`].
  pub struct ValuesAppendMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _value_range : ValueRange,
    _spreadsheet_id : &'a str,
    _range : &'a str,
    _value_input_option : ValueInputOption,
    _insert_data_option : Option< InsertDataOption >,
    _include_values_in_response : bool,
    _response_value_render_option : Option< ValueRenderOption >,
    _response_date_time_render_option : Option< DateTimeRenderOption >
  }

  impl< S : Secret > ValuesAppendMethod< '_, S >
  {
    /// Executes the configured append request.
    ///
    /// Sends a `POST` request to:
    /// `https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{range}:append?valueInputOption=...&...`
    ///
    /// - Query parameters are built from `ValuesAppendRequest` (e.g. `valueInputOption`, `insertDataOption`, etc.).
    /// - The JSON body contains a [`ValueRange`] with the actual data to append.
    ///
    /// Returns [`ValuesAppendResponse`] on success, or an [`Error`] if the request fails 
    /// or if response parsing fails.
    ///
    /// # Errors
    /// - [`Error::ApiError`] if the HTTP status is not successful or the API returns an error.
    /// - [`Error::ParseError`] if the body cannot be deserialized into [`ValuesAppendResponse`].
    pub async fn doit( &self ) -> Result< ValuesAppendResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values/{}:append", 
        self.client.endpoint, 
        self._spreadsheet_id, 
        self._range
      );

      let query = ValuesAppendRequest
      {
        value_input_option : self._value_input_option,
        insert_data_option : self._insert_data_option,
        include_values_in_response : self._include_values_in_response,
        response_value_render_option : self._response_value_render_option,
        response_date_time_render_option : self._response_date_time_render_option
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
      .query( &query )
      .json( &self._value_range )
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

      let parsed_response = response.json::< ValuesAppendResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( parsed_response )
    }

    /// #insert_data_option
    /// 
    /// Set up new insertDataOption to request.
    pub fn insert_data_option( mut self, new_val : InsertDataOption ) -> Self 
    {
      self._insert_data_option = Some( new_val );
      self
    }
  }

  /// A builder for clearing values from a sheet.
  ///
  /// This struct lets you configure:
  ///
  /// By calling [`ValuesClearMethod::doit`], you perform an HTTP `POST` request
  /// to `https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:clear`.
  /// 
  /// On success, it returns a [`ValuesClearResponse`] containing metadata about the clear result.
  /// On error, returns an [`Error`].
  pub struct ValuesClearMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _spreadsheet_id : &'a str,
    _range : &'a str
  }

  impl< S : Secret > ValuesClearMethod< '_, S >
  {
    /// Executes the configured clear request.
    ///
    /// Sends a `POST` request to:
    /// `https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:clear`
    ///
    /// Returns [`ValuesClearResponse`] on success, or an [`Error`] if the request fails 
    /// or if response parsing fails.
    ///
    /// # Errors
    /// - [`Error::ApiError`] if the HTTP status is not successful or the API returns an error.
    /// - [`Error::ParseError`] if the body cannot be deserialized into [`ValuesClearResponse`].
    pub async fn doit( &self ) -> Result< ValuesClearResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values/{}:clear", 
        self.client.endpoint, 
        self._spreadsheet_id, 
        self._range
      );

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
      .json( &json!( {} ) )
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

        return Err( Error::ApiError( response_text ) )
      }

      let response_parsed = response.json::< ValuesClearResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( response_parsed )
    } 
  }

  /// A builder for clearing values from a sheet.
  ///
  /// This struct lets you configure:
  ///
  /// By calling [`ValuesBatchClearMethod::doit`], you perform an HTTP `POST` request
  /// to `https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchClear`.
  /// 
  /// On success, it returns a [`BatchClearValuesResponse`] containing metadata about the clear result.
  /// On error, returns an [`Error`].
  pub struct ValuesBatchClearMethod< 'a, S : Secret >
  {
    client : &'a Client< 'a, S >,
    _spreadsheet_id : &'a str,
    _request : BatchClearValuesRequest
  }

  impl< S : Secret > ValuesBatchClearMethod< '_, S >
  {
    /// Executes the configured clear request.
    ///
    /// Sends a `POST` request to:
    /// `https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchClear`
    ///
    /// Returns [`BatchClearValuesResponse`] on success, or an [`Error`] if the request fails 
    /// or if response parsing fails.
    ///
    /// # Errors
    /// - [`Error::ApiError`] if the HTTP status is not successful or the API returns an error.
    /// - [`Error::ParseError`] if the body cannot be deserialized into [`BatchClearValuesResponse`].
    pub async fn doit( &self ) -> Result< BatchClearValuesResponse >
    {
      let endpoint = format!
      (
        "{}/{}/values:batchClear",
        self.client.endpoint,
        self._spreadsheet_id
      );

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
      .json( &self._request )
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

      let response_parsed = response.json::< BatchClearValuesResponse >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( response_parsed )
    }
  }
  // Client implementation methods to avoid circular imports with auth.rs
  impl< S : crate::gcore::Secret > Client< '_, S >
  {
    pub fn spreadsheet( &self ) -> SpreadSheetValuesMethod<S>
    {
      SpreadSheetValuesMethod
      {
        client : self
      }
    }

    pub fn sheet( &self ) -> SpreadSheetMethod<S>
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
  exposed use private::SpreadSheetMethod;
  exposed use private::SheetCopyMethod;
  exposed use private::SpreadSheetValuesMethod;
  exposed use private::ValuesGetMethod;
  exposed use private::ValuesBatchGetMethod;
  exposed use private::ValuesUpdateMethod;
  exposed use private::ValuesBatchUpdateMethod;
  exposed use private::ValuesAppendMethod;
  exposed use private::ValuesClearMethod;
  exposed use private::ValuesBatchClearMethod;
}