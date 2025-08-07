//!
//! Set command -> set specified values in specified columns in specified row
//! 

mod private
{
  use std::collections::HashMap;

  use crate::*;
  use ser::Deserialize;
  use actions::gspread::update_row;
  use gcore::Secret; 
  use gcore::client::Client;
  use gcore::error::
  { 
    Error, 
    Result 
  };

  /// # ParsedJson
  ///
  /// A structure to store the row's primary key and new values for cell updates.
  ///
  /// ## Fields:
  /// - `row_key`:  
  ///   The primary key of the row.
  /// - `row_key_val`:  
  ///   A map of column names to new values.
  #[ derive( Deserialize, Debug ) ]
  struct ParsedJson
  {
    row_key : serde_json::Value,
    row_key_val : HashMap< String, serde_json::Value >,
  }
  
  /// # `parse_json`
  ///
  /// Parses the `--json` flag to extract the row key and values to update.
  ///
  /// ## Parameters:
  /// - `json_str`:  
  ///   The JSON string passed via the `--json` flag.
  /// - `select_row_by_key`:  
  ///   The key to use for identifying the row (e.g., `"id"`).
  ///
  /// ## Returns:
  /// - `Result< ParsedJson >`
  fn parse_json
  (
    json_str : &str,
    select_row_by_key : &str,
  ) -> Result< ParsedJson > 
  {
    let mut parsed_json : HashMap< String, serde_json::Value > = serde_json::from_str( json_str )
    .map_err( | error | Error::InvalidJSON( format!( "Failed to parse JSON: {}", error ) ) )?;

    let row_key = if let Some( row_key ) = parsed_json.remove( select_row_by_key ) 
    {
      row_key
    } 
    else 
    {
      return Err( Error::InvalidJSON( format!( "Key '{}' not found in JSON", select_row_by_key ) ) );
    };

    for ( col_name, _ ) in &parsed_json 
    {
      if !col_name.chars().all( | c | c.is_alphabetic() && c.is_uppercase() ) 
      {
        return Err
        ( 
          Error::InvalidJSON
          ( 
            format!( "Invalid column name: {}. Allowed only uppercase alphabetic letters (A-Z)", col_name )
          )
        );
      }
    };

    Ok
    (
      ParsedJson
      {
        row_key : row_key,
        row_key_val : parsed_json,
      }
    )
  }

  /// # `check_select_row_by_key`
  ///
  /// Validates if the provided row key is allowed.
  ///
  /// ## Parameters:
  /// - `key`:  
  ///   The row's primary key.
  ///
  /// ## Returns:
  /// - `Result< () >`
  fn check_select_row_by_key
  (
    key : &str
  ) -> Result< () > 
  {
    let keys = vec![ "id" ];
    if keys.contains( &key )
    {
      Ok( () )
    } 
    else 
    {
      Err
      ( 
        Error::ParseError( format!( "Invalid select_row_by_key: '{}'. Allowed keys: {:?}", key, keys ) ) 
      )
    }
  }

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    select_row_by_key : &str,
    json_str : &str,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< u32 >
  {
    check_select_row_by_key( select_row_by_key )?;

    match parse_json( json_str, select_row_by_key )
    {
      Ok( parsed_json ) =>
      {
        match update_row( client, spreadsheet_id, table_name, parsed_json.row_key, parsed_json.row_key_val ).await
        {
          Ok( response ) => 
          {
            match response.total_updated_cells
            {
              Some( val ) => Ok( val ),
              None => Ok( 0 ),
            }
          },
          Err( error ) => Err( error )
        }
      }
      Err( error ) => Err( error ),
    }
  }
  
}

crate::mod_interface!
{
  own use action;
}