//!
//! Client to interact with Google Sheets API.
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
 
  use crate::*;
  use gcore::Secret;
  use gcore::error::
  { 
    Error, Result 
  };
  use ser::
  { 
    self, 
    Serialize, 
    Deserialize 
  };

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

  impl< S : Secret > Client< '_, S >
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
  ///   A `&str` denoting the spreadsheet’s identifier.
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

  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct SheetCopyRequest
  {
    #[ serde( rename = "destinationSpreadsheetId" ) ]
    pub dest : Option< String >
  }

  /// The kind of sheet.
  #[ derive( Debug, Serialize, Deserialize) ]
  pub enum SheetType
  {
    /// The sheet is a grid. 
    #[ serde( rename = "GRID" ) ]
    Grid,

    /// The sheet has no grid and instead has an object like a chart or image. 
    #[ serde( rename = "OBJECT" ) ]
    Object,

    /// The sheet connects with an external DataSource and shows the preview of data.
    #[ serde( rename = "DATA_SOURCE" ) ]
    DataSource
  }
  
  /// Properties of a grid.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct GridProperties
  {
    /// The number of rows in the grid. 
    #[ serde( rename = "rowCount" ) ]
    row_count : Option< u64 >,

    /// The number of columns in the grid. 
    #[ serde( rename = "columnCount" ) ]
    column_count : Option< u32 >,

    /// The number of rows that are frozen in the grid. 
    #[ serde( rename = "frozenRowCount" ) ]
    frozen_row_count : Option< u64 >,

    /// The number of columns that are frozen in the grid. 
    #[ serde( rename = "frozenColumnCount" ) ]
    frozen_column_count : Option< u64 >,

    /// True if the grid isn't showing gridlines in the UI. 
    #[ serde( rename = "hideGridlines" ) ]
    hide_grid_lines : Option< bool >,

    /// True if the row grouping control toggle is shown after the group. 
    #[ serde( rename = "rowGroupControlAfter" ) ]
    row_group_control_after : Option< bool >,

    /// True if the column grouping control toggle is shown after the group. 
    #[ serde( rename = "columnGroupControlAfter" ) ]
    column_group_control_after : Option< bool >
  }

  /// Represents a color in the RGBA color space. 
  /// More information here [color google docs](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#Color)
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct Color
  {
    /// The amount of red in the color as a value in the interval [0, 1]. 
    pub red : Option< f32 >,

    /// The amount of green in the color as a value in the interval [0, 1]. 
    pub green : Option< f32 >,

    /// The amount of blue in the color as a value in the interval [0, 1]. 
    pub blue : Option< f32 >,

    /// The fraction of this color that should be applied to the pixel.
    pub alpha : Option< f32 >
  }

  /// Theme color types.  
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum ThemeColorType
  {
    /// Represents the primary text color 
    #[ serde( rename = "TEXT" ) ]
    Text,

    /// Represents the primary background color 
    #[ serde( rename = "BACKGROUND" ) ]
    Background,

    /// Represents the first accent color 
    #[ serde( rename = "ACCENT1" ) ]
    Accent1,

    /// Represents the second accent color 
    #[ serde( rename = "ACCENT2" ) ]
    Accent2,

    #[ serde( rename = "ACCENT3" ) ]
    /// Represents the third accent color 
    Accent3,

    #[ serde( rename = "ACCENT4" ) ]
    /// Represents the fourth accent color 
    Accent4,

    #[ serde( rename = "ACCENT5" ) ]
    /// Represents the fifth accent color
    Accent5,

    #[ serde( rename = "ACCENT6" ) ]
    /// Represents the sixth accent color
    Accent6,

    /// Represents the color to use for hyperlinks
    #[ serde( rename = "LINK" ) ]
    Link
  }

  /// A color value.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum ColorStyle
  {
    #[ serde( rename = "rgbColor" ) ]
    RgbColor( Color ),

    #[ serde( rename = "themeColor" ) ]
    ThemeColor( ThemeColorType )
  }

  /// An unique identifier that references a data source column.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataSourceColumnReference
  {
    /// The display name of the column. It should be unique within a data source. 
    pub name : Option< String >
  }

  /// A column in a data source.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataSourceColumn
  {
    /// The column reference. 
    pub reference : Option< DataSourceColumnReference >,

    /// The formula of the calculated column. 
    pub formula : Option< String >
  }

  /// An enumeration of data execution states. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum DataExecutionState
  {
    /// The data execution has not started. 
    #[ serde( rename = "NOT_STARTED" ) ]
    NotStarted,
    
    /// The data execution has started and is running.
    #[ serde( rename = "RUNNING" ) ]
    Running,

    /// The data execution is currently being cancelled.
    #[ serde( rename = "CANCELLING" ) ]
    Cancelling,

    /// The data execution has completed successfully. 
    #[ serde( rename = "SUCCEEDED" ) ]
    Succeeded,

    /// The data execution has completed with errors.
    #[ serde( rename = "FAILED" ) ]
    Failed
  }

  /// An enumeration of data execution error code.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum DataExecutionErrorCode
  {
    /// The data execution timed out. 
    #[ serde( rename = "TIMED_OUT" ) ]
    TimedOut,

    /// The data execution returns more rows than the limit.
    #[ serde( rename = "TOO_MANY_ROWS" ) ]
    TooManyRows,

    /// The data execution returns more columns than the limit.
    #[ serde( rename = "TOO_MANY_COLUMNS" ) ]
    TooManyColumns,

    /// The data execution returns more cells than the limit.
    #[ serde( rename = "TOO_MANY_CELLS" ) ]
    TooManyCells,

    /// Error is received from the backend data execution engine (e.g. BigQuery)
    #[ serde( rename = "ENGINE" ) ]
    Engine,

    /// One or some of the provided data source parameters are invalid. 
    #[ serde( rename = "PARAMETER_INVALID" ) ]
    ParameterInvalid,

    /// The data execution returns an unsupported data type. 
    #[ serde( rename = "UNSUPPORTED_DATA_TYPE" ) ]
    UnsupportedDataType,

    /// The data execution returns duplicate column names or aliases.
    #[ serde( rename = "DUPLICATE_COLUMN_NAMES" ) ]
    DuplicateColumnNames,

    /// The data execution is interrupted. Please refresh later.
    #[ serde( rename = "INTERRUPTED" ) ]
    Interrupted,

    /// The data execution is currently in progress, can not be refreshed until it completes. 
    #[ serde( rename = "CONCURRENT_QUERY" ) ]
    ConcurrentQuery,

    /// Other errors. 
    #[ serde( rename = "OTHER" ) ]
    Other,

    /// The data execution returns values that exceed the maximum characters allowed in a single cell.
    #[ serde( rename = "TOO_MANY_CHARS_PER_CELL" ) ]
    TooManyCharsPerCell,

    /// The database referenced by the data source is not found.
    #[ serde( rename = "DATA_NOT_FOUND" ) ]
    DataNotFound,

    /// The user does not have access to the database referenced by the data source. 
    #[ serde( rename = "PERMISSION_DENIED" ) ]
    PermissionDenied,

    /// The data execution returns columns with missing aliases. 
    #[ serde( rename = "MISSING_COLUMN_ALIAS" ) ]
    MissingColumnAlias,

    /// The data source object does not exist. 
    #[ serde( rename = "OBJECT_NOT_FOUND" ) ]
    ObjectNotFound,

    /// The data source object is currently in error state.
    #[ serde( rename = "OBJECT_IN_ERROR_STATE" ) ]
    ObjectInErrorState,

    /// The data source object specification is invalid. 
    #[ serde( rename = "OBJECT_SPEC_INVALID" ) ]
    ObjectSprecInvalid,

    /// The data execution has been cancelled. 
    #[ serde( rename = "DATA_EXECUTION_CANCELLED" ) ]
    DataExecutionCancelled
  }

  /// The data execution status.
  /// More information [here](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#DataExecutionStatus)
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataExecutinStatus
  {
    /// The state of the data execution.
    pub state : Option< DataExecutionState >,

    /// The error code
    #[ serde( rename = "errorCode" ) ]
    pub error_code : Option< DataExecutionErrorCode >,

    /// The error message, which may be empty. 
    #[ serde( rename = "errorMessage" ) ]
    pub error_message : Option< String >,

    /// lastRefreshTime
    #[ serde( rename = "lastRefreshTime" ) ]
    pub last_refresh_time : Option< String >
  }

  /// Additional properties of a [DATA_SOURCE](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/sheets#SheetType) sheet. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataSourceSheetProperties
  {
    /// ID of the [DataSource](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets#DataSource) the sheet is connected to. 
    #[ serde( rename = "dataSourceId" ) ]
    pub data_source_id : Option< String >,

    /// The columns displayed on the sheet, corresponding to the values in [RowData](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/sheets#RowData). 
    pub columns : Option< Vec< DataSourceColumn > >,

    /// The data execution status.
    #[ serde( rename = "dataExecutionStatus" ) ]
    pub data_executin_status : Option< DataExecutinStatus >
  }

  /// Properties of a sheet. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct SheetProperties
  {
    /// The ID of the sheet. Must be non-negative. This field cannot be changed once set. 
    #[ serde( rename = "sheetId" ) ]
    pub sheet_id : Option< u64 >,

    /// The name of the sheet. 
    pub title : Option< String >,

    /// The index of the sheet within the spreadsheet. When adding or updating sheet properties, if this field is excluded then
    /// the sheet is added or moved to the end of the sheet list. When updating sheet indices or inserting sheets, movement 
    /// is considered in "before the move" indexes. For example, if there were three sheets (S1, S2, S3) in order to move S1
    /// ahead of S2 the index would have to be set to 2. A sheet index update request is ignored if the requested index is
    /// identical to the sheets current index or if the requested new index is equal to the current sheet index + 1. 
    pub index : Option< u64 >,

    #[ serde( rename = "sheetType" ) ]
    /// The type of sheet. Defaults to GRID. This field cannot be changed once set.
    pub sheet_type : Option< SheetType >,

    /// Additional properties of the sheet if this sheet is a grid. (If the sheet is an object sheet, containing a chart or image, then this field will be absent.) When writing it is an error to set any grid properties on non-grid sheets. 
    #[ serde( rename = "gridProperties" ) ]
    pub grid_properties : Option< GridProperties >,

    /// True if the sheet is hidden in the UI, false if it's visible. 
    pub hidden : Option< bool >,

    /// The color of the tab in the UI. Deprecated: Use tabColorStyle. 
    #[ serde( rename = "tabColor" ) ]
    pub tab_color : Option< Color >,

    /// The color of the tab in the UI. If tabColor is also set, this field takes precedence. 
    #[ serde( rename = "tabColorStyle" ) ]
    pub tab_color_style : Option< ColorStyle >,

    /// True if the sheet is an RTL sheet instead of an LTR sheet. 
    #[ serde( rename = "rightToLeft" ) ]
    pub right_to_left : Option< bool >,

    /// Output only. If present, the field contains DATA_SOURCE sheet specific properties. 
    #[ serde( rename = "dataSourceSheetProperties" ) ]
    pub data_source_sheet_properties : Option< DataSourceSheetProperties >
  }


  #[ derive( Debug, Serialize ) ]
  pub struct GetValuesRequest
  {
    #[ serde( rename = "majorDimension" ) ]
    major_dimension : Option< Dimension >,

    #[ serde( rename = "valueRenderOption" ) ]
    value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "dateTimeRenderOption" ) ]
    date_time_render_option : Option< DateTimeRenderOption >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct BatchGetValuesRequest
  {
    ranges : Vec< String >,

    #[ serde( rename = "majorDimension" ) ]
    major_dimension : Option< Dimension >,

    #[ serde( rename = "valueRenderOption" ) ]
    value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "dateTimeRenderOption" ) ]
    date_time_render_option : Option< DateTimeRenderOption >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct UpdateValuesRequest
  {
    #[ serde( rename = "valueInputOption" )]
    value_input_option : ValueInputOption,

    #[ serde( rename = "includeValuesInResponse" ) ]
    include_values_in_response : Option< bool >,

    #[ serde( rename = "responseValueRenderOption" ) ]
    response_value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    response_date_time_render_option : Option< DateTimeRenderOption >
  }

  /// The request body.
  #[ derive( Debug, Serialize, Clone ) ]
  pub struct BatchUpdateValuesRequest 
  {
    /// The new values to apply to the spreadsheet.
    pub data : Vec< ValueRange >,

    #[ serde( rename = "valueInputOption" ) ]
    /// How the input data should be interpreted.
    pub value_input_option : ValueInputOption,

    /// Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The updatedData field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).
    #[ serde( rename = "includeValuesInResponse" ) ]
    pub include_values_in_response : Option< bool >,

    /// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
    #[ serde( rename = "responseValueRenderOption" ) ]
    pub response_value_render_option : Option< ValueRenderOption >,

    /// Determines how dates, times, and durations in the response should be rendered. This is ignored if responseValueRenderOption is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    pub response_date_time_render_option : Option< DateTimeRenderOption >,
  }

  #[ derive( Debug, Serialize ) ]
  pub struct ValuesAppendRequest
  {
    #[ serde( rename = "valueInputOption" ) ]
    pub value_input_option : ValueInputOption,
    
    #[ serde( rename = "insertDataOption" ) ]
    pub insert_data_option : Option< InsertDataOption >,

    #[ serde( rename = "includeValuesInResponse" ) ]
    pub include_values_in_response : bool,

    #[ serde( rename = "responseValueRenderOption" ) ]
    pub response_value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    pub response_date_time_render_option : Option< DateTimeRenderOption >
  }

  /// The request body.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct BatchClearValuesRequest
  {
    /// The ranges to clear, in A1 notation or R1C1 notation.
    pub ranges : Vec< String >
  }

  /// Response from [`values.batchGet`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchGet).
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct BatchGetValuesResponse 
  {
    /// The ID of the spreadsheet.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// A list of ValueRange objects with data for each requested range.
    #[ serde( rename = "valueRanges" ) ]
    pub value_ranges : Option< Vec< ValueRange > >,
  }

  /// Response from [`values.update`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/update).
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
  pub struct UpdateValuesResponse 
  {
    /// The ID of the spreadsheet that was updated.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The range (A1 notation) that was updated.
    #[ serde( rename = "updatedRange" ) ]
    pub updated_range : Option< String >,

    /// How many rows were updated.
    #[ serde( rename = "updatedRows" ) ]
    pub updated_rows : Option< u32 >,

    /// How many columns were updated.
    #[ serde( rename = "updatedColumns" ) ]
    pub updated_columns : Option< u32 >,

    /// How many cells were updated.
    #[ serde( rename = "updatedCells" ) ]
    pub updated_cells : Option< u32 >,

    /// If `includeValuesInResponse` was `true`, this field contains the updated data.
    #[ serde( rename = "updatedData" ) ]
    pub updated_data : Option< ValueRange >,
  }

  /// Response from [`values.batchUpdate`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchUpdate).
  #[ derive( Debug, Default, Serialize, Deserialize, Clone ) ]
  pub struct BatchUpdateValuesResponse 
  {
    /// The ID of the spreadsheet that was updated.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// Total number of rows updated.
    #[ serde( rename = "totalUpdatedRows" ) ]
    pub total_updated_rows : Option< u32 >,

    /// Total number of columns updated.
    #[ serde( rename = "totalUpdatedColumns" ) ]
    pub total_updated_columns : Option< u32 >,

    /// Total number of cells updated.
    #[ serde( rename = "totalUpdatedCells" ) ]
    pub total_updated_cells : Option< u32 >,

    /// Total number of sheets with updates.
    #[ serde( rename = "totalUpdatedSheets" ) ]
    pub total_updated_sheets : Option< u32 >,

    /// The response for each range updated (if `includeValuesInResponse` was `true`).
    pub responses : Option< Vec< ValueRange > >,
  }

  /// Response from [`values.append`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append).
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
  pub struct ValuesAppendResponse 
  {
    /// The ID of the spreadsheet to which data was appended.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The range (A1 notation) that covered the appended data before the append.
    #[ serde( rename = "tableRange" ) ]
    pub table_range : Option< String >,

    /// If `includeValuesInResponse` was `true`, this field contains metadata about the update.
    pub updates : Option< UpdateValuesResponse >,
  }

  /// Response from [values.clearBatch](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchClear)
  #[ derive( Debug, Default, Serialize, Deserialize ) ]
  pub struct BatchClearValuesResponse
  {
    /// The spreadsheet the updates were applied to.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits.
    #[ serde( rename = "clearedRanges" ) ]
    pub cleared_ranges : Option< Vec< String > >
  }

  /// Response from [`values.clear`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/clear)
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct ValuesClearResponse
  {
    /// The spreadsheet the updates were applied to.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The range (in A1 notation) that was cleared. (If the request was for an unbounded range or a ranger larger than the bounds of the sheet, this will be the actual range that was cleared, bounded to the sheet's limits.)
    #[ serde( rename = "clearedRange" ) ]
    pub cleared_range : Option< String >
  }

  /// Determines how existing data is changed when new data is input.
  #[ derive( Debug, Clone, Copy, Serialize, Deserialize ) ]
  pub enum InsertDataOption
  {
    /// The new data overwrites existing data in the areas it is written. (Note: adding data to the end of the sheet will still insert new rows or columns so the data can be written.)
    #[ serde( rename = "OVERWRITE" ) ]
    Overwrite,

    /// Rows are inserted for the new data.
    #[ serde( rename = "INSERT_ROWS" ) ]
    InsertRows
  }

  /// Determines how dates should be rendered in the output.
  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum DateTimeRenderOption
  {
    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    #[ serde( rename = "SERIAL_NUMBER" ) ]
    SerialNumber,

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    #[ serde( rename = "FORMATTED_STRING" ) ]
    FormattedString
  }

  /// Determines how values should be rendered in the output.
  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum ValueRenderOption
  {
    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "$1.23".
    #[ serde( rename = "FORMATTED_VALUE" ) ]
    FormattedValue,

    /// Values will be calculated, but not formatted in the reply. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return the number 1.23.
    #[ serde( rename = "UNFORMATTED_VALUE" ) ]
    UnformattedValue,

    /// Values will not be calculated. The reply will include the formulas. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "=A1".
    ///
    /// Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see About date & time values.
    #[ serde( rename = "FORMULA" ) ]
    Formula
  }

  /// Determines how input data should be interpreted.
  #[ derive( Debug, Clone, Copy, Default, Serialize ) ]
  pub enum ValueInputOption
  {
    /// The values the user has entered will not be parsed and will be stored as-is.
    #[ default ]
    #[ serde( rename = "RAW" ) ]
    Raw,

    /// The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    #[ serde( rename = "USER_ENTERED" ) ]
    UserEntered
  }

  /// Indicates which dimension an operation should apply to.
  #[ derive( Debug, Clone, Copy, Serialize, Deserialize ) ]
  pub enum Dimension 
  {
    /// Operates on the rows of a sheet.
    #[ serde( rename = "ROWS" ) ]
    Row,

    /// Operates on the columns of a sheet.
    #[ serde( rename = "COLUMNS" ) ]
    Column,
  }

  /// Data within a range of the spreadsheet.
  #[ derive( Debug, Clone, Default, serde::Serialize, serde::Deserialize ) ]
  pub struct ValueRange
  {
    /// The range the values cover, in A1 notation. For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended.
    pub range : Option< String >,

    /// The major dimension of the values.
    /// For output, if the spreadsheet data is: A1=1,B1=2,A2=3,B2=4, then requesting range=A1:B2,majorDimension=ROWS will return [[1,2],[3,4]], whereas requesting range=A1:B2,majorDimension=COLUMNS will return [[1,3],[2,4]].
    ///
    /// For input, with range=A1:B2,majorDimension=ROWS then [[1,2],[3,4]] will set A1=1,B1=2,A2=3,B2=4. With range=A1:B2,majorDimension=COLUMNS then [[1,2],[3,4]] will set A1=1,B1=3,A2=2,B2=4.
    ///
    /// When writing, if this field is not set, it defaults to ROWS.
    #[ serde( rename = "majorDimension" ) ]
    pub major_dimension : Option< Dimension >,

    /// The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell.
    ///
    /// For output, empty trailing rows and columns will not be included.
    /// 
    /// For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string.
    pub values : Option< Vec< Vec< serde_json::Value > > >
  }

}


crate::mod_interface!
{
  own use
  {
    Auth,
    Client,
    SheetProperties,
    Dimension,
    ValueRange,
    InsertDataOption,
    ValueInputOption,
    ValueRenderOption,
    ValuesAppendRequest,
    ValuesAppendResponse,    
    UpdateValuesResponse,
    BatchUpdateValuesRequest,
    BatchUpdateValuesResponse,
    ValuesClearResponse,
    BatchClearValuesRequest,
    BatchClearValuesResponse
  };
}