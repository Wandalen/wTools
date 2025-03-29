//!
//! Action for the command "cell set".
//!
//! Updates the value of a selected cell in the specified Google Sheet.
//!


mod private
{
  use crate::*;
  use serde_json::json;
  use actions::gspread::set_cell;
  use gcore::Secret;
  use gcore::client::Client;
  use gcore::error::
  {
    Error,
    Result
  }; 

  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
    value : &str
  ) -> Result< u32 >
  {
    match set_cell( client, spreadsheet_id, sheet_name, cell_id, json!( value ) ).await
    {
      Ok( response ) =>
      {
        match response.updated_cells 
        {
          Some( amount ) => Ok( amount ),
          None => Err( Error::CellError( "Some problem with cell updating".to_string() ) )
        }
      },
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use action;
}