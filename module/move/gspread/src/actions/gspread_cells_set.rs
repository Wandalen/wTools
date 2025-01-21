//!
//! Set command -> set specified values in specified columns in specified row
//! 

mod private
{
  use crate::*;
  use actions::gspread::
  {
    Error,
    Result,
    update_row
  };
  use ser:: Deserialize;
  use std::collections::HashMap;

  /// Structure to keep rows key and new values for cells updating.
  #[ derive( Deserialize, Debug ) ]
  struct ParsedJson< 'a >
  {
    row_key : &'a str,
    row_key_val : HashMap< String, String >
  }
  
  /// Function to parse `--json` flag.
  /// 
  /// It retirive `--select-row-by-key` flag from json and set it to `row_key` field.
  /// Other pairs it set to `row_key_val`
  /// 
  /// **Returns**
  ///  - `ParsedJson` object
  fn parse_json< 'a >
  (
    json_str : &'a str,
    select_row_by_key : &str,
  ) -> Result< ParsedJson< 'a > > 
  {
    let mut parsed_json: HashMap< String, String > = serde_json::from_str( json_str )
    .map_err( | error | Error::InvalidJSON( format!( "Failed to parse JSON: {}", error ) ) )?;

    let row_key = if let Some( row_key ) = parsed_json.remove( select_row_by_key ) 
    {
      Box::leak( row_key.into_boxed_str() )
    } 
    else 
    {
      return Err
      (
        Error::InvalidJSON
        (
          format!( "Key '{}' not found in JSON", select_row_by_key)
        )
      );
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
        row_key_val : parsed_json
      }
    )
  }

  /// Check availables keys.
  /// Available keys: "id" -> row's id
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

  pub async fn action
  (
    select_row_by_key : &str,
    json_str : &str,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< i32 >
  {
    check_select_row_by_key( select_row_by_key )?;

    match parse_json( json_str, select_row_by_key )
    {
      Ok( parsed_json ) => 
      match update_row( spreadsheet_id, table_name, parsed_json.row_key, parsed_json.row_key_val ).await
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
      Err( error ) => Err( error ),
    }
  }
  
}

crate::mod_interface!
{
  own use action;
}