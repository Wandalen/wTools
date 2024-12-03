//!
//! Cells commands.
//! set command -> set specified values in specified columns in specified row.
//! 

mod private
{
  use clap::Subcommand;

  use crate::*;
  use actions::gspread::get_spreadsheet_id_from_url;

  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    #[ command( name = "set" ) ]
    Set
    {
      #[ arg( long ) ]
      select_row_by_key : String,

      #[ arg( long ) ]
      json : String,

      #[ arg( long ) ]
      url : String,

      #[ arg( long ) ]
      tab : String
    }

  }

  pub async fn command
  (
    hub : &SheetsType,
    commands : Commands
  )
  {
    match commands
    {
      Commands::Set { select_row_by_key, json, url, tab } =>
      {
        let spreadsheet_id = get_spreadsheet_id_from_url( url.as_str() ).unwrap();
        
        let result = actions::gspread_cells_set::action
        (
          &hub,
          select_row_by_key.as_str(),
          json.as_str(),
          spreadsheet_id,
          tab.as_str()
        ).await;

        match result
        {
          Ok( msg ) => println!( "{}", msg ),
          Err( error ) => println!( "{}", error )
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
    Commands
  };
}