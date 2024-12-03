//!
//! Collection of subcommands fo command "cell"
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use actions;
  use actions::gspread::get_spreadsheet_id_from_url;
  use client::SheetsType;

  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    #[ command( name = "get" ) ]
    Get
    {
      #[ arg( long ) ]
      url : String,

      #[ arg( long ) ]
      tab : String,

      #[ arg( long ) ]
      cel : String,
    },

    #[ command( name = "set" ) ]
    Set
    {
      #[ arg( long ) ]
      url : String,

      #[ arg( long ) ]
      tab : String,

      #[ arg( long ) ]
      cel : String,

      #[ arg( long ) ]
      val : String
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
      Commands::Get { url, tab, cel } =>
      {
        let spreadsheet_id = get_spreadsheet_id_from_url( url.as_str() ).unwrap();

        let result = actions::gspread_cell_get::action
        (
          hub,
          spreadsheet_id,
          tab.as_str(),
          cel.as_str()
        ).await;

        match result
        {
          Ok( value ) => println!( "Value: {}", value ),
          Err( error ) => println!( "Error: {}", error ),
        }
      },

      Commands::Set { url, tab, cel, val } =>
      {
        let spreadsheet_id = get_spreadsheet_id_from_url( url.as_str() ).unwrap();

        let result = actions::gspread_cell_set::action
        (
          hub,
          spreadsheet_id,
          tab.as_str(),
          cel.as_str(),
          val.as_str()
        ).await;

        match result
        {
          Ok( value ) => println!( "Success: {:?}", value ),
          Err( error ) => println!( "Error: {}", error ),
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
    Commands,
  };
}