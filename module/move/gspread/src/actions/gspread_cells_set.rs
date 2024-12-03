//!
//! Set command -> set specified values in specified columns in specified row
//! 

mod private
{
  use crate::*;
  use google_sheets4::api::
  {
    BatchUpdateValuesRequest, 
    ValueRange
  };
  use ser::
  { 
    Deserialize, 
    JsonValue 
  };
  use std::collections::HashMap;

  /// Structure for --json value
  #[ derive( Deserialize, Debug ) ]
  struct ParsedJson
  {
    #[ serde( flatten ) ]
    columns : HashMap< String, String >
  }
  
  /// Parse --json value
  fn parse_json
  (
    json_str : &str
  ) -> Result< ParsedJson, String > 
  {
    serde_json::from_str::< ParsedJson >( json_str ).map_err
    (
      | err | format!( "Failed to parse JSON: {}", err )
    )
  }

  /// Check availables keys.
  /// Available keys: "id" -> row's id
  fn check_select_row_by_key
  (
    key : &str
  ) -> Result< (), String > 
  {
    let keys = vec![ "id" ];
    if keys.contains( &key )
    {
      Ok( () )
    } 
    else 
    {
      Err( format!( "Invalid select_row_by_key: '{}'. Allowed keys: {:?}", key, keys ) )
    }
  }

  fn is_all_uppercase_letters
  (
    s : &str
  ) -> Result< (), String >
  {
    if s.chars().all( | c | c.is_ascii_uppercase() ) 
    {
      Ok( () )
    } 
    else 
    {
      Err( format!( "The string '{}' contains invalid characters. Only uppercase letters (A-Z) are allowed.", s ) )
    }
  }

  pub async fn action
  (
    hub : &SheetsType,
    select_row_by_key : &str,
    json_str : &str,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< String, String >
  {
    check_select_row_by_key( select_row_by_key )?;

    let mut pairs = parse_json( json_str )?;

    let row_id = pairs
    .columns
    .remove( select_row_by_key )
    .ok_or_else( || format!( "Key '{}' not found in JSON", select_row_by_key ) )?;

    let mut value_ranges= Vec::new();

    for ( key, value ) in pairs.columns.into_iter()
    {
      is_all_uppercase_letters( key.as_str() )?;
      value_ranges.push
      ( 
        ValueRange
        { 
          range: Some( format!( "{}!{}{}", table_name, key, row_id ) ), 
          values: Some( vec![ vec![ JsonValue::String( value.to_string() ) ] ] ),
          ..Default::default() 
        } 
      );
    };

    let req = BatchUpdateValuesRequest
    {
      value_input_option: Some( "USER_ENTERED".to_string() ),
      data: Some( value_ranges ),
      include_values_in_response: Some( true ),
      ..Default::default()
    };

    let result = hub
    .spreadsheets()
    .values_batch_update( req, spreadsheet_id )
    .doit()
    .await;

    match result
    {
      Ok( _ ) => Ok( format!( "Cells were sucsessfully updated!" ) ),
      Err( error ) => Err( format!( "{}", error ) )
    }
  }
}

crate::mod_interface!
{
  own use action;
}