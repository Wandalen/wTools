

mod private
{
  use clap::Parser;
  
  use crate::*;
  use gcore::Secret;
  use gcore::client::Client;
  use actions::utils::get_spreadsheet_id_from_url;

  #[ derive( Debug, Parser ) ]
  pub struct Args
  {
    #[ arg( long, help = "Full URL of Google Sheet.\n\
    It has to be inside of '' to avoid parse errors.\n\
    Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
    pub url : String,

    #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
    pub tab : String,

    #[ arg( long, help = "A string with key pair view, like [\"A\", \"new_val\"], where A is a column index." ) ]
      key_by : String,

    #[ arg( long, help = "Action to take if one or more rows are found.
    Available: 
      - all - Clear all matched rows.
      - first - Clear first matched.
      - last - Clear last matched." ) ]
    on_find : String
  }

  pub async fn command< S : Secret >
  (
    client : &Client< '_, S >,
    command : Args
  )
  {
    match command
    {
      Args{ url, tab, key_by, on_find } => 
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

        match actions::gspread_clear_custom::action
        (
          client,
          spreadsheet_id,
          &tab,
          &key_by,
          &on_find
        )
        .await
        {
          Ok( ranges ) => println!( "Updated ranges: {:?}", ranges ),
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
    Args,
    command
  };
}