

mod private
{
  use std::collections::HashMap;
  use crate::*;
  use actions::gspread::append_row;
  use gcore::Secret;
  use gcore::client::Client;
  use gcore::error::
  { 
    Error, 
    Result 
  };

  /// # parse_json
  /// 
  /// Parse privded string to HashMap< String, serde_json::Value >
  /// 
  /// ## Errors:
  /// 
  /// Can occur if provided string is not valid.
  fn parse_json
  ( 
    json_str : &str 
  ) -> Result< HashMap< String, serde_json::Value > >
  {
    let parsed_json : HashMap< String, serde_json::Value > = serde_json::from_str( json_str )
    .map_err( | error | Error::InvalidJSON( format!( "Failed to parse JSON: {}", error ) ) )?;

    Ok( parsed_json )
  }

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    json_str : &str
  ) -> Result< u32 >
  {
    match parse_json( json_str )
    {
      Ok( row_key_val ) => 
      {
        match append_row( client, spreadsheet_id, sheet_name, &row_key_val ).await
        {
          Ok( response ) => Ok( response.total_updated_cells.unwrap() ),
          Err( error ) => Err( error )
        }
      }
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    action,
  };
}