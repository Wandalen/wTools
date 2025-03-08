//!
//! Copy command action 
//!

mod private
{
  use crate::*;
  use actions::gspread::copy_to;
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
    sheet_id : &str,
    dest : &str
  ) -> Result< String >
  {
    match copy_to
    (
      client, 
      spreadsheet_id, 
      sheet_id, 
      dest
    )
    .await
    {
      Ok( response ) =>
      {
        let title = response.title.unwrap_or_default();
        Ok( title )
      },
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