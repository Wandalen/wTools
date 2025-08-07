//!
//! clear command
//! 

mod private
{
  use crate::*;
  use gcore::Secret;
  use gcore::client::Client;
  use commands::gspread::CommonArgs;
  use actions::utils::get_spreadsheet_id_from_url;

  pub async fn command< S : Secret >
  (
    client : &Client< '_, S >,
    args : CommonArgs
  )
  {
    match args
    {
      CommonArgs { url, tab } => 
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( url.as_str() ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_clear::action
        (
          client,
          spreadsheet_id,
          &tab
        )
        .await
        {
          Ok( range ) => println!( "Range {range} was successfully cleared" ),
          Err( error ) => eprintln!( "Error:\n{error}" )
        }
      }
    }
  }
}

crate::mod_interface!
{
  own use
  {
    command,
  };
}