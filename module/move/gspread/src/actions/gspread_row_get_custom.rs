

mod private
{
  use crate::*;
  use actions::gspread::get_row_by_custom_row_key;
  use actions::utils::
  {
    parse_key_by, 
    parse_on_find
  };
  use gcore::
  {
    Secret,
    client::Client, 
    error::Result
  };
  
  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : &str,
    on_find : &str
  ) -> Result< Vec< Vec< serde_json::Value > > >
  {
    let key_by = parse_key_by( key_by )?;
    let on_find = parse_on_find( on_find )?;

    match get_row_by_custom_row_key
    (
      client, 
      spreadsheet_id, 
      sheet_name, 
      key_by, 
      on_find
    )
    .await
    {
      Ok( rows ) => Ok( rows ),
      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    action
  };
}