//!
//! Action for command "cell set"
//!
//! It updates a selected cell
//!


mod private
{
  use google_sheets4::api::ValueRange;
  use crate::*;
  use actions::gspread::
  { 
    Result,
    Error 
  };
  use client::SheetsType;
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str,
    cell_id : &str,
    value : &str
  ) -> Result< i32 >
  {

    let value = JsonValue::String( value.to_string() );
    let value_range = ValueRange
    {
      values : Some( vec![ vec![ value ] ] ),
      ..ValueRange::default()
    };

    match hub
    .spreadsheets()
    .values_update( value_range, spreadsheet_id, format!( "{}!{}", table_name, cell_id ).as_str() )
    .value_input_option( "USER_ENTERED" )
    .doit()
    .await
    {
      Ok( ( _, response) ) =>
      {
        match response.updated_cells
        {
          Some( number ) => Ok( number ),
          None => Err( Error::CellError( "Some problem with cell updating".to_string() ) )
        }
      }
      Err( error) => Err( Error::ApiError( error ) )
    }

  }
}

crate::mod_interface!
{
  own use action;
}