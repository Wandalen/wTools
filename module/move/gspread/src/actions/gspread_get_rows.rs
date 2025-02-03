//!
//! Action for command "rows"
//!
//! It returns all rows but not header
//!


mod private
{
  use crate::*;
  use client::SheetsType;
  use actions::gspread::
  {
    Error,
    Result
  };
  use ser::JsonValue;

  pub async fn action
  (
    hub : &SheetsType,
    spreadsheet_id : &str,
    table_name : &str
  ) -> Result< Vec< Vec < JsonValue > > >
  {
    match hub
    .spreadsheets()
    .values_get( spreadsheet_id, format!( "{}!A2:Z", table_name ).as_str() )
    .doit()
    .await
    {
      Ok( ( _, response ) ) =>
      {
        match response.values
        {
          Some( values ) => Ok( values ),
          None => Ok( Vec::new() )
        }
      },
      Err( error ) => Err( Error::ApiError( error ) )
    }
  }
}

crate::mod_interface!
{
  own use action;
}
