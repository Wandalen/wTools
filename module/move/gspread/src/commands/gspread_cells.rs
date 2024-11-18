//!
//! Collection of subcommands fo command "cell"
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use actions;
  use actions::gspread::get_sheetspread_id_from_url;
  use client::SheetsType;

  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    #[ command( name = "get" ) ]
    Get
    {
      #[ arg( long ) ]
      url: String,

      #[ arg( long ) ]
      tab: String,

      #[ arg( long ) ]
      cel: String,
    },

    #[ command( name = "set" ) ]
    Set
    {
      #[ arg( long ) ]
      url: String,

      #[ arg( long ) ]
      tab: String,

      #[ arg( long ) ]
      cel: String,

      #[ arg( long ) ]
      val: String
    }
  }

  pub async fn command
  (
    hub: &SheetsType,
    commands: Commands
  )
  {
    match commands
    {
      Commands::Get { url, tab, cel } =>
      {
        let sheetspread_id = get_sheetspread_id_from_url( url.as_str() ).unwrap();

        let result = actions::gspread_cell_get::action
        (
          hub,
          sheetspread_id,
          tab.as_str(),
          cel.as_str()
        ).await;

        match result
        {
          Ok( ValueRange ) => println!( "Value: {}", ValueRange.get( 0 ).unwrap().get( 0 ).unwrap() ),
          Err( error ) => println!( "Error: {}", error ),
        }
      },

      Commands::Set { url, tab, cel, val } =>
      {
        let sheetspread_id = get_sheetspread_id_from_url( url.as_str() ).unwrap();

        let result = actions::gspread_cell_set::action
        (
          hub,
          sheetspread_id,
          tab.as_str(),
          cel.as_str(),
          val.as_str()
        ).await;

        match result
        {
          Ok( Value ) => println!( "Success: {:?}", Value ),
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