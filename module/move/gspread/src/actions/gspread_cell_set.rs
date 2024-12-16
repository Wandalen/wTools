//!
//! Action for command "cell set"
//!
//! It updates a selected cell
//!


mod private
{
  use google_sheets4::api::ValueRange;
  use crate::*;
  use actions::gspread::Result;
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

    let result = hub
    .spreadsheets()
    .values_update( value_range, spreadsheet_id, format!( "{}!{}", table_name, cell_id ).as_str() )
    .value_input_option( "USER_ENTERED" )
    .doit()
    .await?
    .1
    .updated_cells
    .unwrap();

    Ok( result )
  }
}

crate::mod_interface!
{
  own use action;
}