//!
//! Action for command "cell get"
//!
//! It returns a selected cell
//!

mod private
{
  

  use crate::*;
  use actions::gspread::
  {
    Error,
    Result
  };
  use client::SheetsType;
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str,
    cell_id : &str,
  ) -> Result< JsonValue >
  {
    match hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!{}", table_name, cell_id ).as_str() )
    .doit()
    .await
    {
      Ok( (_, response ) ) => 
      match response.values
      {
        Some( values ) => Ok( values.get( 0 ).unwrap().get( 0 ).unwrap().clone() ),
        None => Ok( JsonValue::Null.clone() )
      }
      Err( error ) => Err( Error::ApiError( error ) )
    }

  }
}

crate::mod_interface!
{
  own use action;
}