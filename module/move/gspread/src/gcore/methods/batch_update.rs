mod private 
{
  use crate::{gcore::{types::Spreadsheet, DeleteDimensionRequest, Error, Request, Response, Result}, *};
  use gcore::{Secret, Client};

  pub struct SpreadSheetBatchUpdate< 'a, 'b, S : Secret >
  {
    pub client : &'a Client< 'a, S >,
    pub requests : Vec< Request >,
    pub spreadsheet_id : &'b str,
    pub include_spreadsheet_in_response : bool,
    pub response_ranges : Vec< String >,
    pub response_include_grid_data : bool
  }

  impl< 'a, 'b, S : Secret > SpreadSheetBatchUpdate< 'a, 'b, S >
  {
    pub fn delete_dimension
    (
      mut self,
      req : DeleteDimensionRequest
    ) -> SpreadSheetBatchUpdate< 'a, 'b, S >
    {
      self.requests.push( Request::DeleteDimension( req ) );
      self
    }

    pub async fn doit( &self ) -> Result< Response >
    {
      let endpoint = format!
      (
        "{}/{}:batchUpdate",
        self.client.endpoint,
        self.spreadsheet_id
      );

      let body = BatchUpdateRequest
      {
        requests : self.requests.clone(),
        include_spreadsheet_in_response : self.include_spreadsheet_in_response,
        response_ranges : self.response_ranges.clone(),
        response_include_grid_data : self.response_include_grid_data
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
      .json( &body )
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

      let response_parsed = response.json::< Response >()
      .await
      .map_err( | err | Error::ParseError( err.to_string() ) )?;

      Ok( response_parsed )
    }
  }

  #[ derive( Debug, ser::Serialize, ser::Deserialize ) ]
  pub struct BatchUpdateRequest
  {
    /// A list of updates to apply to the spreadsheet. Requests will be applied in the order they are specified. If any request is not valid, no requests will be applied.
    requests : Vec< Request >,
    /// Determines if the update response should include the spreadsheet resource.
    #[ serde( rename = "includeSpreadsheetInResponse" ) ]
    include_spreadsheet_in_response : bool,
    /// Limits the ranges included in the response spreadsheet. Meaningful only if includeSpreadsheetInResponse is 'true'.
    #[ serde( rename = "resoponseRanges" ) ]
    response_ranges : Vec< String >,
    /// True if grid data should be returned. Meaningful only if includeSpreadsheetInResponse is 'true'. This parameter is ignored if a field mask was set in the request.
    #[ serde( rename = "responseIncludeGridData" ) ]
    response_include_grid_data : bool 
  }

  #[ derive( Debug, ser::Serialize, ser::Deserialize ) ]
  pub struct BatchUpdateResponse
  {
    #[ serde( rename = "spreadsheetId" ) ]
    spreadsheet_id : Option< String >,
    replies : Option< Vec< Response > >,
    #[ serde( rename = "updatedSpreadsheet" ) ]
    updated_spreadsheet : Option< Spreadsheet >
  }
}

crate::mod_interface!
{
  own use
  {
    SpreadSheetBatchUpdate
  };
}